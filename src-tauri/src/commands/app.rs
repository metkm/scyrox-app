use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn get_minimize_to_tray(
    app_handle: tauri::AppHandle,
) -> bool {
    app_handle.store("config.json")
        .map(|store| {
            store.get("minimize_to_tray")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

#[tauri::command]
pub fn set_minimize_to_tray(
    // state: State<'_, Mutex<models::AppState>>,
    app_handle: tauri::AppHandle,
    value: bool,
) {
    if let Ok(store) = app_handle.store("config.json") {
        store.set("minimize_to_tray", value);
    }
}
