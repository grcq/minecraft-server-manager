use serde_json::{json, Map, Value};

use crate::utils::fs;
use crate::Error;

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

use tauri::Manager;
use std::io::{Read, Write};
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::fs::OpenOptions;
use std::path::Path;

type ChildMap = Arc<Mutex<HashMap<String, Child>>>;

lazy_static! {
    static ref RUNNING_SERVERS: ChildMap = Arc::new(Mutex::new(HashMap::new()));
}

pub fn startOutputFetching() -> String {
    return "".to_string();
}

#[tauri::command]
pub async fn send_message_to_service(message: String, read: bool) -> Result<String, Error> {
    let pipe_name = r"\\.\pipe\MSMServicePipe";
    let mut pipe = OpenOptions::new()
        .read(true)
        .write(true)
        .open(pipe_name)
        .expect("Failed to open pipe");
    pipe.write_all(message.as_bytes()).expect("Failed to write to pipe");

    if read {
        let mut buffer = [0; 512];
        let n = pipe.read(&mut buffer).expect("Failed to read from pipe");
        let response = String::from_utf8_lossy(&buffer[..n]);
        return Ok(response.to_string());
    }

    Ok("Success".to_string())
}

#[tauri::command]
pub async fn install_server(path: &str, server_data: Map<String, Value>) -> Result<bool, Error> {
    let server_type = server_data.get("type").unwrap().as_str().unwrap();
    let server_version = server_data.get("version").unwrap().as_str().unwrap();
    let port = server_data.get("port").unwrap().as_str().unwrap();
    let memory = server_data.get("memory").unwrap().as_str().unwrap();

    println!("Installing server at path: {}", path);
    println!("Server type: {}", server_type);
    println!("Server version: {}", server_version);

    fs::create_dir(path);

    let server_properties = format!("server-ip=0.0.0.0\nserver-port={}\nquery.port={}", port, port);
    let server_properties_path = format!("{}/server.properties", path);
    fs::write_file(server_properties_path, server_properties);

    let eula = "eula=true";
    let eula_path = format!("{}/eula.txt", path);
    fs::write_file(eula_path, eula.to_string());

    let server_jar_path = format!("{}/server.jar", path);
    match server_type {
        "Vanilla" => {
            let url = format!("https://launcher.mojang.com/v1/objects/{}.jar", server_version);
            fs::download_file(url, server_jar_path).await?;
        }
        "Fabric" => {
            let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-server-launcher/{}/fabric-server-launcher-{}.jar", server_version, server_version);
            fs::download_file(url, server_jar_path).await?;
        },
        "Paper" => {
            let version_exists_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", server_version);
            let version_exists_t = reqwest::get(version_exists_url).await.unwrap().text().await.unwrap();
            let version_exists: Value = serde_json::from_str(&version_exists_t).unwrap();
            if version_exists["error"].is_null() {
                let builds_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", server_version.to_string());
                let builds_response = reqwest::get(builds_url).await.unwrap().text().await.unwrap();
                let builds_value: Value = serde_json::from_str(&builds_response).unwrap();
                let builds = builds_value["builds"].as_array().unwrap();

                let latest_build = builds.last().unwrap().as_u64().unwrap();

                let url = format!("https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", server_version, latest_build, server_version, latest_build);
                fs::download_file(url, server_jar_path).await.expect("Failed to download Paper jar");
            } else {
                println!("Invalid Paper version");
                return Ok(false)
            }

        }
        _ => {
            println!("Invalid server type");
            return Ok(false)
        }
    }

    let start_bat = format!("java -Xmx{}M -Xms128M -XX:MaxRAMPercentage=95.0 -Dterminal.jline=false -Dterminal.ansi=true -jar server.jar nogui", memory);
    let start_bat_path = format!("{}/start.bat", path);
    fs::write_file(start_bat_path, start_bat);

    return Ok(true);
}

#[tauri::command]
pub async fn get_terminal_output(path: &str) -> Result<String, Error> {
    let output = fs::read_file(format!("{}/output.log", path)).expect("");
    return Ok(output);
}

fn get_id(path: &str) -> &str {
    return path.split("/").collect::<Vec<&str>>().last().unwrap();
}

