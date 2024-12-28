use serde::{Deserialize, Serialize};
use super::base::{RequestId, EmptyResult, PingParams, MCPNotificationParams};
use super::initialization::InitializeResult;
use super::sampling::CreateMessageParams;
use super::cancellation::CancelledNotificationParams;
use super::progress::ProgressNotificationParams;
use super::logging::LoggingMessageParams;
use super::resources::ResourceUpdatedParams;
use super::roots::ListRootsParams;
use super::client_completion::CompleteResult;
use super::prompts::{GetPromptResult, ListPromptsResult};
use super::resources::{ListResourcesResult, ListResourceTemplatesResult, ReadResourceResult};
use super::tools::{CallToolResult, ListToolsResult};

/// A union of possible server requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ServerRequest {
    #[serde(rename = "ping")]
    Ping {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        #[serde(default)]
        params: PingParams,
    },
    #[serde(rename = "sampling/createMessage")]
    CreateMessage {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: CreateMessageParams,
    },
    #[serde(rename = "roots/list")]
    ListRoots {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        #[serde(default)]
        params: ListRootsParams,
    },
}

/// A union of possible server notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ServerNotification {
    #[serde(rename = "notifications/cancelled")]
    Cancelled {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: CancelledNotificationParams,
    },
    #[serde(rename = "notifications/progress")]
    Progress {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: ProgressNotificationParams,
    },
    #[serde(rename = "notifications/message")]
    LoggingMessage {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: LoggingMessageParams,
    },
    #[serde(rename = "notifications/resources/updated")]
    ResourceUpdated {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        params: ResourceUpdatedParams,
    },
    #[serde(rename = "notifications/resources/list_changed")]
    ResourceListChanged {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        #[serde(default)]
        params: MCPNotificationParams,
    },
    #[serde(rename = "notifications/tools/list_changed")]
    ToolListChanged {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        #[serde(default)]
        params: MCPNotificationParams,
    },
    #[serde(rename = "notifications/prompts/list_changed")]
    PromptListChanged {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        #[serde(default)]
        params: MCPNotificationParams,
    },
}

/// A union of all possible server results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerResult {
    Empty(EmptyResult),
    Initialize(InitializeResult),
    Complete(CompleteResult),
    GetPrompt(GetPromptResult),
    ListPrompts(ListPromptsResult),
    ListResources(ListResourcesResult),
    ListResourceTemplates(ListResourceTemplatesResult),
    ReadResource(ReadResourceResult),
    CallTool(CallToolResult),
    ListTools(ListToolsResult),
}