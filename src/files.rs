//! files.rs
//!
//! Manages file operations such as reading and writing files.

use std::{env, fs::File};
use std::io::Write;

/// Get the content of a file on the server.
///
/// Get the working directory from the command line arguments.
/// If no directory is provided, it defaults to the current directory.
/// The function constructs the full path to the file and reads its content.
///
/// # Arguments
///
/// * `file_path` - Path to the file to be read.
pub fn get_file_content(file_path: String) -> Option<String> {
    // Get the working directory from command line arguments
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 1{
        dir = env_args[2].clone();
    }
    
    // Construct the full path to the file
    let path = std::path::Path::new(&dir).join(file_path);
    
    // Check if the file exists and read its content
    if path.exists() {
        let content = std::fs::read_to_string(path).ok()?;
        Some(content)
    } else {
        None
    }
}

/// Create a file on the server
///
/// Get the working directory from the command line arguments.
/// If no directory is provided, it defaults to the current directory.
/// The function constructs the full path to the file and create a file.
///
/// # Arguments
///
/// * `file_name` - Name of the file to create.
/// * `file_content` - Content to write to the file.
pub fn create_file(file_name: String, file_content: String) -> std::io::Result<()> {
    // Get the working directory from command line arguments
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 1{
        dir = env_args[2].clone();
    }

    // Construct the full path to the file
    let path = std::path::Path::new(&dir).join(&file_name);

    // Write the content to the file
    let mut file = File::create(path)?;
    file.write_all(file_content.as_bytes())?;
    file.flush()?;
    Ok(())
}