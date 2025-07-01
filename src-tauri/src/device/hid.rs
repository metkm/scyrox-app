use hidapi::{HidDevice, HidResult};

use crate::device::{constants::{Command, MouseEepromAddr, REPORT_ID}, utils::get_usb_crc};


pub fn write_eeprom(
    device: &HidDevice,
    command: Command,
    address: MouseEepromAddr,
    value: &[u8],
    length: u8
) -> HidResult<usize> {
    let address_bytes = u16::from(address).to_be_bytes();

    let mut buffer: [u8; 17] = [
        0x08,
        command.into(),
        0x00,
        *address_bytes.first().unwrap_or(&0),
        *address_bytes.first().unwrap_or(&0),
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
        0xEF
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

    device.write(&buffer)
}
