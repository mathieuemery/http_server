use std::{env, fs::File};
use std::io::Write;

pub fn get_file_content(file_path: String) -> Option<String> {
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 1{
        dir = env_args[2].clone();
    }
    
    let path = std::path::Path::new(&dir).join(file_path);
    
    if path.exists() {
        let content = std::fs::read_to_string(path).ok()?;
        Some(content)
    } else {
        None
    }
}

pub fn create_file(file_name: String, file_content: String) -> std::io::Result<()> {
    let env_args: Vec<String> = env::args().collect();
    let mut dir = "./".to_string();
    if env_args.len() > 1{
        dir = env_args[2].clone();
    }

    let path = std::path::Path::new(&dir).join(&file_name);

    let mut file = File::create(path)?;
    file.write_all(file_content.as_bytes())?;
    file.flush()?;
    Ok(())
}