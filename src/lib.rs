//! MCP Schema - Rust implementation of the Model Context Protocol
//!
//! This crate provides Rust types and serialization support for the Model Context Protocol (MCP),
//! a protocol designed for secure, bidirectional integration between Large Language Model (LLM)
//! applications and external data sources or tools.
//!
//! # Features
//!
//! - Complete JSON-RPC 2.0 support
//! - Type-safe MCP message handling
//! - Serde serialization/deserialization
//! - Comprehensive error types
//!
//! # Example
//!
//! ```rust
//! use mcp_schema::types::{
//!     InitializeParams, ClientCapabilities, Implementation,
//!     JSONRPCRequest, JSONRPC_VERSION,
//! };
//!
//! // Create an initialize request
//! let request = JSONRPCRequest {
//!     json_rpc: JSONRPC_VERSION.to_string(),
//!     id: 1.into(),
//!     method: "initialize".to_string(),
//!     params: InitializeParams {
//!         protocol_version: "2024-11-05".to_string(),
//!         capabilities: ClientCapabilities::default(),
//!         client_info: Implementation {
//!             name: "example-client".to_string(),
//!             version: "1.0.0".to_string(),
//!             extra: Default::default(),
//!         },
//!     },
//! };
//! ```
//!
//! # Protocol Version
//!
//! This implementation targets MCP version 2024-11-05.

pub mod types;

// Re-export commonly used types at the crate root
pub use types::{
    ClientCapabilities,
    ClientNotification,
    // Message enums
    ClientRequest,
    Cursor,

    Implementation,

    // Core protocol types
    InitializeParams,
    InitializeResult,
    JSONRPCError,
    JSONRPCNotification,
    // Base JSON-RPC types
    JSONRPCRequest,
    JSONRPCResponse,
    ProgressToken,
    RequestId,
    ServerCapabilities,
    ServerNotification,
    ServerRequest,
    ServerResult,
    // Protocol constants
    JSONRPC_VERSION,
    LATEST_PROTOCOL_VERSION,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_initialize_request_serialization() {
        let request = JSONRPCRequest {
            json_rpc: JSONRPC_VERSION.to_string(),
            id: RequestId::Number(1),
            method: "initialize".to_string(),
            params: InitializeParams {
                protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test-client".to_string(),
                    version: "0.1.0".to_string(),
                    extra: Default::default(),
                },
            },
        };

        let json = serde_json::to_value(&request).unwrap();

        assert_eq!(json["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(json["method"], "initialize");
        assert_eq!(json["id"], 1);
        assert_eq!(json["params"]["protocolVersion"], LATEST_PROTOCOL_VERSION);
        assert_eq!(json["params"]["clientInfo"]["name"], "test-client");
        assert_eq!(json["params"]["clientInfo"]["version"], "0.1.0");
    }

    #[test]
    fn test_initialize_response_deserialization() {
        let json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "logging": {},
                    "prompts": {
                        "listChanged": true
                    }
                },
                "serverInfo": {
                    "name": "test-server",
                    "version": "0.1.0"
                }
            }
        });

        let response: JSONRPCResponse<InitializeResult> = serde_json::from_value(json).unwrap();
        assert_eq!(response.json_rpc, JSONRPC_VERSION);
        assert!(matches!(response.id, RequestId::Number(1)));
        assert_eq!(response.result.protocol_version, LATEST_PROTOCOL_VERSION);
        assert_eq!(response.result.server_info.name, "test-server");
        assert_eq!(response.result.server_info.version, "0.1.0");
    }
}

