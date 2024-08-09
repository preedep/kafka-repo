use log::debug;
use serde_json::Value;

use crate::data_state::AppState;
use crate::entities::APIError;
use crate::entities_ai::{
    AISearchResult, OpenAICompleteRequest, OpenAICompleteRequestMessage, OpenAICompletionResult,
};

pub async fn ai_search(
    index_name:&String,
    query_message: &String,
    app_state: &AppState,
) -> Result<AISearchResult, APIError> {
    let ai_search_key = app_state.clone().azure_ai_search_key.unwrap();
    let client = reqwest::Client::new();
    let url = format!("https://nick-ai-dev002.search.windows.net/indexes('{}')/docs/search?api-version=2024-05-01-preview", index_name);
    //let url = "https://nick-ai-dev002.search.windows.net/indexes('ekafka-inventory-idx-001')/docs/search?api-version=2024-05-01-preview";
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", ai_search_key)
        .json(&serde_json::json!(
            {
                "search": query_message,
                "queryType": "semantic",
                "semanticConfiguration": "ekafka-semantic-dev001",
                "captions": "extractive",
                "answers": "extractive|count-3",
                "queryLanguage": "en-US"
            }
        ))
        .send()
        .await
        .map_err(|e| APIError::new(&format!("Failed to send request to AI Search: {}", e)))?;

    let r = response
        .json::<AISearchResult>()
        .await
        .map_err(|e| APIError::new(&format!("Failed to parse response from AI Search: {}", e)))?;

    Ok(r)
}

pub async fn open_ai_completion(
    query_message: &String,
    app_state: &AppState,
) -> Result<OpenAICompletionResult, APIError> {
    let open_ai_key = app_state.clone().azure_open_ai_key.unwrap();
    let client = reqwest::Client::new();
    let url = "https://nickazureaidev002.openai.azure.com/openai/deployments/gpt-4/chat/completions?api-version=2024-02-15-preview";

    //let query_message = query_message.split("\n").collect::<Vec<&str>>();
    let json_req = OpenAICompleteRequest {
        messages: vec![OpenAICompleteRequestMessage::new("user", query_message)],
        max_tokens: 800,
        temperature: 0.7,
        top_p: 0.95,
        stop: None,
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", open_ai_key)
        .json(&json_req)
        .send()
        .await
        .map_err(|e| APIError::new(&format!("Failed to send request to OpenAI: {}", e)))?;

    let r = response
        .json::<OpenAICompletionResult>()
        .await
        .map_err(|e| APIError::new(&format!("Failed to parse response from OpenAI: {}", e)))?;

    debug!("OpenAI Response: {:#?}", r);

    Ok(r)
}
