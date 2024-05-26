use std::fs;
use std::path::Path;

use crate::Error;

#[tauri::command]
pub fn read_file(path: &str) -> Result<String, Error> {
    Ok(fs::read_to_string(path).unwrap());
}

#[tauri::command]
pub fn write_file(path: &str, content: &str) {
    fs::write(path, content).expect("Failed to write file");
}

#[tauri::command]
pub async fn read_dir(path: &str) -> Result<Vec<String>, Error> {
    let mut files = vec![];
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        let path = path.to_str().expect("Failed to convert path to string");
        files.push(path.to_string());
    }
    Ok(files)
}

#[tauri::command]
pub fn create_dir(path: &str) {
    fs::create_dir(path).expect("Failed to create directory");
}