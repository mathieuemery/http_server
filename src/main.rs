//! main.rs
//!
//! Entry point of the HTTP server.
//! This file sets up the TCP listener and handles incoming connections.
//!
//! Modules:
//! - `files`: Handles file reading/writing.
//! - `request`: Parses incoming HTTP requests into `Request` objects.
//! - `response`: Generates `Response` objects based on the request.
//! - `utils`: Contains shared constants, helpers, and compression logic.

mod files;
mod request;
mod response;
mod utils;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};

use crate::request::Request;
use crate::response::Response;

fn main() {
    println!("Server started successfully");

    // Create a TCP listener on port 4221
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    // Accept incoming connections and check for errors
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handler(stream));
            }                            
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

/// Handles a single client connection.
///
/// Reads the incoming HTTP request, parses it, generates a response,
/// and writes it back to the stream. If the "Connection: close" header
/// is present, the function will close the connection.
///
/// # Arguments
///
/// * `stream` - A TCP stream representing the client's connection.
fn handler (mut stream: TcpStream){
    // Loop to handle multiple requests from the same client
    loop{
        // Read the incoming request
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
    
        if bytes_read == 0 {
            return;
        }
    
        let stream_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    
        // Parse the stream string into a Request object
        let request : Request = match Request::from_str(stream_string){
            Ok(r) => r,
            Err(e) => {
                println!("Error parsing request: {:?}", e);
                return;
            }
        };
    
        // Generate the response based on the request
        let response : Response = Response::parse_request(request);
    
        // Write the response back to the stream
        stream.write(&response.as_bytes()).unwrap();
        stream.flush().unwrap();

        // Check if the "Connection: close" header is present
        // If it is, close the connection
        if response.headers.contains(&("Connection".to_string(), "close".to_string())){
            println!("Connection closed");
            return;
        }
    }
}