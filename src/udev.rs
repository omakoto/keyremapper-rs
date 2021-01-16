use std::{error::Error, fmt, ptr, sync::Arc};

use native::c_string_from_str;

use crate::native::{self, string_from_c_str};

#[derive(Debug)]
pub enum UdevError {
    ErrnoError(String, i32),
    UnknownError(String),
    UnknownErrorWithInner(String, Box<dyn Error>),
}

impl Error for UdevError {}

impl UdevError {
    fn from_errno(msg: &str, errno: i32) -> UdevError {
        return UdevError::ErrnoError(msg.to_string(), errno);
    }

    fn new_unknown_error(msg: &str) -> UdevError {
        return UdevError::UnknownError(msg.to_string());
    }

    fn new_unknown_error_with_inner(msg: &str, err: Box<dyn Error>) -> UdevError {
        return UdevError::UnknownErrorWithInner(msg.to_string(), err);
    }
}

impl fmt::Display for UdevError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self {
            &UdevError::ErrnoError(msg, e) => {
                write!(f, "Errno error: {} {}", msg, errno::Errno(*e))
            }
            &UdevError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
            &UdevError::UnknownErrorWithInner(msg, inner) => {
                write!(f, "Unknown error: {}: {}", msg, inner)
            }
        };
    }
}

#[derive(Debug, Clone)]
pub enum UdevAction {
    Add,
    Remove,
    Unknown(String),
}

impl UdevAction {
    fn from_string(action: &str) -> UdevAction {
        return match action {
            "add" => UdevAction::Add,
            "remove" => UdevAction::Remove,
            any => UdevAction::Unknown(any.to_string()),
        };
    }
}

#[derive(Debug, Clone)]
pub struct UdevEvent {
    pub action: UdevAction,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct UdevMonitor {
    subsystem: String,
    udev: *mut native::udev,
    udev_monitor: *mut native::udev_monitor,
    udev_fd: Arc<std::os::unix::io::RawFd>,
}

unsafe impl Send for UdevMonitor {}

impl UdevMonitor {
    pub fn new(subsystem: &str) -> Result<UdevMonitor, UdevError> {
        unsafe {
            // Set up udev_monitor.
            let udev = native::udev_new();
            if udev == ptr::null_mut() {
                return Err(UdevError::new_unknown_error("udev_new() failed"));
            }
            let udev_monitor =
                native::udev_monitor_new_from_netlink(udev, c_string_from_str("udev").as_ptr());
            log::debug!(
                "udev_monitor_new_from_netlink() returned {:?}",
                udev_monitor
            );

            native::udev_monitor_filter_add_match_subsystem_devtype(
                udev_monitor,
                c_string_from_str(subsystem).as_ptr(),
                ptr::null(),
            );
            native::udev_monitor_enable_receiving(udev_monitor);

            // Get the FD.
            let fd = native::udev_monitor_get_fd(udev_monitor);
            if fd < 0 {
                return Err(UdevError::from_errno("udev_monitor_get_fd() failed", -fd));
            }

            // Make the FD non-blocking.
            {
                match libc::fcntl(fd, libc::F_GETFD) {
                    -1 => {
                        return Err(UdevError::ErrnoError(
                            "fcntl(F_GETFD) failed".to_string(),
                            errno::errno().0,
                        ));
                    }
                    flags => match libc::fcntl(fd, libc::F_SETFD, flags | libc::O_NONBLOCK) {
                        -1 => {
                            return Err(UdevError::ErrnoError(
                                "fcntl(F_SETFD, O_NONBLOCK) failed".to_string(),
                                errno::errno().0,
                            ));
                        }
                        _ => {}
                    },
                }
            }

            let um = UdevMonitor {
                subsystem: subsystem.to_string(),
                udev: udev,
                udev_monitor: udev_monitor,
                udev_fd: Arc::new(fd),
            };

            um.drain_events();

            return Ok(um);
        }
    }

    pub fn udev_fd(&self) -> std::os::unix::io::RawFd {
        *self.udev_fd
    }

    pub fn next_event(&self) -> Result<UdevEvent, UdevError> {
        unsafe {
            let dev = native::udev_monitor_receive_device(self.udev_monitor);
            if !dev.is_null() {
                let action = string_from_c_str(native::udev_device_get_action(dev));
                let name = string_from_c_str(native::udev_device_get_sysname(dev));
                let path = string_from_c_str(native::udev_device_get_devpath(dev));
                log::debug!("I: ACTION={}\n", action);
                log::debug!("I: DEVNAME={}\n", name);
                log::debug!("I: DEVPATH={}\n", path);
                log::debug!("---\n");

                /* free dev */
                native::udev_device_unref(dev);
                return Ok(UdevEvent {
                    action: UdevAction::from_string(&action),
                    name,
                    path,
                });
            } else {
                return Err(UdevError::new_unknown_error(
                    "udev_monitor_receive_device() failed",
                ));
            }
        }
    }

    pub fn drain_events(&self) {
        loop {
            match self.next_event() {
                Err(_) => {
                    return;
                }
                Ok(_) => {
                    // Consume this event and try the next one.
                }
            }
        }
    }
}

impl Drop for UdevMonitor {
    fn drop(&mut self) {
        if Arc::strong_count(&self.udev_fd) > 1 {
            return;
        }

        unsafe {
            native::udev_monitor_unref(self.udev_monitor);
            self.udev_monitor = ptr::null_mut();

            native::udev_unref(self.udev);
            self.udev = ptr::null_mut();
        }
    }
}
