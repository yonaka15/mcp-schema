//! Core JSON-RPC and MCP base types
//!
//! This module provides the fundamental types used in both JSON-RPC and MCP communications.
//! It includes request/response structures, error handling, and common utility types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Current JSON-RPC version (always "2.0")
pub const JSONRPC_VERSION: &str = "2.0";

/// Latest supported Model Context Protocol version
pub const LATEST_PROTOCOL_VERSION: &str = "2024-11-05";

// Standard JSON-RPC error codes
/// Parse error (-32700): Invalid JSON
pub const PARSE_ERROR: i32 = -32700;
/// Invalid Request (-32600): JSON is valid but not a valid request object
pub const INVALID_REQUEST: i32 = -32600;
/// Method not found (-32601): Method does not exist or is unavailable
pub const METHOD_NOT_FOUND: i32 = -32601;
/// Invalid params (-32602): Method parameters are invalid
pub const INVALID_PARAMS: i32 = -32602;
/// Internal error (-32603): Internal JSON-RPC error
pub const INTERNAL_ERROR: i32 = -32603;

/// A request ID for JSON-RPC, which can be either a string or a number.
///
/// # Examples
///
/// ```rust
/// use mcp_schema::types::RequestId;
///
/// let id: RequestId = 1.into(); // Using number
/// let id: RequestId = "request-1".to_string().into(); // Using string
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

impl From<i64> for RequestId {
    fn from(n: i64) -> Self {
        RequestId::Number(n)
    }
}

impl From<String> for RequestId {
    fn from(s: String) -> Self {
        RequestId::String(s)
    }
}

impl From<&str> for RequestId {
    fn from(s: &str) -> Self {
        RequestId::String(s.to_string())
    }
}

/// A progress token for associating progress notifications with a request.
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
/// * `T` - The type of the request parameters
///
/// # Examples
///
/// ```rust
/// use mcp_schema::types::{JSONRPCRequest, JSONRPC_VERSION};
/// use serde_json::Value;
///
/// let request: JSONRPCRequest<Value> = JSONRPCRequest {
///     json_rpc: JSONRPC_VERSION.to_string(),
///     method: "example".to_string(),
///     id: 1.into(),
///     params: serde_json::json!({"key": "value"}),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCRequest<T> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub method: String,
    pub id: RequestId,
    pub params: T,
}

/// A generic JSON-RPC notification (no response expected).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCNotification<T> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub method: String,
    pub params: T,
}

/// A generic JSON-RPC successful response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<U> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: RequestId,
    pub result: U,
}

/// A JSON-RPC error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCError {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    pub id: RequestId,
    pub error: RPCErrorDetail,
}

/// Detailed error information for a JSON-RPC error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCErrorDetail {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Base parameters for MCP requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCPRequestParams {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Metadata for MCP requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMeta {
    #[serde(rename = "progressToken", skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
}

/// Base parameters for MCP notifications.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_id_serialization() {
        let number_id: RequestId = 1.into();
        let string_id: RequestId = "test-1".into();

        let json_number = serde_json::to_value(&number_id).unwrap();
        let json_string = serde_json::to_value(&string_id).unwrap();

        assert_eq!(json_number, json!(1));
        assert_eq!(json_string, json!("test-1"));
    }

    #[test]
    fn test_jsonrpc_request_serialization() {
        let request: JSONRPCRequest<Value> = JSONRPCRequest {
            json_rpc: JSONRPC_VERSION.to_string(),
            method: "test".to_string(),
            id: 1.into(),
            params: json!({"test": true}),
        };

        let json = serde_json::to_value(&request).unwrap();
        
        assert_eq!(json["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(json["method"], "test");
        assert_eq!(json["id"], 1);
        assert_eq!(json["params"]["test"], true);
    }

    #[test]
    fn test_jsonrpc_error_serialization() {
        let error = JSONRPCError {
            json_rpc: JSONRPC_VERSION.to_string(),
            id: 1.into(),
            error: RPCErrorDetail {
                code: INVALID_REQUEST,
                message: "Invalid request".to_string(),
                data: Some(json!({"details": "Missing required field"})),
            },
        };

        let json = serde_json::to_value(&error).unwrap();
        
        assert_eq!(json["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(json["id"], 1);
        assert_eq!(json["error"]["code"], INVALID_REQUEST);
        assert_eq!(json["error"]["message"], "Invalid request");
        assert_eq!(json["error"]["data"]["details"], "Missing required field");
    }

    #[test]
    fn test_mcp_request_params() {
        let params = MCPRequestParams {
            meta: Some(RequestMeta {
                progress_token: Some(ProgressToken::String("token-1".to_string())),
            }),
            extra: {
                let mut map = HashMap::new();
                map.insert("custom".to_string(), json!("value"));
                map
            },
        };

        let json = serde_json::to_value(&params).unwrap();
        
        assert_eq!(json["_meta"]["progressToken"], "token-1");
        assert_eq!(json["custom"], "value");
    }

    #[test]
    fn test_notification_serialization() {
        let notification = JSONRPCNotification {
            json_rpc: JSONRPC_VERSION.to_string(),
            method: "test/notification".to_string(),
            params: MCPNotificationParams::default(),
        };

        let json = serde_json::to_value(&notification).unwrap();
        
        assert_eq!(json["jsonrpc"], JSONRPC_VERSION);
        assert_eq!(json["method"], "test/notification");
        assert!(json["params"].as_object().unwrap().is_empty());
    }
}