use hidapi::HidDevice;
use thiserror::Error;

pub struct AppState {
    pub device: Option<HidDevice>
}

#[derive(serde::Serialize)]
pub struct DpiValue {
    value: u32,
    color: String
}

#[derive(serde::Serialize)]
pub struct MouseConfig {
    // version: String,
    // battery_level: u8,
    // battery_charging: bool,
    // battery_voltage: i16,
    report_rate: u16,
    // dpi_values: Vec<DpiValue>,
    // current_dpi_index: u8,
    // max_dpi_index: u8,
}

impl MouseConfig {
    pub fn from_slice(buffer: &[u8]) -> Self {
        MouseConfig {
            report_rate: {
                buffer
                    .first()
                    .and_then(|val| {
                        if *val > 0x10 {
                            Some((*val / 0x10) as u16 * 2000)
                        } else {
                            Some(1000 / *val as u16)
                        }
                    })
                    .unwrap_or(0)
                    .into()
            }
        }
    }
}



#[derive(Error, Debug)]
pub enum AppError {
    #[error("device not found")]
    DeviceNotFound,
    #[error("hid error")]
    HidError(#[from] hidapi::HidError)

    // #[error(transparent)]
    // Io(#[from] std::io::Error)

    // #[error("hid error")]
    // HidError(#[from] hidapi),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
