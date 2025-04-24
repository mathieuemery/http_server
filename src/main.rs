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
use threadpool::ThreadPool;

use crate::request::Request;
use crate::response::Response;

fn main() {
    println!("Server started successfully");

    // Max number of threads
    let pool = ThreadPool::new(10);

    // Create a TCP listener on port 4221
    let listener = match TcpListener::bind("127.0.0.1:4221"){
        Ok(l) => l,
        Err(e) => {
            println!("Error binding to port: {}", e);
            return;
        }
    };
    
    // Accept incoming connections and check for errors
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    handler(stream);
                });
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
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return,  // Connection closed
            Ok(b) => b,
            Err(e) => {
                println!("Error reading from stream: {:?}", e);
                return;
            }
        };
    
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
        if let Err(e) = stream.write(&response.as_bytes()){
            println!("Error writing to stream: {:?}", e);
            return;
        }

        if let Err(e) = stream.flush(){
            println!("Error flushing stream: {:?}", e);
            return;
        }

        // Check if the "Connection: close" header is present
        // If it is, close the connection
        if response.headers.contains(&("Connection".to_string(), "close".to_string())){
            println!("Connection closed");
            return;
        }
    }
}