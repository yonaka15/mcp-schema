//! This module defines MCP + JSON-RPC data structures, using `serde` for
//! serialization/deserialization. It follows the JSON-RPC 2.0 specification
//! and the Model Context Protocol (MCP) while ensuring Rust naming conventions
//! are kept (`snake_case` internally, `camelCase` in JSON).
//!
//! # Overview
//!
//! - `JSONRPCRequest<T>` / `JSONRPCResponse<U>` are generic types for JSON-RPC messages.
//! - MCP extends JSON-RPC with additional fields, metadata, and custom structures
//!   (e.g., `InitializeParams`, `ClientCapabilities`, etc.).
//! - All fields use `#[serde(rename_all = "camelCase")]` so Rust code remains snake_case
//!   while JSON output remains camelCase.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The JSON-RPC version string (always "2.0").
pub const JSONRPC_VERSION: &str = "2.0";

/// The latest Model Context Protocol version.
pub const LATEST_PROTOCOL_VERSION: &str = "2024-11-05";

// Below are standard JSON-RPC error codes.
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

/// A request ID for JSON-RPC, which can be either a string or a number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

/// A progress token for associating progress notifications with a request.
/// This can be either a string or a number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
    String(String),
    Number(i64),
}

/// A cursor for pagination.
pub type Cursor = String;

/// A generic JSON-RPC request.
///
/// # Type Parameters
///
/// - `T`: The type of the `params` field, containing request-specific data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCRequest<T> {
    /// Must be "2.0" for JSON-RPC.
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,

    /// Method name.
    pub method: String,

    /// Request ID (string or number).
    pub id: RequestId,

    /// Generic request parameters.
    pub params: T,
}

/// A generic JSON-RPC notification. Notifications do not carry an `id`.
///
/// # Type Parameters
///
/// - `T`: The type of the `params` field, containing notification-specific data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCNotification<T> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub method: String,
    pub params: T,
}

/// A generic JSON-RPC response.
///
/// # Type Parameters
///
/// - `U`: The type of the `result` field, containing response-specific data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<U> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: RequestId,

    /// Result object when the request completes successfully.
    pub result: U,
}

/// A JSON-RPC error message, indicating that a request failed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCError {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: RequestId,
    pub error: RPCErrorDetail,
}

/// Provides details about a JSON-RPC error, including an optional `data` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCErrorDetail {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Parameters for an MCP request, allowing additional arbitrary fields via `flatten`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCPRequestParams {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,

    /// Arbitrary extra fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// `_meta` field for MCP requests, optionally containing a progress token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMeta {
    #[serde(rename = "progressToken", skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
}

/// Parameters for an MCP notification, allowing additional arbitrary fields via `flatten`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MCPNotificationParams {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Base result type for MCP responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCPResultBase {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Indicates success but carries no data.
pub type EmptyResult = MCPResultBase;

/// Represents parameters for a cancelled-notification, which can be sent by either side.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledNotificationParams {
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for initializing communication (client -> server).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: Implementation,
}

/// A result returned by the server after an `initialize` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,

    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: Implementation,

    /// Optional instructions from the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Describes capabilities a client might support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<HashMap<String, Value>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Describes whether the client supports updated-list notifications for roots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// A set of capabilities the server may support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Indicates server support for prompt-related features.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Indicates server support for resource-related features.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Indicates server support for tool-related features.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Represents the name and version of an MCP implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Implementation {
    pub name: String,
    pub version: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for the `ping` method (client or server). Generally empty.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PingParams {}

/// Parameters for a progress notification, typically referencing a long-running request.
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

/// A structure for request parameters that may involve pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _meta: Option<RequestMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Indicates that a result can include pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result containing a list of resources known to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    pub resources: Vec<Resource>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result containing a list of resource templates known to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    pub resource_templates: Vec<ResourceTemplate>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for the `resources/read` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResourceParams {
    pub uri: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result from the `resources/read` method, containing resource contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResourceResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub contents: Vec<ResourceContents>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for `resources/subscribe`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeParams {
    pub uri: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for `resources/unsubscribe`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeParams {
    pub uri: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for a `notifications/resources/updated` message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUpdatedParams {
    pub uri: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A resource object that the server can read, possibly with extra metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub uri: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(flatten)]
    pub annotated: Annotated,
}

/// A resource template, which can be used to generate resource URIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
    pub uri_template: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(flatten)]
    pub annotated: Annotated,
}

/// Contents of a resource. May be text or binary data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceContents {
    Text(TextResourceContents),
    Blob(BlobResourceContents),
}

/// Represents textual resource contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextResourceContents {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub text: String,
}

