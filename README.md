# mcp-schema

This repository (`mcp-schema`) contains a **Rust port** of the official [Model Context Protocol (MCP)](https://github.com/modelcontextprotocol/specification/blob/main/schema/schema.ts) schema originally written in TypeScript.

## What is MCP?

**Model Context Protocol (MCP)** is an open protocol designed to enable **secure, bidirectional integration** between Large Language Model (LLM) applications and external data sources or tools. Its key characteristics include:

1. **Primary Goals**

   - Enables seamless integration between LLM-based applications and external data sources or tools.
   - Provides secure two-way communication between AI systems and data sources.

2. **Main Features**

   - Allows servers to expose tools callable by language models.
   - Facilitates interactions with external systems (e.g., database queries, API calls, computations).
   - Adopts a client-host-server architecture, where a host can run multiple client instances.

3. **Use Cases**

   - Building AI-enhanced IDE (Integrated Development Environment) features.
   - Extending chat interfaces with advanced functionalities.
   - Creating custom AI workflows that integrate multiple tools.

4. **Benefits**
   - Maintains clear security boundaries while unifying AI functionality across different applications.
   - Provides standardized interfaces, making it easier to connect disparate systems.
   - Ensures secure, bidirectional connections between data sources and AI tools.

## Why Port MCP Schema to Rust?

Rust offers several advantages for complex protocol implementations like MCP:

- **Type Safety**  
  Rust’s static type system catches logical errors at compile time, reducing runtime bugs.

- **Flexible Data Conversion**  
  Using `serde`, Rust can seamlessly serialize/deserialize structured data to/from JSON, making it ideal for protocol messages.

- **Compile-Time Constraint Checks**  
  The Rust compiler enforces constraints early, helping ensure correctness before deployment.

- **Customizable Serialization/Deserialization**  
  With Rust traits and derive macros, you can fine-tune how MCP messages are structured and validated, ensuring robust and secure communication.

These features are particularly valuable for web and microservice architectures, where **strong reliability and security** are crucial.

## Repository Overview

- **`src/`**  
  Contains the Rust code that mirrors the structure and definitions from the official TypeScript schema.

- **`Cargo.toml`**  
  Project configuration for building and managing dependencies.

- **`README.md`**  
  This file, outlining the purpose, usage, and key benefits of porting MCP to Rust.

## Getting Started

1. **Install Rust**  
   Make sure a recent version of Rust is installed (e.g., via [rustup](https://www.rust-lang.org/tools/install)).

2. **Clone this repository**
   ```bash
   git clone https://github.com/yonaka15/mcp-schema.git
   cd mcp-schema
   ```

## License

This project is licensed under the MIT License—see the [LICENSE](LICENSE) file for details.
