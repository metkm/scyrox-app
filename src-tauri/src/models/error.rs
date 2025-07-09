use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("device not found")]
    DeviceNotFound,
    #[error("hid error")]
    HidError(#[from] hidapi::HidError),
    #[error("invalid value")]
    InvalidValue,
    #[error("crc problem")]
    CrcProblem,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
