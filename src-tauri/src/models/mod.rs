use hidapi::HidDevice;

use crate::device;

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

