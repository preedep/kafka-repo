
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::EncodingKey;
use log::debug;


use crate::data_service::{post_login};
use crate::data_state::AppState;
use crate::entities::{
    APIError, APIResponse, Claims, JwtResponse, SearchKafkaRequest, SearchKafkaResponse, UserLogin,
};
use crate::entities_ai::{ AISearchResultValue, OpenAICompletionResult};
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
/*
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
*/
/// Build the prompt for the AI search based on the query and context.
fn build_prompt(query: &str, context: &str) -> String {
    format!(
        "Based on the following context, please answer the questions provided:\n\nContext:\n{}\n\nQuestions:\n{}\n",
        context, query
    )
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
) -> APIWebResponse<OpenAICompletionResult> {
    debug!("Searching Open AI with query: {:#?}", search_request);
    let mut final_prompt = String::new();

    if let Some(query_message) = &search_request.ai_search_query {
        //split questions by open ai
        let mut first_prompt =
            "Please split the following text into questions and non-questions:\n\n".to_string();
        first_prompt.push_str(query_message);
        first_prompt.push_str("\n\n List the questions under a \"Questions\" section and the non-questions under a \"Non-Questions\" section. Format each question and non-questions as a numbered list and header must use **Questions:** and **Non-Questions:** :");

        let result = crate::azure_ai_apis::open_ai_completion(&first_prompt, &app_state).await?;
        debug!(
            "Result from Open AI Completion for split question and non question: {:#?}",
            result
        );
        if result.choices.is_none() {
            return Err(APIError::new("Action is empty"));
        }

        //loop for all choices
        for choice in result.choices.unwrap() {
            let text = choice.message.unwrap().content.unwrap_or("".to_string());
            let (mut questions, non_questions) = split_questions_and_non_questions(&text);

            questions.append(&mut non_questions.clone());
            //debug!("Questions: {:#?}", questions);
            for question in questions {
                // AI search must specific with query message first
                // search each question with AI search
                final_prompt.push_str("Question: ");
                final_prompt.push_str(&question);
                final_prompt.push_str("\n");
                let indexes = app_state
                    .azure_ai_search_indexes
                    .clone()
                    .unwrap_or_default();

                for (_index, ai_search_index) in indexes.iter().enumerate() {
                    let index_name = ai_search_index.index_name.clone();
                    for semantic in ai_search_index.clone().semantics.unwrap() {
                        /*
                        Filter again
                        */
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
                        let new_question = question.clone();
                        array_of_filters.push(new_question);
                        let question = array_of_filters
                            .clone()
                            .into_iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join(" and ");

                        debug!("Question for ai search combine columns : {:#?}", question);

                        let result = crate::azure_ai_apis::ai_search(
                            &index_name,
                            &semantic.name,
                            &semantic.select_fields,
                            &question,
                            &app_state,
                        )
                        .await?;
                        debug!("Result from AI Search: {:#?}", result);
                        if let Some(content) = result.search_answers {
                            let combine_data = content
                                .iter()
                                .map(|c| {
                                    format!(
                                        "Answer: {}\n",
                                        c.clone().text.unwrap_or("".to_string()),
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n\n");
                            final_prompt.push_str(&combine_data);
                        }
                        if let Some(value) = result.value {
                            //sort by score
                            let n_top = sort_ai_search_result_by_score_get_n_top(value,3);
                            //generate combined prompt
                            let combine_data = n_top
                                .iter()
                                .map(|c| {
                                    if let Some(cap) = &c.search_captions {
                                        cap.iter()
                                            .map(|c| {
                                                if !c.clone().text.unwrap_or_default().is_empty()
                                                    && !c
                                                        .clone()
                                                        .highlights
                                                        .unwrap_or_default()
                                                        .is_empty()
                                                {
                                                    /*
                                                    format!(
                                                        "Summary: {}\nRelevant Highlights Section: {}\n",
                                                        c.clone().text.unwrap_or_default(),
                                                        c.clone().highlights.unwrap_or_default()
                                                    )*/
                                                    format!(
                                                        "Summary: {}\n",
                                                        c.clone().text.unwrap_or_default()
                                                    )
                                                } else {
                                                    "".to_string()
                                                }
                                            })
                                            .skip_while(|p| p.is_empty())
                                            .collect::<Vec<String>>()
                                            .join("\n")
                                    } else {
                                        "".to_string()
                                    }
                                })
                                .skip_while(|p| p.is_empty())
                                .collect::<Vec<String>>()
                                .join("\n");


                            let mut map_app = HashMap::new();
                            for value_item in n_top {
                                if let Some(consumer_app) = value_item.clone().consumer_app {
                                   if !map_app.contains_key(&consumer_app) {
                                        map_app.insert(consumer_app.clone(), consumer_app.clone());
                                    }
                                }
                                if let Some(app_owner) = value_item.clone().app_owner {
                                   if !map_app.contains_key(&app_owner) {
                                        map_app.insert(app_owner.clone(), app_owner.clone());
                                    }
                                }
                            }

                            if map_app.len() > 0 {
                                let apps = map_app.keys().map(|c| c.to_string()).collect::<Vec<String>>().join(" or ");

                                let question_app_info = format!("(full_application_name: ({}) or business_application_name: ({}) )", apps, apps);

                                debug!("Question for ai search app info : {:#?}", question_app_info);

                                let result_app_info = crate::azure_ai_apis::ai_search(
                                    &"azureblob-app-info-invenindex-json".to_string(),
                                    &"app-info-semantics-dev003".to_string(),
                                    &"full_application_name,application_id,business_application_name,application_level,service,app_category".to_string(),
                                    &question_app_info,
                                    &app_state,
                                )
                                    .await?;
                                debug!("question_app_info: {:#?}", question_app_info);
                                debug!(
                                    "Result from AI Search for app information: {:#?}",
                                    result_app_info
                                );

                                if let Some(values) = result_app_info.value {
                                    //sort by score
                                    //let mut values = values.clone();
                                    let n_top = sort_ai_search_result_by_score_get_n_top(values,3);

                                    let combine_data = n_top
                                        .iter()
                                        .map(|c| {
                                            format!(
                                                "Application Information of Application Name or App Name: {}\nApplication ID: {}\nBusiness Application Name: {}\nApplication Level: {}\nService: {}\nApp Category: {}\n",
                                                c.clone().full_application_name.unwrap_or("".to_string()),
                                                c.clone().application_id.unwrap_or("".to_string()),
                                                c.clone().business_application_name.unwrap_or("".to_string()),
                                                c.clone().application_level.unwrap_or("".to_string()),
                                                c.clone().service.unwrap_or("".to_string()),
                                                c.clone().app_category.unwrap_or("".to_string())
                                            )
                                        })
                                        .collect::<Vec<String>>()
                                        .join("\n");
                                    final_prompt.push_str(&combine_data);
                                }
                            }
                            final_prompt.push_str(&combine_data);
                        }
                    }
                }
            }
            /*
            if !non_questions.is_empty() {
                //debug!("Non-Questions: {:#?}", non_questions);
                final_prompt.push_str("Non-Questions:\n");
                for non_question in non_questions {
                    final_prompt.push_str(&non_question);
                    final_prompt.push_str("\n");
                }
            }*/
            final_prompt.push_str("\n\n");
        }
        // load all data from csv
        /*
        debug!("Load all csv data");
        if let (Some(ds_inventory), Some(ds_consumer)) =
            (&app_state.kafka_inventory, &app_state.kafka_consumer)
        {
            let result = search(ds_inventory, ds_consumer, &search_request)?;
            let csv_data = result
                .iter()
                .map(|d| {
                    format!(
                        "App Owner or Producer: {}\nE-Kafka Topic Name: {}\nConsumer Group Id: {}\nConsumer or Consume App: {}\n",
                        d.app_owner, d.topic_name, d.consumer_group_id, d.consumer_app
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            final_prompt.push_str(&csv_data);
        }
        */

        let final_prompt = build_prompt(query_message, &final_prompt);
        debug!("Final Prompt: \n{}", final_prompt);
        let result = crate::azure_ai_apis::open_ai_completion(&final_prompt, &app_state).await?;
        debug!("Result from Open AI Completion: {:#?}", result);
        if result.choices.is_none() {
            return Err(APIError::new("Open AI Completion result is empty"));
        }
        Ok(APIResponse { data: result })
    } else {
        Err(APIError::new(
            "Failed to search AI , Please provide query message",
        ))
    }
}

fn sort_ai_search_result_by_score_get_n_top(mut value: Vec<AISearchResultValue>, n:usize) -> Vec<AISearchResultValue> {
    value.sort_by(|a, b| {
        b.search_score.unwrap_or(0.0)
            .partial_cmp(&a.search_score.unwrap_or(0.0)).unwrap()
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
