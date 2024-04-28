#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

mod commands;

fn main() {

  // Define and create a system tray
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let toggle_status = CustomMenuItem::new("toggle".to_string(), "Toggle On/Off");
  let tray_menu = SystemTrayMenu::new()
    .add_item(toggle_status)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
  // Commands
    .invoke_handler(tauri::generate_handler![commands::get_open_windows,
                                            commands::get_open_processes,
                                            commands::get_fav_phrases,
                                            commands::get_whitelist,
                                            commands::get_active_status,
                                            commands::set_whitelist_window,
                                            commands::set_whitelist_process,
                                            commands::toggle_app_state])
  // Attach system tray menu
    .system_tray(tray)
    // On left click, we'll unhide the app
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "toggle" => {
            // Todo
          }
          "quit" => {
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();

            if item_handle.get_title().eq("Hide") {
              window.hide().unwrap();
              item_handle.set_title("Show").unwrap();
            } else {
              window.show().unwrap();
              item_handle.set_title("Hide").unwrap();
            }
          }
          _ => {}
        }
      }
      _ => {}
    })
  // Run
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
