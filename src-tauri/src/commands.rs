use std::sync::Mutex;

use tauri::State;

use crate::device;
use crate::device::hid::write_eeprom;
use crate::device::constants::{Command, MouseEepromAddr};
use crate::device::utils::voltage_to_level;
use crate::models::{self, AppError, MouseConfig};

#[tauri::command]
pub fn set_current_dpi_index(state: State<'_, Mutex<models::AppState>>, index: u8) -> Result<usize, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound)
    };

    write_eeprom(
        device,
        Command::WriteFlashData,
        MouseEepromAddr::CurrentDPI.into(),
        &[index, 0x55 - index],
        2
    )
}

#[tauri::command]
pub fn read_mouse_config(state: State<'_, Mutex<models::AppState>>) -> Result<MouseConfig, String> {
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
            Ok(_) => {
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
            },
            Err(err) => {
                return Err(err.to_string())
            }
        }
        
        addr += 10;
    }

    Ok(MouseConfig::from_slice(&full_buffer))
}



#[tauri::command]
pub fn get_mouse_battery(state: State<'_, Mutex<models::AppState>>) -> Result<u8, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound)
    };

    let mut buffer = [0_u8; 10];
    device::read::read(device, Command::BatteryLevel, 0x00, &[], &mut buffer)?;

    let voltage = i16::from_be_bytes([
        *buffer.get(8).unwrap_or(&0),
        *buffer.get(9).unwrap_or(&0)
    ]);

    Ok(voltage_to_level(voltage))
}
