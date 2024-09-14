use std::collections::HashMap;
use std::sync::Arc;

use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::EncodingKey;
use log::{debug, error};

use crate::data_service::post_login;
use crate::data_state::AppState;
use crate::entities::{
    APIError, APIResponse, Claims, JwtResponse, SearchKafkaRequest, SearchKafkaResponse, UserLogin,
};
use crate::entities_ai::{AISearchResultValue, OpenAICompletionResult};
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

fn split_questions_and_non_questions(input: &str) -> (Vec<String>, Vec<String>) {
    // Split the input string into two parts: Questions and Non-Questions
    let parts: Vec<&str> = input.split("**Non-Questions:**").collect();

    let mut questions = Vec::new();
    let mut non_questions = Vec::new();

    if parts.len() == 2 {
        // Split the questions part by "**Questions:**" to remove the label
        if let Some(question_part) = parts[0].split("**Questions:**").nth(1) {
            questions = question_part
                .lines() // Split by lines instead of '\n' to handle all line endings
                .filter(|line| !line.trim().is_empty()) // Filter out empty lines
                .map(|line| line.trim_start_matches("1.").trim().to_string())
                .collect();
        }

        // Process the non-questions part
        non_questions = parts[1]
            .lines() // Split by lines instead of '\n' to handle all line endings
            .filter(|line| !line.trim().is_empty()) // Filter out empty lines
            .map(|line| line.trim_start_matches("1.").trim().to_string())
            .collect();
    }

    (questions, non_questions)
}
/// Perform AI search using Azure AI and Open AI Completion.
///
/// # Arguments
///
/// * `app_state` - The shared state of the application.
/// * `search_request` - The request object containing the search query for AI search.
///
/// # Returns
///
/// Returns `APIWebResponse` with content of type `OpenAICompletionResult` or an `APIError` if the search fails.
pub async fn post_ai_search(
    app_state: web::Data<Arc<AppState>>,
    search_request: Json<SearchKafkaRequest>,
) -> APIWebResponse<String> {
    debug!("Searching Open AI with query: {:#?}", search_request);
    //let mut final_prompt = String::new();

    if let Some(query_message) = &search_request.ai_search_query {
        let empty = "".to_string();

        let pre_knowledge = app_state.knowledge.as_ref().unwrap_or(&empty);

        let mut knowledge = String::new();
        knowledge.push_str(pre_knowledge);
        knowledge.push_str("\n");
        // find more knowledge from AI search
        let mut array_of_filters = Vec::new();
        if let Some(app_owner) = &search_request.app_owner {
            array_of_filters.push(format!("App_owner: {}", app_owner));
        }
        if let Some(topic_name) = &search_request.topic_name {
            array_of_filters.push(format!("Topic_name: {}", topic_name));
        }
        if let Some(consumer_app) = &search_request.consumer_app {
            array_of_filters.push(format!("Consumer_app: {}", consumer_app));
        }

        let mut question = array_of_filters
            .clone()
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" and ");

        if !question.is_empty() {
            question.push_str(" and  ");
            question.push_str("( ");
            question.push_str(&query_message);
            question.push_str(") ");
        }else{
            question.push_str("( ");
            question.push_str(&query_message);
            question.push_str(") ");
        }

        if let Some(azure_ai_search) = app_state.azure_ai_search_indexes.as_ref() {
            for index in azure_ai_search {
                let index = index.to_owned();
                let index_name = index.index_name;
                if let Some(semantics) = index.semantics.as_ref() {
                    for semantic in semantics {
                        let semantic = semantic.to_owned();
                        let semantic_name = semantic.name;
                        let fields = semantic.select_fields;

                        let result = crate::azure_ai_apis::ai_search(
                            &index_name,
                            &semantic_name,
                            &fields,
                            &question,
                            &app_state,
                        ).await.map_err(|e| {
                            error!("Failed to search AI: {}", e);
                            APIError::new("Failed to search AI")
                        })?;
                        //append knowledge from AI search
                        debug!("Result from AI Search: {:#?}", result);
                        if let Some(values) = result.value {

                            for value in values {
                                let empty = String::new();
                                let description = value.description.as_ref().unwrap_or(&empty);
                                knowledge.push_str(format!("Here is Description or Detail of e-kafka  : {}\n", description).as_str());
                                /*
                                let app_owner = value.app_owner.as_ref().unwrap_or(&empty);
                                let topic_name = value.topic_name.as_ref().unwrap_or(&empty);
                                let consumer_app = value.consumer_app.as_ref().unwrap_or(&empty);
                                let mq_topic = value.mq_topic.as_ref().unwrap_or(&empty);
                                knowledge.push_str("Here is e-Kafka information\n");
                                knowledge.push_str(format!("Here is Topic Name of e-kafka  : {}\n", topic_name).as_str());
                                knowledge.push_str(format!("Here App Owner of e-kafka topic : {}\n", app_owner).as_str());
                                knowledge.push_str(format!("Here Consumer App of e-kafka topic : {}\n", consumer_app).as_str());
                                knowledge.push_str(format!("Here MQ Topic which relate with e-kafka topic : {}\n", mq_topic).as_str());
                                knowledge.push_str("\n");

                                 */
                            }
                        }
                    }
                }
            }
        }

        debug!("Knowledge: {:#?}", knowledge);

        let result =
            crate::azure_ai_apis::open_ai_completion(&query_message, &knowledge, &app_state).await?;
        debug!("Result from Open AI Completion: {:#?}", result);

        Ok(APIResponse { data: result })
    } else {
        Err(APIError::new(
            "Failed to search AI , Please provide query message",
        ))
    }
}

fn sort_ai_search_result_by_score_get_n_top(
    mut value: Vec<AISearchResultValue>,
    n: usize,
) -> Vec<AISearchResultValue> {
    value.sort_by(|a, b| {
        b.search_score
            .unwrap_or(0.0)
            .partial_cmp(&a.search_score.unwrap_or(0.0))
            .unwrap()
    });
    // get top n after sorted by score
    value[0..std::cmp::min(n, value.len())].to_vec()
}
pub async fn post_topic_kafka_relation_render(
    data: web::Data<Arc<AppState>>,
    search_request: Json<SearchKafkaRequest>,
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
