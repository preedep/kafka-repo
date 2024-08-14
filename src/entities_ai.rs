use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResult {
    #[serde(rename = "@odata.context")]
    pub odata_context: Option<String>,
    #[serde(rename = "@search.answers")]
    pub search_answers: Option<Vec<AISearchAnswerValue>>,
    #[serde(rename = "@search.nextPageParameters")]
    pub search_next_parameters: Option<AISearchNextPageParameters>,
    pub value: Option<Vec<AISearchResultValue>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResultCaption {
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "highlights")]
    pub highlights: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchNextPageParameters {
    #[serde(rename = "search")]
    pub search: Option<String>,
    #[serde(rename = "queryType")]
    pub query_type: Option<String>,
    #[serde(rename = "semanticConfiguration")]
    pub semantic_configuration: Option<String>,
    #[serde(rename = "captions")]
    pub captions: Option<String>,
    #[serde(rename = "answers")]
    pub answers: Option<String>,
    #[serde(rename = "queryLanguage")]
    pub query_language: Option<String>,
    #[serde(rename = "skip")]
    pub skip: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchAnswerValue {
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(rename = "highlights")]
    pub highlights: Option<String>,
    #[serde(rename = "score")]
    pub score: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResultValue {
    #[serde(rename = "@search.score")]
    pub search_score: Option<f64>,
    #[serde(rename = "@search.rerankerScore")]
    pub search_reranker_score: Option<f64>,
    #[serde(rename = "@search.captions")]
    pub search_captions: Option<Vec<AISearchResultCaption>>,
    pub id: Option<String>,
    #[serde(rename = "App_owner")]
    pub app_owner: Option<String>,
    #[serde(rename = "Topic_name")]
    pub topic_name: Option<String>,
    #[serde(rename = "MQ_topic")]
    pub mq_topic: Option<String>,
    #[serde(rename = "Consumer_group_id")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "Consumer_app")]
    pub consumer_app: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompleteRequestMessage {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "content")]
    pub content: String,
}
impl OpenAICompleteRequestMessage {
    pub fn new(role: &str, content: &str) -> Self {
        OpenAICompleteRequestMessage {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompleteRequest {
    #[serde(rename = "messages")]
    pub messages: Vec<OpenAICompleteRequestMessage>,
    #[serde(rename = "max_tokens")]
    pub max_tokens: i64,
    #[serde(rename = "temperature")]
    pub temperature: f64,
    #[serde(rename = "top_p")]
    pub top_p: f64,
    #[serde(rename = "stop")]
    pub stop: Option<Vec<String>>,
}

//
//  response of Open AI API
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionResult {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
