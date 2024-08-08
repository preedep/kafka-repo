use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResult {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@search.answers")]
    pub search_answers: Option<Vec<AISearchAnswerValue>>,
    #[serde(rename = "@search.nextPageParameters")]
    pub search_next_parameters: Option<AISearchNextPageParameters>,
    pub value: Option<Vec<AISearchResultValue>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResultCaption {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "highlights")]
    pub highlights: String,
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
    #[serde(rename = "Consumer_group_id")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "Consumer_app")]
    pub consumer_app: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionResult {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    #[serde(rename = "prompt_filter_results")]
    pub prompt_filter_results: Option<Vec<OpenAIPromptFilterResult>>,
    pub choices: Option<Vec<OpenAIChoice>>,
    pub usage: Option<OpenAIUsage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIPromptFilterResult {
    #[serde(rename = "prompt_index")]
    pub prompt_index: i64,
    #[serde(rename = "content_filter_results")]
    pub content_filter_results: OpenAIContentFilterResults,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIContentFilterResults {
    pub hate: Option<OpenAIHate>,
    pub jailbreak: Option<OpenAIJailbreak>,
    #[serde(rename = "self_harm")]
    pub self_harm: Option<OpenAISelfHarm>,
    pub sexual: Option<OpenAISexual>,
    pub violence: Option<OpenAIViolence>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIHate {
    pub filtered: bool,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIJailbreak {
    pub filtered: bool,
    pub detected: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAISelfHarm {
    pub filtered: bool,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAISexual {
    pub filtered: bool,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIViolence {
    pub filtered: bool,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIChoice {
    pub text: String,
    pub index: i64,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    #[serde(rename = "logprobs")]
    pub log_probs: serde_json::Value,
    #[serde(rename = "content_filter_results")]
    pub content_filter_results: OpenAIContentFilterResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIProtectedMaterialCode {
    pub filtered: bool,
    pub detected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIProtectedMaterialText {
    pub filtered: bool,
    pub detected: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAIUsage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}
