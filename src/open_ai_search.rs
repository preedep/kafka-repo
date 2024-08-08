use log::debug;
use serde_json::Value;
use crate::data_state::AppState;
use crate::entities::{AISearchResult, APIError, OpenAICompletionResult};

pub async fn ai_search(query_message: &String, app_state: &AppState) -> Result<AISearchResult, APIError> {
    let ai_search_key = app_state.clone().azure_ai_search_key.unwrap();
    let client = reqwest::Client::new();
    let url = "https://nick-ai-dev002.search.windows.net/indexes('ekafka-inventory-idx-001')/docs/search?api-version=2024-05-01-preview";
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
        .map_err(|e| APIError::new(&format!("Failed to send request to OpenAI: {}", e)))?;


    let r = response.json::<AISearchResult>().await.map_err(
        |e| APIError::new(&format!("Failed to parse response from OpenAI: {}", e)))?;

    debug!("Response from AI Search : {:?}", r);

    Ok(r)
}

pub async fn open_ai_completion(query_message: &String, app_state: &AppState) -> Result<OpenAICompletionResult, APIError> {
    let open_ai_key = app_state.clone().azure_open_ai_key.unwrap();
    let client = reqwest::Client::new();
    let url = "https://nickazureaidev002.openai.azure.com/openai/deployments/gpt-35-turbo/completions?api-version=2023-09-15-preview";
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("api-key", open_ai_key)
        .json(&serde_json::json!(
            {
                "prompt": query_message,
                "max_tokens": 150,
                "temperature": 1,
                "top_p": 0.5,
                "frequency_penalty": 0.0,
                "presence_penalty": 0.0,
                "stop": ["\n"]
            }
        )).send().await.map_err(|e| APIError::new(&format!("Failed to send request to OpenAI: {}", e)))?;



    Ok(OpenAICompletionResult{

    })
}