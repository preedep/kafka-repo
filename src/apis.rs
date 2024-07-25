use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use log::debug;

use crate::data_state::AppState;
use crate::entities::{APIError, APIResponse, SearchKafkaResponse};
use crate::export::export_mm_file;
use crate::{data_service, entities};

type APIWebResponse<T> = Result<APIResponse<T>, APIError>;

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
    search_request: web::Json<entities::SearchKafkaRequest>,
) -> APIWebResponse<Vec<SearchKafkaResponse>> {
    debug!("Searching kafka with request: {:?}", search_request);
    if let (Some(ds_inventory), Some(ds_consumer)) = (&data.kafka_inventory, &data.kafka_consumer) {
        let result = data_service::search(ds_inventory, ds_consumer, &search_request)?;
        return Ok(APIResponse { data: result });
    }
    Err(APIError::new("Failed to search kafka"))
}

pub async fn post_topic_kafka_relation_render(
    data: web::Data<Arc<AppState>>,
    search_request: web::Json<entities::SearchKafkaRequest>,
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

        let html = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Kafka Topic Relation</title>
                <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
                <script>mermaid.initialize({{startOnLoad:true}});</script>
            </head>
            <body>
                <div class="mermaid">
                    {}
                </div>
            </body>
            </html>
            "#,
            mermaid_text
        );


        let r = HttpResponse::Ok()
            .content_type("text/html")
            .body(html);
        return Ok(r);
    }
    Err(APIError::new("Failed to search kafka"))
}
