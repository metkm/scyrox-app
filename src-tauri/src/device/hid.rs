use hidapi::{HidDevice, HidResult};

use crate::device::{constants::{Command, MouseEepromAddr, REPORT_ID}, utils::get_usb_crc};

//   const data = Uint8Array.of(
//     command,
//     0x00,
//     address >> 8,
//     address & 0xff,
//     length,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0x00,
//     0xef,
//   )

//   for (let index = 0; index < value.length; index++) {
//     data[5 + index] = value[index] || 0
//   }

//   data[15] = getCrc(data) - reportId
//   await device.sendReport(reportId, data)

pub fn write_eeprom(
    device: &HidDevice,
    command: Command,
    address: MouseEepromAddr,
    value: &[u8],
    length: u8
) -> HidResult<usize> {
    let address_bytes = u16::from(address).to_be_bytes();

    // let low: u8 = address.into() >> 8;
    // let high: u8 = address.into() & 0xFF;

    let mut buffer: [u8; 17] = [
        0x08,
        command.into(),
        0x00,
        *address_bytes.get(0).unwrap_or(&0),
        *address_bytes.get(1).unwrap_or(&0),
        // low.try_into().unwrap_or(0x00),
        // high.try_into().unwrap_or(0x00),
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

    for i in 0..value.len() {
        let Some(val) = buffer.get_mut(i + 6) else {
            continue;
        };

        *val = value[i];
    }

    let crc = get_usb_crc(&buffer).wrapping_sub(REPORT_ID);

    if let Some(val) = buffer.get_mut(16) {
        *val = crc;
    };

    println!("{:?}", buffer);
    device.write(&buffer)
}
