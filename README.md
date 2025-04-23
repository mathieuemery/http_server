# HTTP Server in Rust

A lightweight and minimalist HTTP server implemented in Rust, supporting basic HTTP operations, file serving, and content encoding.

## Features

- HTTP request handling for GET and POST methods
- Multithreaded connection handling
- File serving and uploading capabilities
- Content compression using gzip
- Basic endpoints:
  - `/` - Root endpoint returning 200 OK
  - `/hello` - Returns "Hello World!"
  - `/echo/{string}` - Echoes back the provided string
  - `/user-agent` - Returns the client's User-Agent
  - `/files/{filename}` - GET: Serves files / POST: Creates files

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Installation

1. Clone this repository
```bash
git clone https://github.com/yourusername/rust-http-server.git
cd rust-http-server
```

2. Build the project
```bash
cargo build --release
```

3. Run the server
```bash
cargo run --release
```

The server will start on `127.0.0.1:4221` by default.

### Usage

Once the server is running, you can interact with it using any HTTP client:

```bash
# Get the root endpoint
curl -i http://localhost:4221/

# Get the hello endpoint
curl -i http://localhost:4221/hello

# Echo a message
curl -i http://localhost:4221/echo/hello-world

# Get your user agent
curl -i http://localhost:4221/user-agent

# Get a file
curl -i http://localhost:4221/files/example.txt

# Create a file
curl -i -X POST -d "file contents" http://localhost:4221/files/example.txt
```

## Project Structure

- `main.rs` - Server initialization and request handling
- `request.rs` - HTTP request parsing
- `response.rs` - HTTP response generation
- `files.rs` - File operations
- `utils.rs` - Common utilities and constants

## Dependencies

- `flate2` - For gzip compression
- `once_cell` - For lazy static initialization

## Future Improvements

- Enhanced error handling
- Path traversal protection
- Configurable server settings
- Comprehensive logging
- HTTPS support
- More advanced request parsing

## License

This project is open source and available under the [MIT License](LICENSE).
