# mcp-schema

[![Crates.io](https://img.shields.io/crates/v/mcp-schema.svg)](https://crates.io/crates/mcp-schema)
[![Documentation](https://docs.rs/mcp-schema/badge.svg)](https://docs.rs/mcp-schema)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A **Rust implementation** of the [Model Context Protocol (MCP)](https://github.com/modelcontextprotocol/specification) schema, providing type-safe definitions for building MCP clients and servers.

## What is MCP?

**Model Context Protocol (MCP)** is an open protocol that enables **secure, bidirectional communication** between Large Language Model (LLM) applications and external data sources or tools.

### Key Features

- **Tool Integration**: Expose tools and functions that can be called by language models
- **Bidirectional Communication**: Enable two-way interaction between AI systems and external services
- **Type Safety**: Strong typing for all protocol messages and data structures
- **Extensible Architecture**: Support for custom tools, prompts, and resources

### Use Cases

- Building AI-enhanced development environments
- Creating intelligent chat interfaces with external tool access
- Developing custom AI workflows with multiple tool integrations
- Enabling LLMs to interact with databases, APIs, and file systems

## Why Rust?

This Rust implementation provides several advantages:

- **Type Safety**: Catch errors at compile time with Rust's strong type system
- **Performance**: Zero-cost abstractions and efficient memory management
- **Reliability**: Memory safety guarantees without garbage collection
- **Serde Integration**: Seamless JSON serialization/deserialization
- **Cross-Platform**: Build native MCP servers and clients for any platform

## Installation

Add `mcp-schema` to your project:

```bash
cargo add mcp-schema
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
mcp-schema = "0.2.0"
```

## Usage

### Basic Example

```rust
use mcp_schema::*;
use serde_json::json;

// Create a tool with annotations
let tool = Tool {
    name: "calculate".to_string(),
    title: Some("Calculator".to_string()),
    description: Some("Perform calculations".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "expression": {"type": "string"}
        }
    }),
    output_schema: Some(json!({
        "type": "object",
        "properties": {
            "result": {"type": "number"}
        }
    })),
    annotations: Some(ToolAnnotations {
        read_only_hint: Some(true),
        destructive_hint: Some(false),
        idempotent_hint: Some(true),
        open_world_hint: Some(false),
    }),
};

// Handle tool results with structured content
let result = CallToolResult {
    content: vec![
        ContentBlock::Text(TextContent {
            text: "Calculation complete".to_string(),
        })
    ],
    structured_content: Some(json!({
        "result": 42,
        "confidence": 0.95
    })),
    is_error: Some(false),
};
```

### Elicitation API Example

```rust
use mcp_schema::*;
use serde_json::json;

// Request user input with schema validation
let elicitation = ElicitationCreateParams {
    message: "Please provide your preferences".to_string(),
    requested_schema: json!({
        "type": "object",
        "properties": {
            "theme": {
                "type": "string",
                "enum": ["light", "dark", "auto"]
            }
        },
        "required": ["theme"]
    }),
};

// Handle user response
let response = ElicitationCreateResult {
    action: ElicitationAction::Accept,
    content: Some(json!({
        "theme": "dark"
    })),
};
```

## Features (v0.2.0)

### Tool Extensions
- **Title**: Human-readable tool names
- **Output Schema**: JSON Schema validation for tool outputs
- **Annotations**: Behavioral hints for tools
  - `readOnlyHint`: Tool doesn't modify state
  - `destructiveHint`: Tool performs destructive operations
  - `idempotentHint`: Tool can be called multiple times safely
  - `openWorldHint`: Tool may return different results over time

### Structured Content
- Support for machine-readable data alongside human-readable content
- Enables rich data exchange between tools and LLMs

### Elicitation API
- Request structured input from users
- Schema validation for user responses
- Support for accept/reject/edit actions

### Full Type Coverage
- Complete implementation of MCP protocol types
- JSON-RPC 2.0 message handling
- Server capabilities and client information
- Resource management and prompts

## Project Structure

```
mcp-schema/
├── src/
│   ├── lib.rs      # Public API exports
│   └── types.rs    # Core MCP type definitions
├── tests/
│   └── test_latest_spec.rs  # Comprehensive test suite
├── Cargo.toml      # Project configuration
└── README.md       # This file
```

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- Tool annotations and extensions
- Structured content serialization
- Elicitation API functionality
- Backward compatibility
- JSON serialization/deserialization

## Development

```bash
# Build the library
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Resources

- [Official MCP Specification](https://github.com/modelcontextprotocol/specification)
- [MCP Documentation](https://modelcontextprotocol.io)
- [Crates.io Package](https://crates.io/crates/mcp-schema)
- [API Documentation](https://docs.rs/mcp-schema)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Model Context Protocol team for the original TypeScript implementation
- The Rust community for excellent serialization tools (serde)
- Contributors and users of this library