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
#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, PartialEq)]
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
    #[serde(rename = "full_application_name")]
    pub full_application_name: Option<String>,
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    #[serde(rename = "business_application_name")]
    pub business_application_name: Option<String>,
    #[serde(rename = "application_level")]
    pub application_level: Option<String>,
    #[serde(rename = "company_or_subsidiary_name")]
    pub company_or_subsidiary_name: Option<String>,
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(rename = "app_category")]
    pub app_category: Option<String>,
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
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub index: Option<u32>,
    pub message: Option<Message>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchSemantics {
    #[serde(rename = "select_fields")]
    pub select_fields: String,
    #[serde(rename = "semantic_name")]
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchIndex {
    #[serde(rename = "index_name")]
    pub index_name: String,
    #[serde(rename = "semantics")]
    pub semantics: Option<Vec<AISearchSemantics>>,
}
