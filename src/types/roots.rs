use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Parameters for roots/list
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsParams {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result listing root URIs from the client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub roots: Vec<Root>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents a root directory or file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}