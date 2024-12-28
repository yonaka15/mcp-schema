use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Syslog-like logging severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

/// Parameters for enabling/adjusting logging
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLevelParams {
    pub level: LoggingLevel,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A notification with a log message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingMessageParams {
    pub level: LoggingLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    pub data: Value,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}