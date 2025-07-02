use hidapi::HidDevice;
use thiserror::Error;

use crate::device::{constants::MouseEepromAddr, utils::buffer_to_hex};

pub struct AppState {
    pub device: Option<HidDevice>
}

#[derive(serde::Serialize)]
pub struct DpiValue {
    value: u32,
    color: String
}

impl DpiValue {
    pub fn bytes_to_value(bytes: &[u8; 4]) -> i32 {
        let low_bits = bytes.first().unwrap_or(&0);
        let high_bits = bytes.get(2).unwrap_or(&0);

        let masked = *high_bits as i32 * MouseEepromAddr::DPIValue as i32;
        let high = masked >> 2;

        let mut value = *low_bits as i32 + (high << 8);
        value = (value + 1) * 50;

        value
    }
}

#[derive(serde::Serialize)]
pub struct MouseConfig {
    // version: String,
    report_rate: u16,
    dpi_values: Vec<DpiValue>,
    current_dpi_index: u8,
    max_dpi_index: u8,
}

impl MouseConfig {
    pub fn from_slice(buffer: &[u8]) -> Self {
        MouseConfig {
            report_rate: 'report_rate: {
                let Some(val) = buffer.first() else {
                    break 'report_rate 0;
                };

                if *val == 0 {
                    break 'report_rate 0;
                }

                if *val > 0x10 {
                    (*val / 0x10) as u16 * 2000
                } else {
                    1000 / *val as u16
                }
            },
            dpi_values: {
                let mut values: Vec<DpiValue> = Vec::with_capacity(8);

                for i in 0..8 {
                    let dpi_base_addr = i * 4 + MouseEepromAddr::DPIValue as usize;

                    let bytes: &[u8; 4] = &buffer[dpi_base_addr..dpi_base_addr + 4].try_into().unwrap();
                    
                    let color_base_add = dpi_base_addr + MouseEepromAddr::DPIColor as usize;
                    let color_bytes: &[u8; 3] = &buffer[color_base_add..(color_base_add + 3)].try_into().unwrap();

                    values.push(DpiValue {
                        value: DpiValue::bytes_to_value(bytes) as u32,
                        color: buffer_to_hex(color_bytes)
                    });
                }

                values
            },
            current_dpi_index: *buffer.get(MouseEepromAddr::CurrentDPI as usize).unwrap_or(&0),
            max_dpi_index: *buffer.get(MouseEepromAddr::MaxDPI as usize).unwrap_or(&0)
        }
    }
}

#[derive(serde::Serialize)]
pub struct Battery {
    pub charging: bool,
    pub level: u8,
}


#[derive(Error, Debug)]
pub enum AppError {
    #[error("device not found")]
    DeviceNotFound,
    #[error("hid error")]
    HidError(#[from] hidapi::HidError),
    #[error("invalid value")]
    InvalidValue,
    #[error("crc problem")]
    CrcProblem
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
