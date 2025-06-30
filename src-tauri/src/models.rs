use hidapi::HidDevice;


pub struct AppState {
    pub device: Option<HidDevice>
}
