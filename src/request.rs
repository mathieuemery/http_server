use crate::utils::{HTTPMethod, HTTPVersion, RequestParseError};

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub target: String,
    pub http_version: HTTPVersion,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request{
    pub fn from_str(request: String) -> Result<Self, RequestParseError>{
        // Separate the blocks for the request line, headers and body
        let blocks = request.split("\r\n").collect::<Vec<&str>>();

        if blocks.is_empty() {
            println!("Empty request");
            return Err(RequestParseError::InvalidRequestLine)
        }

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

        let method = method.unwrap();

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
        let http_version = http_version.unwrap();

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