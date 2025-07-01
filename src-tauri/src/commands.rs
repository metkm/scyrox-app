use std::sync::Mutex;

use tauri::State;

use crate::device::hid::write_eeprom;
use crate::device::constants::{Command, MouseEepromAddr};
use crate::models;

#[tauri::command]
pub fn set_current_dpi_index(state: State<'_, Mutex<models::AppState>>, index: u8) -> Result<usize, String> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err("device is not connected".to_string());
    };

    write_eeprom(
        device,
        Command::WriteFlashData,
        MouseEepromAddr::CurrentDPI.into(),
        &[index, 0x55 - index],
        2
    )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_full_eeprom(state: State<'_, Mutex<models::AppState>>) -> Result<(), String> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err("device is not connected".to_string());
    };

    let mut full_buffer: [u8; 255] = [0x00; 255];
    let mut chunk_buffer: [u8; 17] = [0x00; 17];

    let mut addr = 0;

    while addr < 0x100 {
        if let Err(err) = write_eeprom(device, Command::ReadFlashData, addr, &[], 10) {
            return Err(err.to_string())
        }

        match device.read_timeout(&mut chunk_buffer, 50) {
            Ok(read_size) => {
                let buff_without_report_id = &chunk_buffer[1..];

                for i in 0..10 {
                    let Some(source) = buff_without_report_id.get(5 + i) else {
                        continue;
                    };

                    let Some(target) = full_buffer.get_mut(addr as usize + i) else {
                        continue;
                    };

                    *target = *source;
                }

                println!("read size {:?} - {:?}", read_size, &buff_without_report_id)
            },
            Err(err) => {
                return Err(err.to_string())
            }
        }
        
        addr += 10;
    }

    println!("{:?}", full_buffer);

    Ok(())
}
