use std::sync::Mutex;

use tauri::State;

use crate::models;

#[tauri::command]
pub fn get_minimize_to_tray(state: State<'_, Mutex<models::AppState>>) -> bool {
    let state = state.lock().unwrap();
    state.minimize_to_tray
}

#[tauri::command]
pub fn set_minimize_to_tray(state: State<'_, Mutex<models::AppState>>, value: bool) {
    let mut state = state.lock().unwrap();
    state.minimize_to_tray = value;
}
