use hidapi::HidDevice;

use crate::{
    device::{constants::Command, hid::write_eeprom},
    models::error::AppError,
};

pub fn read(
    device: &HidDevice,
    command: Command,
    addr: u16,
    value: &[u8],
    buffer: &mut [u8],
) -> Result<usize, AppError> {
    write_eeprom(device, command, addr, value, value.len() as u8)?;
    let read_count = device.read_timeout(buffer, 50)?;

    Ok(read_count)
}
