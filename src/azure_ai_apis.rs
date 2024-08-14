use log::debug;
use serde_json::Value;

use crate::data_state::AppState;
use crate::entities::APIError;
use crate::entities_ai::{
    AISearchResult, OpenAICompleteRequest, OpenAICompleteRequestMessage, OpenAICompletionResult,
};
/**
 * Performs an AI search using the Azure AI Search service.
 *
 * \param index_name The name of the index to search.
 * \param semantics_configuration The semantic configuration to use for the search.
 * \param query_message The search query message.
 * \param app_state The application state containing configuration and credentials.
 * \return A result containing the AI search result or an API error.
 */
pub async fn ai_search(
    index_name: &String,
    semantics_configuration: &String,
    query_message: &String,
    app_state: &AppState,
) -> Result<AISearchResult, APIError> {
    let api_endpoint = app_state.clone().azure_ai_search_url.unwrap();
    let ai_search_key = app_state.clone().azure_ai_search_key.unwrap();
    let client = reqwest::Client::new();
    let url = format!(
        "{}/indexes('{}')/docs/search?api-version=2024-05-01-preview",
        api_endpoint, index_name
    );
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", ai_search_key)
        .json(&serde_json::json!(
            {
                "search": query_message,
                "queryType": "semantic",
                "semanticConfiguration": semantics_configuration,
                "captions": "extractive",
                "answers": "extractive|count-5",
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
/**
 * Performs a completion request using the OpenAI API.
 *
 * \param query_message The query message to send to OpenAI.
 * \param app_state The application state containing configuration and credentials.
 * \return A result containing the OpenAI completion result or an API error.
 */
pub async fn open_ai_completion(
    query_message: &String,
    app_state: &AppState,
) -> Result<OpenAICompletionResult, APIError> {
    let api_endpoint = app_state.clone().azure_open_ai_url.unwrap();
    let open_ai_key = app_state.clone().azure_open_ai_key.unwrap();
    let client = reqwest::Client::new();
    let url = format!(
        "{}/openai/deployments/gpt-4/chat/completions?api-version=2024-02-15-preview",
        api_endpoint
    );

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
