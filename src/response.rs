extern crate flate2;

use std::vec;
use std::io::Write;
use flate2::Compression;

use crate::request::Request;
use crate::files::{get_file_content, create_file};
use crate::utils::{HTTPVersion, HTTP_RESPONSE_CODES, COMPRESSION_ALGORITHMS, HTTPMethod};

pub struct Response{
    http_version: HTTPVersion,
    status_code: u16,
    pub headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl Response{
    pub fn new(request: Request, content_type: Option<String>, status_code: u16, body: Option<String>) -> Self {
        let http_version = request.http_version;

        match body{
            Some(b) => {
                let mut content_length = b.len().to_string();
                let mut headers = vec![];
                let mut raw_body = b.into_bytes();

                if let Some((_, v)) = request.headers.iter().find(|(k, _)| k == "Accept-Encoding") {
                    for encoding in v.split(',') {
                        let encoding = encoding.trim();
                        if COMPRESSION_ALGORITHMS.contains(&encoding) {
                            headers.push(("Content-Encoding".to_string(), encoding.trim().to_string()));
                            if encoding == "gzip" {
                                let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
                                encoder.write_all(&raw_body).unwrap();
                                raw_body = encoder.finish().unwrap();
                                content_length = raw_body.len().to_string();
                                break;
                            }
                        }
                    }
                }

                if request.headers.contains(&("Connection".to_string(), "close".to_string())) {
                    headers.push(("Connection".to_string(), "close".to_string()));
                }

                headers.push(("Content-Type".to_string(), content_type.unwrap()));
                headers.push(("Content-Length".to_string(), content_length));

                Self {
                    http_version,
                    status_code,
                    headers,
                    body: Some(raw_body),
                }
            },
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

    pub fn parse_request(request: Request) -> Self {
        if request.target.starts_with("/echo/"){
            let content  = request.target.split("/").collect::<Vec<&str>>()[2..].join("/");
            if content.len() > 0 {
                return Response::new(request, Some("text/plain".to_string()), 200, Some(content));
            }
            else{
                return Response::new(request, None,  404, None);
            }
        }
        else if request.target.starts_with("/files/"){
            let content  = request.target.split("/").collect::<Vec<&str>>()[2..].join("/");
            if content.len() > 0 {
                if request.method == HTTPMethod::POST{
                    match create_file(content, request.body.clone()) {
                        Ok(_)  => {
                            return Response::new(request, None, 201, None);
                        }
                        Err(_) => {
                            return Response::new(request, None, 500, None);
                        }
                    }
                }
                else{
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
        else if request.target == "/hello"{
            return Response::new(request, Some("text/plain".to_string()), 200, Some("Hello World!".to_string()));
        }
        else if request.target == "/user-agent"{
            let user_agent = request.headers
            .iter()
            .find(|(k, _)| k == "User-Agent")
            .map(|(_, v)| v.clone());
    
            return Response::new(request, Some("text/plain".to_string()), 200, Some(user_agent.unwrap().clone()));
        }
        else if request.target != "/" {
            return Response::new(request, None, 404, None);
        }
        else{
            return Response::new(request, None, 200, None);
        }
    }

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