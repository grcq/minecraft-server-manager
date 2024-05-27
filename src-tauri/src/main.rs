#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod utils;

use std::sync::Mutex;
use std::path::Path;
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenuItem, SystemTrayMenu};

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}


fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit MSM");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide MSM");
  let show = CustomMenuItem::new("show".to_string(), "Show MSM");

  let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);
  let tray = SystemTray::new().with_menu(tray_menu);
  tauri::Builder::default()
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          app.exit(0);
        }
        "hide" => {
          let window = app.get_window("main").unwrap();
          window.hide().unwrap();
        }
        "show" => {
          let window = app.get_window("main").unwrap();
          window.show().unwrap();
        }
        _ => {}
      },
      _ => {}
    })
    .setup(|app| {
      let window = app.get_window(&"main").unwrap();
      window.show().unwrap();

      let client = DeclarativeDiscordIpcClient::new("722830000348463197");
      client.enable();
      app.manage(client);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      utils::fs::read_file,
      utils::fs::write_file,
      utils::fs::read_dir,
      utils::fs::create_dir,
      utils::fs::remove_dir,
      utils::fs::remove_file,
      utils::fs::rename_file,
      utils::fs::copy_file,
      utils::fs::exists,
      utils::fs::create_dir_if_not_exists,
      utils::fs::create_file_if_not_exists,
      utils::server::install_server,
      utils::server::get_terminal_output,
      utils::server::start_server,
      utils::server::stop_server,
      utils::server::send_command,
      utils::server::is_running,
      utils::discord_rpc::set_rpc
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}