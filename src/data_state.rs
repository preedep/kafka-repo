use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub kafka_inventory: Option<DataFrame>,
    pub kafka_consumer: Option<DataFrame>,
    pub user_authentication: Option<DataFrame>,
    pub jwt_secret: String,
    // Azure AI Search
    pub azure_ai_search_url: Option<String>,
    pub azure_ai_search_key: Option<String>,
    // Open AI
    pub azure_open_ai_url: Option<String>,
    pub azure_open_ai_key: Option<String>,
}
