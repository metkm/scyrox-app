use crate::device::utils::voltage_to_level;

#[derive(serde::Serialize)]
pub struct Battery {
    pub charging: bool,
    pub level: u8,
}

impl Battery {
    pub fn from_buffer(buffer: &[u8]) -> Battery {
        let voltage =
            i16::from_be_bytes([*buffer.get(8).unwrap_or(&0), *buffer.get(9).unwrap_or(&0)]);
        let level = voltage_to_level(voltage);

        Battery {
            charging: buffer.get(7).unwrap_or(&0) == &1,
            level,
        }
    }
}

