use glob::{self, PatternError};
use std::{fmt, str::Utf8Error};
use std::{error::Error, os::unix::io::AsRawFd};

pub mod ec;
pub mod uinput;
pub mod input_event;
pub mod device;
pub mod tracker;


pub use input_event::*;
pub use device::*;
pub use tracker::*;

#[derive(Debug)]
pub enum EvdevError {
    IoError(std::io::Error),
    Utf8Error(Utf8Error),
    DeviceGrabError,
    ErrnoError(i32),
    PatternError(glob::PatternError),
    UinputCreationError(String),
    UnknownError(Box<dyn Error>),
    InternalEventDropped,
}

impl EvdevError {
    pub fn new_unknown_error(err: Box<dyn Error>) -> EvdevError {
        return EvdevError::UnknownError(err);
    }
}

impl Error for EvdevError {}

impl fmt::Display for EvdevError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self {
            &EvdevError::IoError(e) => write!(f, "I/O error: {}", e),
            &EvdevError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            &EvdevError::DeviceGrabError => write!(f, "Unable to grab device"),
            &EvdevError::ErrnoError(e) => write!(f, "Errno error: {}", errno::Errno(*e)),
            &EvdevError::PatternError(e) => write!(f, "Pattern error: {}", e),
            &EvdevError::UinputCreationError(msg) => write!(f, "Uinput crewation error: {}", msg),
            &EvdevError::UnknownError(inner) => write!(f, "Unknown error: {}", inner),
            &EvdevError::InternalEventDropped => write!(f, "InternalEventDropped"),
        };
    }
}

impl From<std::io::Error> for EvdevError {
    fn from(err: std::io::Error) -> EvdevError {
        return EvdevError::IoError(err);
    }
}

impl From<Utf8Error> for EvdevError {
    fn from(err: Utf8Error) -> Self {
        return EvdevError::Utf8Error(err);
    }
}

impl From<PatternError> for EvdevError {
    fn from(err: PatternError) -> Self {
        return EvdevError::PatternError(err);
    }
}

impl From<Box<dyn Error>> for EvdevError {
    fn from(err: Box<dyn Error>) -> Self {
        return EvdevError::UnknownError(err);
    }
}

