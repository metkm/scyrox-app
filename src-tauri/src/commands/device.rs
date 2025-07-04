use std::sync::Mutex;

use tauri::State;

use crate::device;
use crate::device::constants::{Command, MouseEepromAddr};
use crate::device::hid::write_eeprom;
use crate::device::utils::get_usb_crc;
use crate::models::{self, AppError, Battery, MouseConfig};

#[tauri::command]
pub fn set_current_dpi_index(
    state: State<'_, Mutex<models::AppState>>,
    index: usize,
) -> Result<usize, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    write_eeprom(
        device,
        Command::WriteFlashData,
        MouseEepromAddr::CurrentDPI.into(),
        &[index as u8, 0x55 - index as u8],
        2,
    )
}

#[tauri::command]
pub fn get_current_dpi_index(
    state: State<'_, Mutex<models::AppState>>,
) -> Result<u8, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let mut buffer = [0x00; 16];
    device::read::read(device, Command::ReadFlashData, MouseEepromAddr::CurrentDPI.into(), &[0x00, 0x00], &mut buffer)?;
    
    Ok(*buffer.get(6).unwrap_or(&1))
}

#[tauri::command]
pub fn read_mouse_config(
    state: State<'_, Mutex<models::AppState>>,
) -> Result<MouseConfig, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let mut full_buffer: [u8; 255] = [0x00; 255];
    let mut chunk_buffer: [u8; 17] = [0x00; 17];

    let mut addr = 0;

    while addr < 0x100 {
        write_eeprom(device, Command::ReadFlashData, addr, &[], 10)?;

        device.read_timeout(&mut chunk_buffer, 50)?;
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

        addr += 10;
    }

    Ok(MouseConfig::from_slice(&full_buffer))
}

#[tauri::command]
pub fn get_mouse_battery(
    state: State<'_, Mutex<models::AppState>>,
    app_handle: tauri::AppHandle,
) -> Result<Battery, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let mut buffer = [0_u8; 10];
    device::read::read(device, Command::BatteryLevel, 0x00, &[], &mut buffer)?;

    let battery: Battery = Battery::from_buffer(&buffer);

    if let Some(tray_icon) = app_handle.tray_by_id("tray_icon_battery") {
        tray_icon
            .set_tooltip(Some(format!("{:?}%", battery.level)))
            .inspect_err(|err| eprintln!("{err:?}"))
            .ok();
    }

    Ok(battery)
}

#[tauri::command]
pub fn get_dongle_version(state: State<'_, Mutex<models::AppState>>) -> Result<String, AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let mut buffer = [0_u8; 10];
    device::read::read(device, Command::GetDongleVersion, 0x00, &[], &mut buffer)?;

    let version_string = format!(
        "{}.{:02X}",
        buffer.get(6).unwrap_or(&0),
        buffer.get(7).unwrap_or(&0)
    );

    Ok(version_string)
}

#[tauri::command]
pub fn update_dpi_value(
    state: State<'_, Mutex<models::AppState>>,
    index: u8,
    value: usize,
) -> Result<(), AppError> {
    if value < 10 {
        return Err(AppError::InvalidValue);
    }

    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let low = ((value / 50 - 1) & 0xFF) as u8;
    let high = (((value / 50 - 1) >> 8) & 0xFF) as u8;

    let mut buffer = [low, low, (high << 2) | (high << 6), 0x00];

    let crc = get_usb_crc(&buffer);

    let Some(val) = buffer.get_mut(3) else {
        return Err(AppError::CrcProblem);
    };

    *val = crc;
    write_eeprom(
        device,
        Command::WriteFlashData,
        MouseEepromAddr::DPIValue as u16 + index as u16 * 4,
        &buffer,
        buffer.len() as u8,
    )?;

    Ok(())
}

#[tauri::command]
pub fn set_key(
    state: State<'_, Mutex<models::AppState>>,
    index: u16,
    value: [u8; 3],
) -> Result<(), AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let addr = MouseEepromAddr::KeyFunction as u16 + index * 4;
    let mut buffer = [
        *value.first().unwrap_or(&0),
        *value.get(1).unwrap_or(&0),
        *value.get(2).unwrap_or(&0),
        0x00,
    ];

    let crc = get_usb_crc(&buffer);
    if let Some(last_val) = buffer.last_mut() {
        *last_val = crc;
    }

    write_eeprom(
        device,
        Command::WriteFlashData,
        addr,
        &buffer,
        buffer.len() as u8,
    )?;

    Ok(())
}

#[tauri::command]
pub fn set_key_multimedia(
    state: State<'_, Mutex<models::AppState>>,
    index: u16,
    value: u16,
) -> Result<(), AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    let addr = MouseEepromAddr::ShortcutKey as u16 + index * 0x20;
    let mut buffer: [u8; 8] = [
        0x02,
        0x82,
        (value & 0xFF) as u8,
        (value >> 8) as u8,
        0x42,
        (value & 0xFF) as u8,
        (value >> 8) as u8,
        0x00,
    ];

    let crc = get_usb_crc(&buffer);
    if let Some(val) = buffer.last_mut() {
        *val = crc;
    }

    write_eeprom(
        device,
        Command::WriteFlashData,
        addr,
        &buffer,
        buffer.len() as u8,
    )?;

    Ok(())
}

#[tauri::command]
pub fn set_performance_time(
    state: State<'_, Mutex<models::AppState>>,
    value: u8
) -> Result<(), AppError> {
    let state = state.lock().unwrap();

    let Some(device) = &state.device else {
        return Err(AppError::DeviceNotFound);
    };

    write_eeprom(device, Command::WriteFlashData, MouseEepromAddr::PerformanceTime as u16, &[value, 0x55 - value], 2)?;

    Ok(())
}
