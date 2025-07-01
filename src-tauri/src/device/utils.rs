// starts from 1 because the first byte is reportId

pub fn get_usb_crc(buffer: &[u8]) -> u8 {
    let mut crc: i32 = 
        buffer[1..buffer.len() - 1]
        .iter()
        .fold(0, |acc, e| acc + *e as i32);

    crc &= 0xFF;
    crc = 0x55 - crc;

    crc as u8
}

const VOLTAGES: [i16; 21] = [
  3050, 3420, 3480, 3540, 3600, 3660, 3720, 3760, 3800, 3840, 3880, 3920, 3940, 3960, 3980, 4000,
  4020, 4040, 4060, 4080, 4110,
];

pub fn voltage_to_level(voltage: i16) -> u8 {
    let Some(voltage_index) = VOLTAGES
        .iter()
        .position(|rvolt| *rvolt > voltage) else {
            return 0;
        };

    let Some(rounded_voltage) = VOLTAGES.get(voltage_index) else {
        return 0;
    };

    let Some(rounded_voltage_down) = VOLTAGES.get(voltage_index - 1) else {
        return 0;
    };

    let interval = (rounded_voltage - rounded_voltage_down) / 5;
    let level = (voltage - *rounded_voltage_down) / interval + (voltage_index as i16 - 1) * 5;

    level.try_into().unwrap_or(0)
}
