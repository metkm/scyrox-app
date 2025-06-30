use std::sync::Mutex;

use tauri::State;

use crate::device::hid::write_eeprom;
use crate::device::constants::{Command, MouseEepromAddr};
use crate::models;

#[tauri::command]
pub fn set_current_dpi_index(state: State<'_, Mutex<models::AppState>>, index: u8) {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return;
    };

    println!("{:?} - {:?}", index, 0x55 - index);

    match write_eeprom(
        device,
        Command::WriteFlashData,
        MouseEepromAddr::CurrentDPI,
        &[index, 0x55 - index],
        2
    ) {
        Ok(yea) => {
            println!("succsess {:?}", yea)
        }
        Err(err) => {
            println!("{:?}", err)
        }
    };

    // device::hid::write_eeprom(device, command, address, value, length)
}
