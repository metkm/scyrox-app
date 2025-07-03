use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem}, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}, App, Manager, Window, WindowEvent
};
use tauri_plugin_store::StoreExt;

mod commands;
mod device;
mod models;

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        let app_handle = window.app_handle();

        let minimize_to_tray = app_handle.store("config.json")
            .and_then(|store| {
                Ok(
                    store.get("minimize_to_tray")
                        .and_then(|value| value.as_bool())
                        .unwrap_or(false)
                )
            })
            .unwrap_or(false);

        if minimize_to_tray {
            window.hide().unwrap();
            api.prevent_close();
        }
    }
}

pub fn handle_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    let _tray = TrayIconBuilder::with_id("tray_icon_battery")
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            _ => {}
        })
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|icon, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                let app_handle = icon.app_handle();
                let window = app_handle.get_webview_window("main").unwrap();
                window.show().ok();
            };
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(handle_setup)
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
