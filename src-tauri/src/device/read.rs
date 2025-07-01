use hidapi::HidDevice;

use crate::device::{constants::Command, hid::write_eeprom};

pub fn read_full_eeprom(device: &HidDevice) -> Result<(), String> {
    let full_buffer: [u8; 255] = [0x00; 255];
    let mut chunk_buffer: [u8; 16] = [0x00; 16];

    let mut addr = 0;

    while addr < 0x100 {
        if let Err(err) = write_eeprom(device, Command::ReadFlashData, addr, &[], 0x10) {
            return Err(err.to_string())
        }

        match device.read_timeout(&mut chunk_buffer, 50) {
            Ok(read_size) => {
                println!("read size {:?}", read_size)
            },
            Err(err) => {
                return Err(err.to_string())
            }
        }
        
        addr += 10;
    }

    Ok(())
}
