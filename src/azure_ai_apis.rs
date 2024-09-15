use async_openai::config::AzureConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use log::debug;

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
    select_fields: &String,
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
    //let select_fields = app_state.azure_ai_search_select_fields.clone().unwrap_or("*".to_string());
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
                "queryLanguage": "en-US",
                "select": select_fields
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
    prompt_message: &String,
    knowledge: &String,
    app_state: &AppState,
) -> Result<String, APIError> {
    let az_config = app_state.open_ai_config.clone();

    let res = process_with_llm(prompt_message, knowledge, &az_config)
        .await
        .map_err(|e| APIError::new(&format!("Failed to process with LLM: {}", e)))?;

    debug!("OpenAI Completion Result: {:#?}", res);
    Ok(res)
}

// Function to handle the LLM chain execution and processing (Refactor LLM logic)
async fn process_with_llm(
    input: &str,
    knowledge: &str,
    az_config: &AzureConfig,
) -> Result<String, APIError> {
    debug!("Azure config : {:?}", az_config);
    let client = async_openai::Client::with_config(az_config.to_owned());

    let ai_assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
        .content( "You are a world-class technical documentation writer. Use the following knowledge to answer the user's query.")
        .build()
        .map_err(|e| APIError::new(&format!("Failed to build system message: {}", e)))?;

    let knowledge_message = ChatCompletionRequestSystemMessageArgs::default()
        .content(knowledge)
        .build()
        .map_err(|e| APIError::new(&format!("Failed to build knowledge message: {}", e)))?;

    let human_message = ChatCompletionRequestUserMessageArgs::default()
        .content(input)
        .build()
        .map_err(|e| APIError::new(&format!("Failed to build human message: {}", e)))?;

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .max_tokens(1000u32)
        .temperature(0.7)
        .top_p(1.0)
        .messages(vec![
            ai_assistant_message.into(),
            knowledge_message.into(),
            human_message.into(),
        ])
        .build()
        .map_err(|e| APIError::new(&format!("Failed to build completion request: {}", e)))?;

    debug!("Request: {:?}", request);

    let res = client
        .chat()
        .create(request)
        .await
        .map_err(|e| APIError::new(&format!("Failed to create chat completion: {}", e)))?;
    debug!("Response: {:?}", res);
    let mut text_result = String::new();
    if res.choices.is_empty() {
        text_result.push_str("No response from OpenAI");
        return Ok(text_result);
    }
    for choice in res.choices {
        if let Some(content) = choice.message.content {
            text_result.push_str(&content);
        }
    }
    Ok(text_result)
}
