//! request.rs
//!
//! This file defines the `Request` struct and its methods.
//! It is responsible for parsing incoming HTTP requests into structured data.

use crate::utils::{HTTPMethod, HTTPVersion, RequestParseError};

/// `Request` struct represents an HTTP request.
///
/// It contains the HTTP method, target, HTTP version, headers, and body.
#[derive(Debug)]
pub struct Request {
    /// The HTTP method (GET, POST, etc.)
    pub method: HTTPMethod,

    /// The requested path or resource
    pub target: String,

    /// The HTTP version (HTTP/1.1, HTTP/2, etc.)
    pub http_version: HTTPVersion,

    /// The headers of the request
    pub headers: Vec<(String, String)>,

    /// The body of the request
    pub body: String,
}

impl Request{
    /// Create a new `Request` object from a string.
    ///
    /// Reads and parse the incoming HTTP request string.
    /// Checks the validity of the request line, headers, and body.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming HTTP request string.
    pub fn from_str(request: String) -> Result<Self, RequestParseError>{
        // Separate the blocks for the request line, headers and body
        let blocks = request.split("\r\n").collect::<Vec<&str>>();

        if blocks.is_empty() {
            println!("Empty request");
            return Err(RequestParseError::InvalidRequestLine)
        }

        // Get the request line
        let request_line = blocks[0].split_whitespace().collect::<Vec<&str>>();

        if request_line.len() != 3 {
            println!("Invalid request line");
            return Err(RequestParseError::InvalidRequestLine)
        }

        // Get the HTTP method
        let method  = HTTPMethod::from_str(request_line[0]);
        if method.is_none() {
            return Err(RequestParseError::InvalidMethod)
        }

        let method = match method{
            Some(m) => m,
            None => {
                println!("Invalid method");
                return Err(RequestParseError::InvalidMethod)
            }
        };

        // Get the target
        let target = request_line[1].to_string();

        if target.is_empty() {
            println!("Invalid target");
            return Err(RequestParseError::InvalidRequestLine)
        }

        // Get the HTTP version
        let http_version = HTTPVersion::from_str(request_line[2]);
        if http_version.is_none() {
            return Err(RequestParseError::InvalidVersion)
        }
        let http_version = match http_version {
            Some(v) => v,
            None => {
                println!("Invalid version");
                return Err(RequestParseError::InvalidVersion)
            }
        };

        // Get the headers and body
        let mut headers = Vec::new();
        let mut body = String::new();

        for i in 1..blocks.len() {
            let header = blocks[i].split(": ").collect::<Vec<&str>>();
            if header.len() == 2 {
                headers.push((header[0].to_string(), header[1].to_string()));
            } else if header.len() == 1 {
                body = blocks[i].to_string();
            }
        }

        if headers.is_empty() {
            return Err(RequestParseError::InvalidHeader)
        }

        // Create the request object
        Ok(Self {
            method,
            target,
            http_version,
            headers,
            body,
        })

    }
}