//! files.rs
//!
//! Manages file operations such as reading and writing files.

use std::{env, fs::File, path::Path};
use std::io::Write;

fn sanitize_path(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);

    // Check if the path is absolute
    if path.is_absolute() {
        eprintln!("Path traversal attempt: absolute path not allowed");
        return None;
    }

    // Check for ".." components
    if path.components().any(|c| match c {
        std::path::Component::ParentDir => true,
        _ => false
    }) {
        eprintln!("Path traversal attempt: parent directory references not allowed");
        return None;
    }

    // Normalize the path by ensuring it doesn't contain special characters
    let valid_chars = path.to_string_lossy()
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '/');
    
    if !valid_chars {
        eprintln!("Path traversal attempt: invalid characters in path");
        return None;
    }
    
    Some(file_path.to_string())
}

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
    // Sanitize the file path to prevent path traversal attacks
    let sanitized_path = match sanitize_path(&file_path) {
        Some(path) => path,
        None => return None,
    };

    // Get the working directory from command line arguments
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 2{
        dir = env_args[2].clone();
    }
    
    // Construct the full path to the file
    let base_dir = Path::new(&dir).canonicalize().ok()?;
    let full_path = base_dir.join(&sanitized_path);

    // Check if the canonical path is within the base directory
    let canonical_path = match full_path.canonicalize() {
        Ok(p) => p,
        Err(_) => return None, // File doesn't exist
    };

    if !canonical_path.starts_with(&base_dir) {
        eprintln!("Path traversal attempt: path escapes base directory");
        return None;
    }
    
    // Check if the file exists and read its content
    if canonical_path.exists() {
        let content = std::fs::read_to_string(canonical_path).ok()?;
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
    // Sanitize the path
    let sanitized_path = match sanitize_path(&file_name){
        Some(path) => path,
         None => return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid file path"
         ))
    };
    
    // Get the working directory from command line arguments
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 2{
        dir = env_args[2].clone();
    }

    // Construct the full path to the file
    let base_dir = Path::new(&dir).canonicalize().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::NotFound, 
            format!("Base directory error: {}", e))
    })?;

    let file_path = base_dir.join(&sanitized_path);

    // Ensure the parent directory exists and is within the base directory
    if let Some(parent) = file_path.parent() {
        // Create parent directories if they don't exist
        std::fs::create_dir_all(parent)?;
        
        // Get the canonical path of the parent directory
        let canonical_parent = parent.canonicalize().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Cannot canonicalize path: {}", e))
        })?;
        
        // Check if the parent directory is within the base directory
        if !canonical_parent.starts_with(&base_dir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Path traversal attempt detected"
            ));
        }
    }

    // Write the content to the file
    let mut file = File::create(&file_path)?;
    file.write_all(file_content.as_bytes())?;
    file.flush()?;
    Ok(())
}