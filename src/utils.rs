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