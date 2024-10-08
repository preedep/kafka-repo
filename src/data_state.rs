use crate::entities_ai::AISearchIndex;
use async_openai::config::AzureConfig;
use polars::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub kafka_inventory: Option<DataFrame>,
    pub kafka_consumer: Option<DataFrame>,
    pub user_authentication: Option<DataFrame>,
    pub jwt_secret: String,
    // Azure AI Search
    pub azure_ai_search_url: Option<String>,
    pub azure_ai_search_key: Option<String>,
    pub azure_ai_search_indexes: Option<Vec<AISearchIndex>>,
    pub azure_ai_search_use_semantics: bool,
    // Open AI
    //pub azure_open_ai_url: Option<String>,
    //pub azure_open_ai_key: Option<String>,
    // static knowledge
    pub knowledge: Option<String>,
    // Azure Open AI Configuration
    pub open_ai_config: AzureConfig,
}
