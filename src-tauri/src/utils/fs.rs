use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ChildStdout;
use std::path::Path;

use crate::Error;

#[tauri::command]
pub fn read_file(path: String) -> Result<String, Error> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    Ok(content)
}

#[tauri::command]
pub fn write_file(path: String, content: String) {
    fs::write(path, content).expect("Failed to write file");
}

#[tauri::command]
pub async fn read_dir(path: String) -> Result<Vec<String>, Error> {
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

#[tauri::command]
pub fn remove_dir(path: &str) {
    fs::remove_dir(path).expect("Failed to remove directory");
}

#[tauri::command]
pub fn remove_file(path: &str) {
    fs::remove_file(path).expect("Failed to remove file");
}

#[tauri::command]
pub fn rename_file(old_path: String, new_path: String) {
    fs::rename(old_path, new_path).expect("Failed to rename file");
}

#[tauri::command]
pub fn copy_file(src: &str, dest: &str) {
    fs::copy(src, dest).expect("Failed to copy file");
}

#[tauri::command]
pub fn exists(src: &str) -> bool {
    Path::new(&src).exists()
}

#[tauri::command]
pub fn create_dir_if_not_exists(path: String) {
    if !Path::new(&path).exists() {
        create_dir(&path);
    }
}

#[tauri::command]
pub fn create_file_if_not_exists(path: String, content: String) {
    if !Path::new(&path).exists() {
        write_file(path.to_string(), content.to_string());
    }
}

pub async fn download_file(url: String, path: String) -> Result<(), Error>{
    let response = reqwest::get(&url).await.expect("Failed to send request");
    let bytes = response.bytes().await.expect("Failed to get bytes");
    fs::write(path, bytes).expect("Failed to write file");
    Ok(())
}

pub fn read_line(stdout: ChildStdout) -> String {
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line
}

pub fn append_file(path: String, content: String) {
    let previous_content = read_file(path.clone()).expect("Failed to read file");
    fs::write(path, format!("{}{}", previous_content, content)).expect("Failed to append file");
}