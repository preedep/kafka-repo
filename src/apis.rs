use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use log::debug;
use polars::prelude::IntoLazy;
use crate::data_service;
use crate::data_state::AppState;
use crate::entities::{APIError, APIResponse};

type APIWebResponse<T> = Result<APIResponse<T>,APIError>;

pub async fn get_app_list(data: web::Data<Arc<AppState>>) -> APIWebResponse<Vec<String>> {
    debug!("Getting app list");
    if let Some(ds) = &data.kafka_inventory {
       let apps = data_service::get_app_list(ds)?;
        return Ok(
            APIResponse {
                data: apps
            }
        )
    }
   Err(
       APIError::new("Failed to get app list")
   )
}