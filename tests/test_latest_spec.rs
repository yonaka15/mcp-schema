use mcp_schema::*;
use serde_json::json;

#[test]
fn test_tool_with_annotations() {
    let tool_json = json!({
        "name": "test_tool",
        "title": "Test Tool",
        "description": "A test tool",
        "inputSchema": {
            "type": "object",
            "properties": {
                "input": {"type": "string"}
            }
        },
        "outputSchema": {
            "type": "object",
            "properties": {
                "result": {"type": "string"}
            }
        },
        "annotations": {
            "readOnlyHint": true,
            "destructiveHint": false,
            "idempotentHint": true,
            "openWorldHint": false
        }
    });

    let tool: Tool = serde_json::from_value(tool_json).unwrap();
    assert_eq!(tool.name, "test_tool");
    assert_eq!(tool.title, Some("Test Tool".to_string()));
    assert!(tool.output_schema.is_some());
    assert!(tool.annotations.is_some());
    
    let annotations = tool.annotations.unwrap();
    assert_eq!(annotations.read_only_hint, Some(true));
    assert_eq!(annotations.destructive_hint, Some(false));
}

#[test]
fn test_call_tool_result_with_structured_content() {
    let result_json = json!({
        "content": [
            {
                "type": "text",
                "text": "Result text"
            }
        ],
        "structuredContent": {
            "temperature": 22.5,
            "humidity": 65
        },
        "isError": false
    });

    let result: CallToolResult = serde_json::from_value(result_json).unwrap();
    assert_eq!(result.content.len(), 1);
    assert!(result.structured_content.is_some());
    assert_eq!(result.is_error, Some(false));
    
    let structured = result.structured_content.unwrap();
    assert_eq!(structured["temperature"], 22.5);
    assert_eq!(structured["humidity"], 65);
}

#[test]
fn test_elicitation_create_request() {
    let request_json = json!({
        "jsonrpc": "2.0",
        "method": "elicitation/create",
        "id": 1,
        "params": {
            "message": "Please provide your email",
            "requestedSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "format": "email"
                    }
                },
                "required": ["email"]
            }
        }
    });

    let request: ClientRequest = serde_json::from_value(request_json).unwrap();
    
    if let ClientRequest::ElicitationCreate { params, .. } = request {
        assert_eq!(params.message, "Please provide your email");
        assert!(params.requested_schema.is_object());
    } else {
        panic!("Expected ElicitationCreate variant");
    }
}

#[test]
fn test_elicitation_create_result() {
    let accept_result = json!({
        "action": "accept",
        "content": {
            "email": "user@example.com"
        }
    });

    let result: ElicitationCreateResult = serde_json::from_value(accept_result).unwrap();
    assert!(matches!(result.action, ElicitationAction::Accept));
    assert!(result.content.is_some());
    
    let reject_result = json!({
        "action": "reject"
    });
    
    let result: ElicitationCreateResult = serde_json::from_value(reject_result).unwrap();
    assert!(matches!(result.action, ElicitationAction::Reject));
    assert!(result.content.is_none());
}

#[test]
fn test_backward_compatibility() {
    // Verify that old-style tools without new fields still work
    let tool_json = json!({
        "name": "old_tool",
        "description": "An old tool",
        "inputSchema": {
            "type": "object"
        }
    });

    let tool: Tool = serde_json::from_value(tool_json).unwrap();
    assert_eq!(tool.name, "old_tool");
    assert!(tool.title.is_none());
    assert!(tool.output_schema.is_none());
    assert!(tool.annotations.is_none());
    
    // Verify old-style CallToolResult still works
    let result_json = json!({
        "content": [
            {
                "type": "text",
                "text": "Result"
            }
        ]
    });
    
    let result: CallToolResult = serde_json::from_value(result_json).unwrap();
    assert!(result.structured_content.is_none());
}