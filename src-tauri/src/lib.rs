use std::sync::Mutex;

use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder, Manager, WindowEvent};

mod commands;
mod device;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::device::set_current_dpi_index,
            commands::device::read_mouse_config,
            commands::device::get_mouse_battery,
            commands::device::get_dongle_version,
            commands::device::update_dpi_value,
            commands::device::set_key,
            commands::device::set_key_multimedia,
            commands::app::set_minimize_to_tray,
            commands::app::get_minimize_to_tray
        ])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i]).unwrap();

            let _tray = TrayIconBuilder::new()
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => { app.exit(0) }
                    _ => {}
                })
                .menu(&menu)
                .build(app)?;

            let device = device::get_device();
            let state = Mutex::new(models::AppState {
                device,
                ..Default::default()
            });

            app.manage(state);

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;

                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app_handle = window.app_handle();
                let app_state = app_handle.state::<Mutex<models::AppState>>();
                let state = app_state.lock().unwrap();

                if state.minimize_to_tray {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