/// Represents binary resource contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobResourceContents {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub blob: String,
}

/// A result containing a list of prompts known to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPromptsResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,

    pub prompts: Vec<Prompt>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for `prompts/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result returned by `prompts/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPromptResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A prompt object or prompt template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Arguments accepted by a prompt, potentially required.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptArgument {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A role in a conversation: either "user" or "assistant".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// A message returned as part of a prompt result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptMessage {
    pub role: Role,
    pub content: PromptContent,
}

/// Represents the content of a prompt message: text, image, or embedded resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromptContent {
    Text(TextContent),
    Image(ImageContent),
    Resource(EmbeddedResource),
}

/// An embedded resource, which can contain a text or blob resource internally.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedResource {
    #[serde(rename = "type")]
    pub kind: String, // e.g., "resource"
    pub resource: ResourceContents,

    #[serde(flatten)]
    pub annotated: Annotated,
}

/// Allows attaching optional annotations and arbitrary extra fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Contains optional annotation data such as `audience` or `priority`.
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

/// Represents text content in a prompt or message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    #[serde(rename = "type")]
    pub kind: String, // "text"
    pub text: String,

    #[serde(flatten)]
    pub annotated: Annotated,
}

/// Represents image content, stored in base64.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub kind: String, // "image"
    pub data: String,
    pub mime_type: String,

    #[serde(flatten)]
    pub annotated: Annotated,
}

/// A result listing server-provided tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    pub tools: Vec<Tool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for the `tools/call` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, Value>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result from the `tools/call` method, potentially indicating an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub content: Vec<PromptContent>,
    
    /// Structured content that conforms to the tool's output schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<Value>,

    /// True if the tool call ended in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Annotations that describe tool behavior hints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destructive_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_world_hint: Option<bool>,
}

/// Defines a tool that can be invoked by the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub input_schema: ToolInputSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Describes the schema for a tool's input parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInputSchema {
    #[serde(rename = "type")]
    pub type_: String, // typically "object"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// Parameters for enabling or adjusting server-side logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLevelParams {
    pub level: LoggingLevel,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Syslog-like logging severity levels.
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

/// A notification with a log message from the server.
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

/// Parameters for the `sampling/createMessage` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageParams {
    pub messages: Vec<SamplingMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>, // "none" | "thisServer" | "allServers"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub max_tokens: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result from `sampling/createMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub role: Role,
    pub content: SamplingContent,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents a text or image message in sampling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SamplingContent {
    Text(TextContent),
    Image(ImageContent),
}

/// A sampling message (one item in `CreateMessageParams`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingMessage {
    pub role: Role,
    pub content: SamplingContent,
}

/// Preferences for selecting a model, including cost or speed priorities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<ModelHint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence_priority: Option<f64>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A hint to use when selecting a model (e.g., substring matches).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelHint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for `completion/complete`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteParams {
    #[serde(rename = "ref")]
    pub r#ref: ReferenceType,
    pub argument: CompleteArgument,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result from `completion/complete`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub completion: CompletionData,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A reference to either a resource or a prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ReferenceType {
    #[serde(rename = "ref/resource")]
    Resource { uri: String },
    #[serde(rename = "ref/prompt")]
    Prompt { name: String },
}

/// An argument for `completion/complete` (name + value).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteArgument {
    pub name: String,
    pub value: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Data returned in the `completion` field, containing possible completions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionData {
    pub values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// Parameters for `roots/list`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsParams {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A result listing root URIs from the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsResult {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, Value>>,
    pub roots: Vec<Root>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Represents a root directory or file, typically starting with `file://`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parameters for the elicitation/create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElicitationCreateParams {
    /// The prompt message to display to the user.
    pub message: String,
    
    /// A JSON Schema defining the structure of the expected user response.
    pub requested_schema: Value,
    
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Result from the elicitation/create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElicitationCreateResult {
    /// The action taken by the user.
    pub action: ElicitationAction,
    
    /// The user's response conforming to the requested schema (if accepted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Value>,
    
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Possible actions for elicitation responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ElicitationAction {
    Accept,
    Reject,
    Cancel,
}
/// A union of all possible client requests. The `method` field identifies the variant.
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
        params: CompleteParams,
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
    #[serde(rename = "elicitation/create")]
    ElicitationCreate {
        #[serde(rename = "jsonrpc")]
        json_rpc: String,
        id: RequestId,
        params: ElicitationCreateParams,
    },
}

/// A union of all possible client notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ClientNotification {
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

/// A union of possible server requests.
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

/// A union of possible server notifications.
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

/// A union of all possible server results.
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
    ElicitationCreate(ElicitationCreateResult),
}
