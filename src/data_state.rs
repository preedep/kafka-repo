use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub kafka_inventory: Option<DataFrame>,
    pub kafka_consumer: Option<DataFrame>,
    pub user_authentication: Option<DataFrame>,
    pub jwt_secret: String,
    pub azure_ai_search_key: Option<String>,
    pub azure_open_ai_key: Option<String>
}
