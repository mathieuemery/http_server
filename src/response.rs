//! response.rs
//!
//! This file defines the `Response` struct and its methods.
//! It is responsible for generating HTTP responses based on the request.
//!
//! It uses the `flate2` crate for encoding and decoding gzip content.

extern crate flate2;

use std::vec;
use std::io::Write;
use flate2::Compression;

use crate::request::Request;
use crate::files::{get_file_content, create_file};
use crate::utils::{HTTPVersion, HTTP_RESPONSE_CODES, COMPRESSION_ALGORITHMS, HTTPMethod};

/// `Response` struct represents an HTTP response.
/// 
/// It contains the HTTP version, status code, headers, and body.
pub struct Response{
    /// The HTTP version (HTTP/1.1, HTTP/2, etc.)
    http_version: HTTPVersion,

    /// The HTTP status code (200, 404, etc.)
    status_code: u16,

    /// The headers of the response
    pub headers: Vec<(String, String)>,

    /// The body of the response
    body: Option<Vec<u8>>,
}

impl Response{
    /// Create a new `Response` object.
    ///
    /// Generates the HTTP response based on the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming HTTP request.
    /// * `content_type` - The content type of the response (e.g., "text/plain").
    /// * `status_code` - The HTTP status code (e.g., 200, 404).
    /// * `body` - The body of the response (e.g., "Hello World!").
    pub fn new(request: Request, content_type: Option<String>, status_code: u16, body: Option<String>) -> Self {
        // Use the HTTP version from the  request
        let http_version = request.http_version;

        // Check if a body is provided
        match body{
            Some(b) => {
                let mut content_length = b.len().to_string();
                let mut headers = vec![];
                let mut raw_body = b.into_bytes();

                // Check if an "Accept-Encoding" header is present
                if let Some((_, v)) = request.headers.iter().find(|(k, _)| k == "Accept-Encoding") {
                    for encoding in v.split(',') {
                        let encoding = encoding.trim();
                        // Check if a special encoding is requested
                        if COMPRESSION_ALGORITHMS.contains(&encoding) {
                            headers.push(("Content-Encoding".to_string(), encoding.trim().to_string()));
                            // If gzip is requested, compress the body
                            if encoding == "gzip" {
                                let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
                                encoder.write_all(&raw_body).unwrap();
                                raw_body = encoder.finish().unwrap();
                                // Compute the new content length
                                content_length = raw_body.len().to_string();
                                break;
                            }
                        }
                    }
                }

                // Check if the "Connection: close" header is present to add it to the response
                if request.headers.contains(&("Connection".to_string(), "close".to_string())) {
                    headers.push(("Connection".to_string(), "close".to_string()));
                }

                // Add the other headers
                headers.push(("Content-Type".to_string(), content_type.unwrap()));
                headers.push(("Content-Length".to_string(), content_length));

                Self {
                    http_version,
                    status_code,
                    headers,
                    body: Some(raw_body),
                }
            },
            // If no body was provided
            None => {
                return Self {
                    http_version,
                    status_code,
                    headers: vec![],
                    body: None,
                }
            }
        }
    }

    /// Parse the incoming request and generate a response.
    ///
    /// Check the request target and method to determine the appropriate response.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming HTTP request.
    pub fn parse_request(request: Request) -> Self {
        // For the "/echo" endpoint, echo back the content
        if request.target.starts_with("/echo/"){
            let content  = request.target.split("/").collect::<Vec<&str>>()[2..].join("/");
            if content.len() > 0 {
                return Response::new(request, Some("text/plain".to_string()), 200, Some(content));
            }
            else{
                return Response::new(request, None,  404, None);
            }
        }
        // For the "/files" endpoint, handle file creation and retrieval
        else if request.target.starts_with("/files/"){
            let content  = request.target.split("/").collect::<Vec<&str>>()[2..].join("/");
            if content.len() > 0 {
                // Check if the request method is POST for file creation
                if request.method == HTTPMethod::POST{
                    // Create the file with the provided content
                    match create_file(content, request.body.clone()) {
                        Ok(_)  => {
                            return Response::new(request, None, 201, None);
                        }
                        Err(_) => {
                            return Response::new(request, None, 500, None);
                        }
                    }
                }
                // Else (GET method), handle file retrieval
                else{
                    // Retrieve the file content
                    match get_file_content(content) {
                        Some(s) => {
                            return Response::new(request, Some("application/octet-stream".to_string()), 200, Some(s));
                        }
                        None => {
                            return Response::new(request, None, 404, None);
                        }
                    }
                }
            }
            else{
                return Response::new(request, None, 404, None);
            }
        }
        // For the "/hello" endpoint, return a "Hello World!" message
        else if request.target == "/hello"{
            return Response::new(request, Some("text/plain".to_string()), 200, Some("Hello World!".to_string()));
        }
        // For the "/user-agent" endpoint, return the User-Agent header provided by the client
        else if request.target == "/user-agent"{
            let user_agent = request.headers
            .iter()
            .find(|(k, _)| k == "User-Agent")
            .map(|(_, v)| v.clone());
    
            return Response::new(request, Some("text/plain".to_string()), 200, Some(user_agent.unwrap().clone()));
        }
        // For an invalid target, return a 404 Not Found response
        else if request.target != "/" {
            return Response::new(request, None, 404, None);
        }
        // For the root endpoing, return a 200 OK response without content
        else{
            return Response::new(request, None, 200, None);
        }
    }

    /// Convert the response structure to a byte array
    ///
    /// Respect the HTTP protocol format for the response.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut response = format!(
            "{} {} {}\r\n",
            self.http_version.to_str(),
            self.status_code,
            HTTP_RESPONSE_CODES.get(&self.status_code).unwrap_or(&"Unknown".to_string())
        )
        .into_bytes();

        for (key, value) in &self.headers {
            response.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }
        response.extend_from_slice(b"\r\n");

        if let Some(ref b) = self.body {
            response.extend_from_slice(b);
        }

        response
    }
}