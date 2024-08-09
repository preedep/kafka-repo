use std::sync::Arc;

use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::EncodingKey;
use log::debug;
use log::kv::ToKey;

use crate::data_service::{post_login, search};
use crate::data_state::AppState;
use crate::entities::{
    APIError, APIResponse, Claims, JwtResponse, SearchKafkaRequest, SearchKafkaResponse, UserLogin,
};
use crate::entities_ai::{AISearchResult, OpenAICompletionResult};
use crate::export::export_mm_file;
use crate::{data_service, entities};

type APIWebResponse<T> = Result<APIResponse<T>, APIError>;

pub async fn login(
    data: web::Data<Arc<AppState>>,
    user_login: Json<UserLogin>,
) -> APIWebResponse<JwtResponse> {
    debug!("Logging in");
    debug!("User: {}", user_login.username);

    if let Some(ds) = &data.user_authentication {
        debug!("User authentication dataset: {:?}", ds);

        let result = post_login(ds, &user_login.username, &user_login.password);

        if let Ok(b) = result {
            return if b {
                let expiration = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::seconds(3600))
                    .expect("valid timestamp")
                    .timestamp();

                let claims = Claims::new(
                    user_login.username.clone(),
                    expiration as usize,
                    "kafka-repo-iss".to_string(),
                    "kafka-repo-service-aud".to_string(),
                );

                let jwt_token = jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &claims,
                    &EncodingKey::from_secret(data.jwt_secret.as_ref()),
                )
                .map_err(|e| {
                    debug!("Failed to encode jwt token: {}", e);
                    APIError::new("Failed to encode jwt token")
                })?;

                let response = JwtResponse {
                    token: jwt_token,
                    token_type: "Bearer".to_string(),
                    expires_in: 3600,
                };

                Ok(APIResponse { data: response })
            } else {
                Err(APIError::new("Invalid username or password"))
            };
        } else {
            let err = result.err().unwrap();
            return Err(err);
        }
    }
    Err(APIError::new("Failed to login"))
}

pub async fn get_apps(data: web::Data<Arc<AppState>>) -> APIWebResponse<Vec<String>> {
    debug!("Getting app list");
    if let Some(ds) = &data.kafka_inventory {
        let apps = data_service::get_app_list(ds)?;
        return Ok(APIResponse { data: apps });
    }
    Err(APIError::new("Failed to get app list"))
}

pub async fn get_topics(
    data: web::Data<Arc<AppState>>,
    app_name: web::Path<String>,
) -> APIWebResponse<Vec<String>> {
    debug!("Getting topic list for app: {}", app_name);
    if let Some(ds) = &data.kafka_inventory {
        let topics = data_service::get_topic_list(ds, &app_name)?;
        return Ok(APIResponse { data: topics });
    }
    Err(APIError::new("Failed to get app list"))
}

pub async fn get_consumers(data: web::Data<Arc<AppState>>) -> APIWebResponse<Vec<String>> {
    debug!("Getting consumer list");
    if let Some(ds) = &data.kafka_consumer {
        let consumers = data_service::get_consumer_list(ds)?;
        return Ok(APIResponse { data: consumers });
    }
    Err(APIError::new("Failed to get consumer list"))
}

pub async fn post_search_kafka(
    data: web::Data<Arc<AppState>>,
    search_request: Json<entities::SearchKafkaRequest>,
) -> APIWebResponse<Vec<SearchKafkaResponse>> {
    debug!("Searching kafka with request: {:?}", search_request);
    if let (Some(ds_inventory), Some(ds_consumer)) = (&data.kafka_inventory, &data.kafka_consumer) {
        let result = data_service::search(ds_inventory, ds_consumer, &search_request)?;
        return Ok(APIResponse { data: result });
    }
    Err(APIError::new("Failed to search kafka"))
}

fn truncate_text(text: &str, max_length: usize) -> String {
    if text.chars().count() > max_length {
        format!("{}...", text.chars().take(max_length).collect::<String>())
    } else {
        text.to_string()
    }
}

fn chunk_records<T>(records: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    records
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn build_prompt(query: &str, context: &str) -> String {
    format!(
        "Answer the following query based on the provided context.\n\nQuery: {}\nContext: {}\n\nAnswer:",
        query, context
    )
}

pub async fn post_ai_search(
    app_state: web::Data<Arc<AppState>>,
    search_request: Json<SearchKafkaRequest>,
) -> APIWebResponse<OpenAICompletionResult> {
    debug!("Searching Open AI with query: {:?}", search_request);

    let mut final_prompt = String::new();

    if let Some(query_message) = &search_request.ai_search_query {
        // AI search must specific with query message first
        let result = crate::open_ai_search::ai_search(query_message, &app_state).await?;
        debug!("Result from AI Search: {:#?}", result);
        if let Some(content) = result.search_answers {
            let combine_data = content
                .iter()
                .map(|c| {
                    format!(
                        "Answer: {}\nHighlights: {}\n",
                        c.clone().text.unwrap_or("".to_string()),
                        c.clone().highlights.unwrap_or("".to_string())
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            final_prompt.push_str(&combine_data);
        }

        // load all data from csv
        debug!("Load all csv data");
        if let (Some(ds_inventory), Some(ds_consumer)) =
            (&app_state.kafka_inventory, &app_state.kafka_consumer)
        {
            let result = search(ds_inventory, ds_consumer, &search_request)?;
            let csv_data = result
                .iter()
                .map(|d| {
                    format!(
                        "Producer or App Owner: {}\nE-Kafka Topic Name: {}\nConsumer Group Id: {}\nConsumer or Consume App: {}\n",
                        d.app_owner, d.topic_name, d.consumer_group_id, d.consumer_app
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            final_prompt.push_str(&csv_data);
        }

        let final_prompt = build_prompt(query_message, &final_prompt);

        //debug!("Final Prompt: \n{}", final_prompt);
        let result = crate::open_ai_search::open_ai_completion(&final_prompt, &app_state).await?;
        debug!("Result from Open AI Completion: {:#?}", result);
        Ok(APIResponse { data: result })
    } else {
        Err(APIError::new(
            "Failed to search AI , Please provide query message",
        ))
    }
}

pub async fn post_topic_kafka_relation_render(
    data: web::Data<Arc<AppState>>,
    search_request: Json<entities::SearchKafkaRequest>,
) -> Result<impl Responder, APIError> {
    debug!("Searching kafka with request: {:?}", search_request);
    if let (Some(ds_inventory), Some(ds_consumer)) = (&data.kafka_inventory, &data.kafka_consumer) {
        let result = data_service::search(ds_inventory, ds_consumer, &search_request)?;
        // Export to mermaid file
        let path = "flowchart.mmd";
        let mermaid_text = export_mm_file(result.clone(), path).map_err(|e| {
            debug!("Failed to export to mermaid file: {}", e);
            APIError::new("Failed to export to mermaid file")
        })?;

        let r = HttpResponse::Ok()
            .content_type("text/plain")
            .body(mermaid_text);
        return Ok(r);
    }
    Err(APIError::new("Failed to search kafka"))
}
