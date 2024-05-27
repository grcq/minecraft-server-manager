use serde_json::{json, Map, Value};

use crate::utils::fs;
use crate::Error;

use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};
use tokio::sync::mpsc;
use std::process::Stdio;
use std::collections::HashMap;
use once_cell::sync::Lazy;

static mut RUNNING_SERVERS: Lazy<HashMap<String, Child>> = Lazy::new(|| HashMap::new());

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
            fs::download_file(url, server_jar_path).await;
        }
        "Fabric" => {
            let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-server-launcher/{}/fabric-server-launcher-{}.jar", server_version, server_version);
            fs::download_file(url, server_jar_path).await;
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
    unsafe {
        return RUNNING_SERVERS.contains_key(id);
    }
}

#[tauri::command]
pub fn send_command(path: &str, command: &str) -> bool {
    let serverId = get_id(path);
    let child = unsafe {
        RUNNING_SERVERS.get_mut(serverId).unwrap()
    };

    let stdin = child.stdin.as_mut().expect("Failed to get stdin");
    let _ = stdin.write_all(command.as_bytes());
    let _ = stdin.write_all(b"\n");

    true
}

#[tauri::command]
pub async fn start_server(path: &str) -> Result<(), Error> {
    let server_id = get_id(path);
    let start_file = format!("{}/start.bat", path);
    let output = format!("{}/output.log", path);

    if fs::exists(&output.clone()) {
        fs::remove_file(&output.clone());
    }
    fs::create_file_if_not_exists(output.clone(), "".to_string());

    let mut child = Command::new("cmd")
        .current_dir(path)
        .arg("/C")
        .arg(&start_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server");
    println!("Started server: {}", server_id);

    let (tx, mut rx) = mpsc::channel(100);

    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    unsafe {
        RUNNING_SERVERS.insert(server_id.to_string(), child);
    }
    
    tokio::spawn(async move {
        loop {
            tokio::select! {
                line = stdout_reader.next_line() => {
                    match line {
                        Ok(line) => {
                            tx.send(line).await.unwrap();
                        }
                        Err(line) => {
                            println!("Error: {}", line);
                        }
                    }
                }
                line = stderr_reader.next_line() => {
                    match line {
                        Ok(line) => {
                            tx.send(line).await.unwrap();
                        }
                        Err(line) => {
                            println!("Error: {}", line);
                        }
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Some(line) = rx.recv().await {
            let previous_output = fs::read_file(output.clone()).expect("");
            fs::write_file(output.clone(), format!("{}\n{}", previous_output, line.unwrap_or_default()));
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_server(path: &str) -> Result<(), Error> {
    fs::remove_file(&format!("{}/output.log", path));
    send_command(path, "stop");

    let server_id = get_id(path);
    let child = unsafe {
        RUNNING_SERVERS.get_mut(server_id).unwrap()
    };

    child.kill().await.expect("Failed to kill server");
    unsafe {
        RUNNING_SERVERS.remove(server_id);
    }

    Ok(())
}