use mcp_schema::{
    ClientCapabilities, Implementation, InitializeParams, InitializeResult, JSONRPCRequest,
    JSONRPCResponse, RequestId, ServerCapabilities, JSONRPC_VERSION, LATEST_PROTOCOL_VERSION,
};
use std::collections::HashMap;

fn main() {
    // Create and serialize an InitializeRequest
    let init_req = JSONRPCRequest {
        json_rpc: JSONRPC_VERSION.to_string(),
        method: "initialize".to_string(),
        id: RequestId::Number(1),
        params: InitializeParams {
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities {
                experimental: None,
                roots: None,
                sampling: None,
                extra: HashMap::new(),
            },
            client_info: Implementation {
                name: "MyClient".into(),
                version: "1.0".into(),
                extra: HashMap::new(),
            },
        },
    };

    // Serialize to JSON
    let json_str = serde_json::to_string_pretty(&init_req).unwrap();
    println!("InitializeRequest (serialized):\n{}\n", json_str);

    // Create and serialize an InitializeResponse
    let init_res = JSONRPCResponse {
        json_rpc: JSONRPC_VERSION.to_string(),
        id: RequestId::Number(1),
        result: InitializeResult {
            meta: None,
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            capabilities: ServerCapabilities {
                experimental: None,
                logging: None,
                prompts: None,
                resources: None,
                tools: None,
                extra: HashMap::new(),
            },
            server_info: Implementation {
                name: "MyServer".into(),
                version: "1.2".into(),
                extra: HashMap::new(),
            },
            instructions: Some("Welcome to MyServer!".into()),
            extra: HashMap::new(),
        },
    };

    // Serialize the response to JSON
    let res_json = serde_json::to_string_pretty(&init_res).unwrap();
    println!("InitializeResponse (serialized):\n{}", res_json);
}
