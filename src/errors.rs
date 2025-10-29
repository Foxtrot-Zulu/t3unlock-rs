use thiserror::Error;

#[derive(Debug, Error)]
pub enum UsbError {
    #[error("USB device not found")]
    NotFound,
    #[error("Access denied (udev permissions?)")]
    AccessDenied,
    #[error("Device is busy")]
    Busy,
    #[error("I/O error")]
    Io,
    #[error("Timeout")]
    Timeout,
    #[error("Protocol mismatch or unsupported device")]
    Protocol,
    #[error("Bad password")]
    BadPassword,
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, UsbError>;
