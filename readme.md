# rsGPT

`rsGPT` is a Rust-based chat application that leverages the power of GPT (Generative Pre-trained Transformer) to create interactive conversation. This application uses the `hyper` crate for handling HTTP requests, `tokio` for asynchronous tasks, and various other libraries for data serialization, coloring, and string manipulation.

## Features

- **Chart Generation**: Create visually appealing charts using the underlying GPT-powered engine.
- **HTTP Server**: Serve generated charts via an HTTP server for easy access and sharing.
- **Asynchronous**: Utilize the `tokio` runtime for efficient asynchronous operations.
- **Data Serialization**: Leverage `serde` and `serde_json` for data serialization and deserialization.
- **Colorful Output**: Make your console output more colorful with the `colored` crate.
- **String Building**: Efficiently build strings using the `string-builder` crate.
- **Persistent Storage**: Use the `rustbreak` crate for managing persistent data storage.

## Getting Started

1. Clone this repository: `git clone https://github.com/your-username/rsGPT.git`
2. Navigate to the project directory: `cd rsGPT`
3. Build the project: `cargo build`
4. Run the application: `cargo run`

## Dependencies

- [hyper](https://crates.io/crates/hyper): A fast and asynchronous HTTP library for Rust.
- [hyper-tls](https://crates.io/crates/hyper-tls): TLS support for the `hyper` crate.
- [tokio](https://crates.io/crates/tokio): Asynchronous runtime for Rust.
- [serde](https://crates.io/crates/serde): A framework for serializing and deserializing Rust data structures.
- [serde_json](https://crates.io/crates/serde_json): JSON support for Serde.
- [colored](https://crates.io/crates/colored): Add color to your terminal output.
- [string-builder](https://crates.io/crates/string-builder): Efficient string building.
- [rustbreak](https://crates.io/crates/rustbreak): A simple, efficient, and flexible database.

## Usage

```bash
# Build the project
cargo build

# Run the application
cargo run
