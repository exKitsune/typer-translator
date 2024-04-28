#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::dialog::ask;
use tauri::{SystemTray, SystemTrayEvent, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, RunEvent, WindowEvent};
use tauri::Manager;

mod commands;

fn main() {

    // Define and create a system tray
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Show/Hide");
    let toggle_status = CustomMenuItem::new("toggle".to_string(), "Toggle On/Off");
    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_status)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let builder = tauri::Builder::default()
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
                    window.set_focus().unwrap();

                    // let new_tray_menu = SystemTrayMenu::new()
                    //     .add_item(CustomMenuItem::new("toggle".to_string(), "Toggle On/Off"))
                    //     .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
                    //     .add_native_item(SystemTrayMenuItem::Separator)
                    //     .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
                    // let tray_h = app.tray_handle();
                    // tray_h.set_menu(new_tray_menu).unwrap();
                    
                }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "toggle" => {
                        // Todo
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            // "Show"
                        } else {
                            window.show().unwrap();
                            // "Hide"
                        };
                        // item_handle.set_title(new_title).unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        });
    
    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[allow(unused_variables)]
    app.run(|app_handle, e| match e {    
        // Triggered when a window is trying to close
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            // for other windows, we handle it in JS
            if label == "main" {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window(&label).unwrap();
                // use the exposed close api, and prevent the event loop to close
                api.prevent_close();
                // ask the user if he wants to quit
                ask(
                Some(&window),
                "Typer Translator",
                "Are you sure that you want to close this window?",
                move |answer| {
                    if answer {
                    // .close() cannot be called on the main thread
                    std::thread::spawn(move || {
                        app_handle.get_window(&label).unwrap().close().unwrap();
                    });
                    }
                },
                );
            }
        }
    
        // Keep the event loop running even if all windows are closed
        // This allow us to catch system tray events when there is no window
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    })
}
