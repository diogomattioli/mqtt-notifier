// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu };

fn main() {
    let system_tray = SystemTray::new()
        .with_menu_on_left_click(true)
        .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("exit", "Exit")));

    tauri::Builder
        ::default()
        .system_tray(system_tray)
        .on_system_tray_event(|_, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "exit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