#[tauri::command]
pub fn is_running(id: &str) -> bool {
    let map = RUNNING_SERVERS.lock().unwrap();
    return map.contains_key(id);
}

#[tauri::command]
pub fn send_command(path: &str, command: &str) -> bool {
    let server_id = get_id(path);
    let mut map = RUNNING_SERVERS.lock().unwrap();
    if let Some(child) = map.get_mut(server_id) {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        //stdin.write_all(command.as_bytes()).expect("Failed to write to stdin");
        //stdin.write_all(b"\n").expect("Failed to write to stdin");
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
pub async fn start_server(path: &str) -> Result<(), Error> {
    let server_id = get_id(path);
    let start_file = format!("{}/start.bat", path);
    let output = format!("{}/output.log", path);
    let output_clone = output.clone();

    if fs::exists(&output.clone()) {
        fs::remove_file(&output.clone());
    }
    fs::create_file_if_not_exists(output.clone(), "".to_string());

    let mut child = Command::new("cmd")
        .args(&["/C", &start_file])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start server");

    if let Some(stdout) = child.stdout.take() {
        let stdout_reader = BufReader::new(stdout);
        let stdout_thread = thread::spawn(move || {
            for line in stdout_reader.lines() {
                match line {
                    Ok(line) => {
                        fs::append_file(output.clone(), format!("\n{}", line));
                    }
                    Err(e) => {
                        println!("Failed to read line: {}", e);
                    }
                
                }
            }
        });
    }

    if let Some(stderr) = child.stderr.take() {
        let stderr_reader = BufReader::new(stderr);
        let stderr_thread = thread::spawn(move || {
            for line in stderr_reader.lines() {
                match line {
                    Ok(line) => {
                        fs::append_file(output_clone.clone(), format!("\n{}", line));
                    }
                    Err(e) => {
                        println!("Failed to read line: {}", e);
                    }
                
                }
            }
        });
    }

    let mut map = RUNNING_SERVERS.lock().unwrap();
    map.insert(server_id.to_string(), child);
    
    Ok(())
}

#[tauri::command]
pub async fn stop_server(path: &str) -> Result<(), Error> {
    fs::remove_file(&format!("{}/output.log", path));
    send_command(path, "stop");

    let mut map = RUNNING_SERVERS.lock().unwrap();
    let server_id = get_id(path);
    if let Some(mut child) = map.remove(server_id) {
        let pid = child.id();
        match child.kill() {
            Ok(_) => {
                println!("Server stopped");
                match child.wait() {
                    Ok(_) => {
                        println!("Server process finished");
                    }
                    Err(e) => {
                        println!("Failed to wait for server process: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to stop server: {}", e);
            }
        }

        thread::sleep(std::time::Duration::from_secs(5));

        if is_process_running(pid) {
            println!("Server process still running");
            if cfg!(windows) {
                let _ = Command::new("taskkill")
                    .args(&["/PID", &pid.to_string(), "/F"])
                    .output()
                    .expect("Failed to kill process");
            } else {
                let _ = Command::new("kill")
                    .args(&["-9", &pid.to_string()])
                    .output()
                    .expect("Failed to kill process");
            }

            if (is_process_running(pid)) {
                println!("Failed to kill server process");
            } else {
                println!("Server process killed");
            }
        } else {
            println!("Server process not running");
        }
    } else {
        println!("Server not running");
    }

    Ok(())
}

fn is_process_running(pid: u32) -> bool {
    if cfg!(windows) {
        let output = Command::new(format!("netstat -ano | findstr \"{}\"", pid))
            .output()
            .expect("Failed to check process status");
        let output_str = String::from_utf8_lossy(&output.stdout);
        output_str.contains("LISTENING")
    } else {
        let output = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
            .expect("Failed to check process status");
        let output_str = String::from_utf8_lossy(&output.stdout);
        output_str.lines().count() > 1
    }
}

#[tauri::command]
pub fn get_server_status(path: &str) -> String {
    let server_id = get_id(path);
    let map = RUNNING_SERVERS.lock().unwrap();
    if let Some(child) = map.get(server_id) {
        return "Running".to_string();
    } else {
        return "Offline".to_string();
    }
}

#[tauri::command]
pub fn upload_files(path: &str) {
}