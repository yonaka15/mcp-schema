// Module declarations
pub mod base;
pub mod cancellation;
pub mod client;
pub mod client_completion;
pub mod common;
pub mod initialization;
pub mod logging;
pub mod progress;
pub mod prompts;
pub mod resources;
pub mod roots;
pub mod sampling;
pub mod server;
pub mod tools;

// Re-exports from base
pub use base::{
    Cursor, EmptyResult, JSONRPCError, JSONRPCNotification, JSONRPCRequest, JSONRPCResponse,
    MCPNotificationParams, MCPRequestParams, MCPResultBase, ProgressToken, RPCErrorDetail, RequestId,
    JSONRPC_VERSION, LATEST_PROTOCOL_VERSION,
};

// Re-exports from common
pub use common::{
    Annotated, Annotations, ImageContent, PaginatedParams, PaginatedResult, Role, TextContent,
};

// Re-exports from initialization
pub use initialization::{
    ClientCapabilities, Implementation, InitializeParams, InitializeResult, PromptsCapability,
    ResourcesCapability, RootsCapability, ServerCapabilities, ToolsCapability,
};

// Re-exports from resources
pub use resources::{
    BlobResourceContents, ListResourceTemplatesResult, ListResourcesResult, ReadResourceParams,
    ReadResourceResult, Resource, ResourceContents, ResourceTemplate, ResourceUpdatedParams,
    SubscribeParams, TextResourceContents, UnsubscribeParams,
};

// Re-exports from prompts
pub use prompts::{
    EmbeddedResource, GetPromptParams, GetPromptResult, ListPromptsResult, Prompt, PromptArgument,
    PromptContent, PromptMessage,
};

// Re-exports from tools
pub use tools::{
    CallToolParams, CallToolResult, ListToolsResult, Tool, ToolInputSchema,
};

// Re-exports from logging
pub use logging::{LoggingLevel, LoggingMessageParams, SetLevelParams};

// Re-exports from sampling
pub use sampling::{
    CreateMessageParams, CreateMessageResult, ModelHint, ModelPreferences, SamplingContent,
    SamplingMessage,
};

// Re-exports from client_completion
pub use client_completion::{
    CompleteArgument, CompleteParams, CompleteResult, CompletionData, ReferenceType,
};

// Re-exports from roots
pub use roots::{ListRootsParams, ListRootsResult, Root};

// Re-exports from cancellation and progress
pub use cancellation::CancelledNotificationParams;
pub use progress::ProgressNotificationParams;

// Re-exports from client and server
pub use client::{ClientNotification, ClientRequest};
pub use server::{ServerNotification, ServerRequest, ServerResult};