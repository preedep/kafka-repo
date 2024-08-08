use crate::data_state::AppState;
use crate::entities::APIError;

pub async fn ai_search(query_message: &String,app_state: &AppState) -> Result<String,APIError> {
    let ai_search_key = app_state.clone().azure_ai_search_key.unwrap();
    let client = reqwest::Client::new();
    let url = "https://nick-ai-dev002.search.windows.net/indexes('ekafka-inventory-idx-001')/docs/search?api-version=2024-05-01-preview";
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("api-key", ai_search_key)
        .json(&serde_json::json!(
            {
                "search": query,
                "queryType": "semantic",
                "semanticConfiguration": "ekafka-semantic-dev001",
                "captions": "extractive",
                "answers": "extractive|count-3",
                "queryLanguage": "en-US"
            }
        ))
        .send()
        .await.map_err(|e| {
            APIError::new(&format!("Failed to send request to OpenAI: {}", e))
        })?;
    Ok("".to_string())
}