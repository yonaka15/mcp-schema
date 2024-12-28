use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The sender or recipient of messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Optional annotations for objects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Contains optional annotation data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Text content in a prompt or message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

/// Image content, stored in base64
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub kind: String,
    pub data: String,
    pub mime_type: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

/// A paginated request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _meta: Option<super::base::RequestMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<super::base::Cursor>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A paginated result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<super::base::Cursor>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}