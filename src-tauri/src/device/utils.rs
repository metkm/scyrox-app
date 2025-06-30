// starts from 1 because the first byte is reportId

pub fn get_usb_crc(buffer: &[u8]) -> u8 {
    let mut crc: i32 = 
        buffer[1..buffer.len() - 1]
        .iter()
        .fold(0, |acc, e| acc + *e as i32);

    crc = crc & 0xFF;
    crc = 0x55 - crc;

    crc as u8
}
