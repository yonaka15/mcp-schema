use serde::{Deserialize, Serialize};
use super::base::{RequestId, MCPNotificationParams, PingParams};
use super::initialization::InitializeParams;
use super::tools::{CallToolParams, ListToolsResult};
use super::resources::{ReadResourceParams, SubscribeParams, UnsubscribeParams};
use super::prompts::{GetPromptParams};
use super::logging::SetLevelParams;
use super::common::PaginatedParams;

/// A union of all possible client requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ClientRequest {
    #[serde(rename = "ping")]
    Ping {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        #[serde(default)]
        params: PingParams,
    },
    #[serde(rename = "initialize")]
    Initialize {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: InitializeParams,
    },
    #[serde(rename = "completion/complete")]
    Complete {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: super::client_completion::CompleteParams,
    },
    #[serde(rename = "logging/setLevel")]
    SetLevel {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: SetLevelParams,
    },
    #[serde(rename = "prompts/get")]
    GetPrompt {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: GetPromptParams,
    },
    #[serde(rename = "prompts/list")]
    ListPrompts {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: PaginatedParams,
    },
    #[serde(rename = "resources/list")]
    ListResources {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: PaginatedParams,
    },
    #[serde(rename = "resources/templates/list")]
    ListResourceTemplates {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: PaginatedParams,
    },
    #[serde(rename = "resources/read")]
    ReadResource {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: ReadResourceParams,
    },
    #[serde(rename = "resources/subscribe")]
    Subscribe {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: SubscribeParams,
    },
    #[serde(rename = "resources/unsubscribe")]
    Unsubscribe {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: UnsubscribeParams,
    },
    #[serde(rename = "tools/call")]
    CallTool {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: CallToolParams,
    },
    #[serde(rename = "tools/list")]
    ListTools {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: PaginatedParams,
    },
}

/// A union of all possible client notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ClientNotification {
    #[serde(rename = "notifications/cancelled")]
    Cancelled {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: super::cancellation::CancelledNotificationParams,
    },
    #[serde(rename = "notifications/progress")]
    Progress {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: super::progress::ProgressNotificationParams,
    },
    #[serde(rename = "notifications/initialized")]
    Initialized {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        #[serde(default)]
        params: MCPNotificationParams,
    },
    #[serde(rename = "notifications/roots/list_changed")]
    RootsListChanged {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        #[serde(default)]
        params: MCPNotificationParams,
    },
}