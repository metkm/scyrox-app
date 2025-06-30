use std::sync::Mutex;

use hidapi::HidDevice;
use tauri::Manager;

mod device;
mod commands;
mod models;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::set_current_dpi_index])
        .setup(|app| {
            let device = device::get_device();

            app.manage(
                Mutex::new(
                    models::AppState {
                        device
                    }
                )
            );

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;

                let window = app.get_webview_window("main").unwrap();
                // window.open_devtools();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
