use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse, Responder};
use log::error;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchKafkaRequest {
    #[serde(rename = "app_owner")]
    pub app_owner: Option<String>,
    #[serde(rename = "topic_name")]
    pub topic_name: Option<String>,
    #[serde(rename = "consumer_app")]
    pub consumer_app: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone,Default)]
pub struct SearchKafkaResponse {
    #[serde(rename = "app_owner")]
    pub app_owner : String,
    #[serde(rename = "topic_name")]
    pub topic_name : String,
    #[serde(rename = "consumer_group_id")]
    pub consumer_group_id : String,
    #[serde(rename = "consumer_app")]
    pub consumer_app : String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIResponse<T: Debug + Serialize + Clone> {
    #[serde(rename = "data")]
    pub(crate) data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIError {
    #[serde(rename = "error")]
    error: String,
}

impl APIError {
    pub fn new(error: &str) -> APIError {
        APIError {
            error: error.to_string(),
        }
    }
}
impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl error::ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

impl<T: Debug + Serialize + Clone> Responder for APIResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Serialize the struct to a JSON string
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize response: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        // Create an HTTP response with JSON content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
