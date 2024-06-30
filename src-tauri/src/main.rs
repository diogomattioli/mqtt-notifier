// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent };

fn main() {
    let system_tray = SystemTray::new()
        .with_menu_on_left_click(true)
        .with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("show", "Show"))
                .add_item(CustomMenuItem::new("exit", "Exit"))
        );

    tauri::Builder
        ::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                window.clone().on_window_event(move |e| {
                    match e {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            let _ = window.hide();
                        }
                        _ => {}
                    }
                });
            }

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            if let Some(window) = app.get_window("main") {
                match event {
                    SystemTrayEvent::MenuItemClick { id, .. } => {
                        match id.as_str() {
                            "show" => {
                                let _ = window.show();
                            }
                            "exit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
