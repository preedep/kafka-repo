use langchain_rust::chain::{Chain, LLMChainBuilder};
use langchain_rust::llm::{AzureConfig, OpenAI};
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::schemas::Message;
use langchain_rust::{fmt_message, fmt_placeholder, fmt_template, message_formatter, prompt_args, template_fstring};
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
    let open_ai = create_openai(
                                app_state.azure_open_ai_url.as_ref().unwrap(),
                                app_state.azure_open_ai_key.as_ref().unwrap()
    );

    let res = process_with_llm(
                               prompt_message,
                               knowledge,
                               &open_ai).await.map_err(|e|
        APIError::new(&format!("Failed to process with LLM: {}", e)))?;
    Ok(res)
}

fn create_openai(open_ai_url: &str, open_ai_key: &str) -> OpenAI<AzureConfig> {
    debug!("Creating OpenAI client with URL: {} and key: {}", open_ai_url, open_ai_key);
    let azure_config = AzureConfig::default()
        .with_api_base(open_ai_url)
        .with_api_key(open_ai_key)
        .with_api_version("2023-03-15-preview")
        .with_deployment_id("gpt-4");

    OpenAI::new(azure_config)
}

// Function to handle the LLM chain execution and processing (Refactor LLM logic)
async fn process_with_llm(
    input: &str,
    knowledge: &str,
    open_ai: &OpenAI<AzureConfig>,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "You are a world-class technical documentation writer. Use the following knowledge to answer the user's query."
        )),
        fmt_placeholder!("history"),
        fmt_message!(Message::new_system_message(format!("Knowledge:\n{}", knowledge))),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!("{input}", "input")))
    ];

    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai.clone())
        .build()?;


    let res = chain
        .invoke(prompt_args! {
            "input" => input,
            "knowledge" => knowledge,
            "history" => Vec::<Message>::new()
        })
        .await;

    if let Ok(result) = res {
        Ok(result)
    } else {
        Err(Box::new(res.err().unwrap()))
    }
}