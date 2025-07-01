use hidapi::HidDevice;

use crate::{
    device::{
        constants::{Command, REPORT_ID},
        utils::get_usb_crc,
    },
    models::AppError,
};

pub fn write_eeprom(
    device: &HidDevice,
    command: Command,
    address: u16,
    value: &[u8],
    length: u8,
) -> Result<usize, AppError> {
    let address_bytes = address.to_be_bytes();

    let mut buffer: [u8; 17] = [
        0x08,
        command.into(),
        0x00,
        *address_bytes.first().unwrap_or(&0),
        *address_bytes.get(1).unwrap_or(&0),
        length,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xEF,
    ];

    for (i, val) in value.iter().enumerate() {
        let Some(buff_val) = buffer.get_mut(i + 6) else {
            continue;
        };

        *buff_val = *val;
    }

    let crc = get_usb_crc(&buffer).wrapping_sub(REPORT_ID);

    if let Some(val) = buffer.get_mut(16) {
        *val = crc;
    };

    let written_count= device.write(&buffer)?;

    Ok(written_count)
}
