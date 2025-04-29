//! utils.rs
//!
//! Contains shared constants, helpers, and command line argument parsing

use clap::Parser;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Stores the HTTP response codes and their meanings.
/// 
/// Static variable that is initialized once and can be used throughout the program.
pub static HTTP_RESPONSE_CODES : Lazy<HashMap<u16, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(200, "OK".to_string());
    m.insert(201, "Created".to_string());
    m.insert(400, "Bad request".to_string());
    m.insert(404, "Not Found".to_string());
    m.insert(500, "Internal Server Error".to_string());
    m
});

/// Stores the some HTTP compression algorithms (not all implemented).
pub static COMPRESSION_ALGORITHMS : Lazy<Vec<&str>> = Lazy::new(|| {
    vec!["gzip", "deflate", "br"]
});

/// Error types for parsing HTTP requests.
#[derive(Debug)]
pub enum RequestParseError {
    InvalidRequestLine,
    InvalidMethod,
    InvalidVersion,
    InvalidHeader,
}

/// Most common HTTP methods.
#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    /// Transforms a string into an `HTTPMethod`.
    ///
    /// # Arguments
    ///
    /// * `method` - The method of the request (e.g., "GET", "POST").
    pub fn from_str(method: &str) -> Option<HTTPMethod> {
        match method {
            "GET" => Some(HTTPMethod::GET),
            "POST" => Some(HTTPMethod::POST),
            "PUT" => Some(HTTPMethod::PUT),
            "DELETE" => Some(HTTPMethod::DELETE),
            _ => None,
        }
    }
}

/// Existing HTTP versions.
#[derive(Debug)]
pub enum HTTPVersion{
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}

impl HTTPVersion {
    /// Transforms a string into an `HTTPVersion`.
    ///
    /// # Arguments
    ///
    /// * `version` - HTTP version of the request (e.g., "HTTP/1.0", "HTTP/1.1").
    pub fn from_str(version: &str) -> Option<HTTPVersion> {
        match version {
            "HTTP/1.0" => Some(HTTPVersion::Http1_0),
            "HTTP/1.1" => Some(HTTPVersion::Http1_1),
            "HTTP/2.0" => Some(HTTPVersion::Http2_0),
            "HTTP/3.0" => Some(HTTPVersion::Http3_0),
            _ => None,
        }
    }

    /// Transforms a `HTTPVersion`` into an String.
    pub fn to_str(&self) -> &str {
        match self {
            HTTPVersion::Http1_0 => "HTTP/1.0",
            HTTPVersion::Http1_1 => "HTTP/1.1",
            HTTPVersion::Http2_0 => "HTTP/2.0",
            HTTPVersion::Http3_0 => "HTTP/3.0",
        }
    }
}

/// Command line arguments parser.
#[derive(Parser, Debug)]
#[command(author = "Mathieu Emery", version, about="A Very simple http server")]
pub struct Args {
    // Address of the server
    #[arg(short, long, help = "Address of the server (default: 127.0.0.1)")]
    address: Option<String>,
    
    // Port to run the server on
    #[arg(short, long, help = "Port to run the server on (default: 4221)")]
    port: Option<u32>,

    // Max amount of threads
    #[arg(short, long, help = "Maximum amount of threads (default: 10)")]
    max_thread_num: Option<usize>,
}

impl Args{
    /// Parses the arguments into a `ServerParams` struct.
    pub fn parse_params() -> Result<ServerParams, String> {
        let args = Args::parse();

        // Get the maximum number of threads available on the system
        let max_threads = num_cpus::get();
        if max_threads == 0 {
            return Err("No CPU detected".to_string());
        }

        let mut num_threads = args.max_thread_num.unwrap_or(10);

        // Set the number of threads to the minimum of the user-specified value and the maximum available
        if num_threads > max_threads{
            eprintln!("Warning: Maximum number of threads is set to the maximum available: {}", max_threads);
            num_threads = max_threads;
        }

        let address = args.address.unwrap_or("127.0.0.1".to_string());

        // Validate the address
        if !Self::check_address(&address) {
            return Err("Invalid address".to_string());
        }

        let port = args.port.unwrap_or(4221);

        // Validate the port number
        if port > 65535 {
            return Err("Port number must be between 0 and 65535".to_string());
        }

        Ok(ServerParams{
            address,
            port,
            nb_threads: num_threads,
        })
    }

    /// Check if the given address is valid.
    fn check_address(address: &str) -> bool {
        // Check if the address is a valid IP address or hostname
        // This is a simple check, you might want to use a more robust method
        address.parse::<std::net::IpAddr>().is_ok() || address.parse::<std::net::SocketAddr>().is_ok()
    }
}

/// Parameters for the server.
pub struct ServerParams{
    address: String,
    port: u32,
    pub nb_threads: usize,
}

impl ServerParams {
    /// Returns the address and port of the server as a string.
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}