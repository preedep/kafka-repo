use std::fmt::{Debug, Display, Formatter};

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse, Responder, ResponseError};
use derive_more::Display;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtResponse {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: usize,
}

#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    #[serde(rename = "sub")]
    sub: String, // Subject (typically the user ID)
    #[serde(rename = "exp")]
    exp: usize, // Expiration time (as a Unix timestamp)
    #[serde(rename = "iat")]
    iat: usize, // Issued at time (as a Unix timestamp)
    #[serde(rename = "iss")]
    iss: String, // Issuer
    #[serde(rename = "aud")]
    aud: String, // Audience
}

impl Claims {
    pub fn new(sub: String, exp: usize, iss: String, aud: String) -> Self {
        let iat = chrono::Utc::now().timestamp() as usize;
        Claims {
            sub,
            exp,
            iat,
            iss,
            aud,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchKafkaRequest {
    #[serde(rename = "app_owner")]
    pub app_owner: Option<String>,
    #[serde(rename = "topic_name")]
    pub topic_name: Option<String>,
    #[serde(rename = "consumer_app")]
    pub consumer_app: Option<String>,
    #[serde(rename = "search_all_text")]
    pub search_all_text: Option<String>,
    #[serde(rename = "ai_search_query")]
    pub ai_search_query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchKafkaResponse {
    #[serde(rename = "app_owner")]
    pub app_owner: String,
    #[serde(rename = "topic_name")]
    pub topic_name: String,
    #[serde(rename = "consumer_group_id")]
    pub consumer_group_id: String,
    #[serde(rename = "consumer_app")]
    pub consumer_app: String,
    #[serde(rename = "description")]
    pub description: String,
}

impl Into<FlowChartItem> for SearchKafkaResponse {
    fn into(self) -> FlowChartItem {
        FlowChartItem {
            project_name_owner_alias: self.app_owner.clone().replace(" ", "_").to_lowercase(),
            project_name_owner: self.app_owner.clone(),
            kafka_topic: self.topic_name.clone(),
            consumer_group: self.consumer_group_id.clone(),
            project_name_consume: self.consumer_app.clone(),
            project_name_consume_alias: self.consumer_app.clone().replace(" ", "_").to_lowercase(),
        }
    }
}

impl From<FlowChartItem> for SearchKafkaResponse {
    fn from(item: FlowChartItem) -> Self {
        SearchKafkaResponse {
            app_owner: item.project_name_owner.clone(),
            topic_name: item.kafka_topic.clone(),
            consumer_group_id: item.consumer_group.clone(),
            consumer_app: item.project_name_consume.clone(),
            description: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowChartItem {
    project_name_owner_alias: String,
    project_name_owner: String,
    pub kafka_topic: String,
    pub consumer_group: String,
    project_name_consume: String,
    project_name_consume_alias: String,
}
impl FlowChartItem {
    pub(crate) fn to_print_string(&self) -> String {
        format!(
            "{}[{}] ---> {} ---> {} ---> {}[{}]",
            self.project_name_owner_alias,
            self.project_name_owner,
            self.kafka_topic,
            self.consumer_group,
            self.project_name_consume_alias,
            self.project_name_consume
        )
    }
}
impl Display for FlowChartItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_print_string())
    }
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
