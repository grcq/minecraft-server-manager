use std::any;

use serde_json::{json, Map, Value};

use crate::utils::fs;
use crate::Error;

#[tauri::command]
pub async fn install_server(path: &str, server_data: Map<String, Value>) -> Result<bool, Error> {
    let server_type = server_data.get("type").unwrap();
    let server_version = server_data.get("version").unwrap();
    let port = server_data.get("port").unwrap();
    let memory = server_data.get("memory").unwrap();

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
    match server_type.as_str().unwrap() {
        "vanilla" => {
            let url = format!("https://launcher.mojang.com/v1/objects/{}.jar", server_version);
            fs::download_file(url, server_jar_path).await;
        }
        "fabric" => {
            let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-server-launcher/{}/fabric-server-launcher-{}.jar", server_version, server_version);
            fs::download_file(url, server_jar_path).await;
        },
        "paper" => {
            let version_exists_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", server_version);
            let version_exists = reqwest::get(version_exists_url).await.unwrap().status().is_success();
            if version_exists {
                let builds_url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", server_version);
                let builds_response = reqwest::get(builds_url).await.unwrap().text();
                let builds = json!(builds_response.await.unwrap());
                let latest_build = builds["builds"][0].as_str().unwrap();

                let url = format!("https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", server_version, latest_build, server_version, latest_build);
                fs::download_file(url, server_jar_path).await;
            } else {
                println!("Invalid Paper version");
                return Ok(false);
            }

        }
        _ => {
            println!("Invalid server type");
            return Ok(false);
        }
    }

    let start_bat = format!("java -Xmx{}M -Xms128M -XX:MaxRAMPercentage=95.0 -Dterminal.jline=false -Dterminal.ansi=true -jar server.jar nogui", memory);
    let start_bat_path = format!("{}/start.bat", path);
    fs::write_file(start_bat_path, start_bat);

    return Ok(true);
}