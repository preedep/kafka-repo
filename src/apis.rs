use crate::data_state::AppState;
use crate::entities::{APIError, APIResponse, SearchKafkaResponse};
use crate::{data_service, entities};
use actix_web::{web, HttpResponse, Responder};
use log::debug;
use polars::prelude::IntoLazy;
use std::sync::Arc;

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
