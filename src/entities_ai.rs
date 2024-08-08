use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResult {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@search.answers")]
    pub search_answers: Vec<AISearchResultValue>,
    pub value: Vec<AISearchResultValue>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResultCaption {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "highlights")]
    pub highlights: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISearchResultValue {
    #[serde(rename = "@search.score")]
    pub search_score: f64,
    #[serde(rename = "@search.rerankerScore")]
    pub search_reranker_score: f64,
    #[serde(rename = "@search.captions")]
    pub search_captions: Vec<AISearchResultCaption>,
    pub id: String,
    #[serde(rename = "App_owner")]
    pub app_owner: String,
    #[serde(rename = "Topic_name")]
    pub topic_name: String,
    #[serde(rename = "Consumer_group_id")]
    pub consumer_group_id: String,
    #[serde(rename = "Consumer_app")]
    pub consumer_app: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionResult {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    #[serde(rename = "prompt_filter_results")]
    pub prompt_filter_results: Vec<OpenAIPromptFilterResult>,
    pub choices: Vec<OpenAIChoice>,
    pub usage: OpenAIUsage,
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
    pub hate: OpenAIHate,
    pub jailbreak: OpenAIJailbreak,
    #[serde(rename = "self_harm")]
    pub self_harm: OpenAISelfHarm,
    pub sexual: OpenAISexual,
    pub violence: OpenAIViolence,
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



