use std::error::Error;
use std::{fmt};

pub mod device;
pub mod ec;
pub mod input_event;
pub mod tracker;
pub mod uinput;

pub use device::*;
pub use input_event::*;
pub use tracker::*;

#[derive(Debug)]
pub enum EvdevError {
    DeviceGrabError,
    ErrnoError(i32),
    InternalEventDropped,
}

impl Error for EvdevError {}

impl fmt::Display for EvdevError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self {
            &EvdevError::DeviceGrabError => write!(f, "Unable to grab device"),
            &EvdevError::ErrnoError(e) => write!(f, "Errno error: {}", errno::Errno(*e)),
            &EvdevError::InternalEventDropped => write!(f, "InternalEventDropped"),
        };
    }
}
