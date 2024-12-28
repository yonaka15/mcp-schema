use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use super::base::ProgressToken;

/// Parameters for a progress notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressNotificationParams {
    pub progress_token: ProgressToken,
    pub progress: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}