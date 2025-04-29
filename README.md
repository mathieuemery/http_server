# RustHTTP - A Lightweight HTTP Server in Rust

A lightweight multithreaded HTTP server written in Rust.

## Features

- **Multithreaded**: Uses a thread pool to handle concurrent connections
- **Request Handling**: Supports GET and POST methods
- **Path Routing**: Handles different endpoints with specific functionalities
- **File Operations**: File reading and writing with path traversal protection
- **Compression**: Supports gzip compression for responses
- **Configurable**: Command-line arguments for server address, port, and thread count

## Endpoints

The server exposes the following endpoints:

- `/`: Root endpoint, returns a 200 OK status
- `/hello`: Returns "Hello World!" with a 200 OK status
- `/echo/{string}`: Echoes back the provided string
- `/user-agent`: Returns the User-Agent header from the request
- `/files/{filename}`: 
  - GET: Retrieves a file from the server
  - POST: Creates a file on the server

## Installation

Make sure you have Rust and Cargo installed on your system. Then clone this repository and build the project:

```bash
git clone https://github.com/yourusername/rusthttp.git
cd rusthttp
cargo build --release
```

## Usage

Run the server with:

```bash
cargo run
```

Or with custom parameters:

```bash
cargo run -- --address 0.0.0.0 --port 8080 --max-thread-num 4
```

### Command Line Arguments

- `-a, --address`: Server address (default: 127.0.0.1)
- `-p, --port`: Server port (default: 4221)
- `-m, --max-thread-num`: Maximum number of threads (default: 10)

## Project Structure

- `main.rs`: Entry point of the server, sets up the TCP listener and thread pool
- `request.rs`: Defines the `Request` struct and parsing functionality
- `response.rs`: Defines the `Response` struct and response generation
- `files.rs`: Handles file operations with security measures
- `utils.rs`: Contains shared constants, helpers, and command line argument parsing

## Performance

The server uses a thread pool to handle concurrent connections. The number of threads can be configured via command-line arguments, with a default of 10 threads. The server automatically limits the maximum number of threads to the number of available CPU cores.

## Security

The server implements several security measures:

- **Path Traversal Protection**: Prevents accessing files outside the specified directory
- **Input Validation**: Sanitizes file paths and request components
- **Error Handling**: Handles invalid requests and errors

## Dependencies

- `threadpool`: For managing concurrent connections
- `flate2`: For gzip compression
- `clap`: For parsing command-line arguments
- `once_cell`: For lazy initialization of static variables
- `num_cpus`: For detecting the number of available CPU cores

## License

[MIT License](LICENSE)