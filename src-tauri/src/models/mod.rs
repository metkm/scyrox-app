use hidapi::HidDevice;

use crate::{device, models::error::AppError};

pub mod dpi;
pub mod mouse;
pub mod battery;
pub mod error;

pub struct AppState {
    pub device: Option<HidDevice>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            device: device::get_device()
        }
    }
}

impl AppState {
    pub fn reconnect(&mut self) -> Result<(), AppError> {
        let device = device::get_device();
        self.device = device;

        if self.device.is_some() {
            Ok(())
        } else {
            Err(AppError::DeviceNotFound)
        }
    }
}
