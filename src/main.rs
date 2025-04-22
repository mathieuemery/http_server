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

    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
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

fn handler (mut stream: TcpStream){
    loop{
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
    
        if bytes_read == 0 {
            return;
        }
    
        let stream_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    
        let request : Request = match Request::from_str(stream_string){
            Ok(r) => r,
            Err(e) => {
                println!("Error parsing request: {:?}", e);
                return;
            }
        };
    
        let response : Response = Response::parse_request(request);
    
        stream.write(&response.as_bytes()).unwrap();
        stream.flush().unwrap();

        if response.headers.contains(&("Connection".to_string(), "close".to_string())){
            println!("Connection closed");
            return;
        }
    }
}