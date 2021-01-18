use std::{
    ffi::c_void,
    fs::{self, File},
    os::unix::io::AsRawFd,
    sync::Arc,
};

use parking_lot::{ReentrantMutex, ReentrantMutexGuard, RwLock};

use crate::native::{self, c_string_from_str};

use super::{ec, EvdevError, EventsDescriptor};
use super::{InputEvent, InputEventTracker};

#[derive(Debug, Clone, Copy)]
struct UinputPtr {
    ptr: *mut native::libevdev_uinput,
}

unsafe impl Send for UinputPtr {}
unsafe impl Sync for UinputPtr {}

#[derive(Debug, Clone)]
struct RawUinput {
    name: String,
    file: Arc<File>,
    fd: std::os::unix::io::RawFd,
    uinput: UinputPtr,
}

impl RawUinput {
    fn new(name: String, file: File, fd: i32, uinput: *mut native::libevdev_uinput) -> RawUinput {
        return RawUinput {
            name: name,
            file: Arc::new(file),
            fd: fd,
            uinput: UinputPtr { ptr: uinput },
        };
    }

    fn send_event(&mut self, ev: &crate::evdev::InputEvent) -> Result<(), EvdevError> {
        log::debug!("Writing event: {}", ev);
        let ret = unsafe { native::libevdev_uinput_write_event(self.uinput.ptr, ev.event_type as u32, ev.code as u32, ev.value) };
        if ret < 0 {
            return Err(EvdevError::ErrnoError(-ret));
        }
        return Ok(());
    }
}

impl Drop for RawUinput {
    fn drop(&mut self) {
        let file_rc = Arc::strong_count(&self.file);
        if file_rc > 1 {
            return;
        }
        log::debug!("Closing uinput device {:?}...", self.name);

        unsafe {
            native::libevdev_uinput_destroy(self.uinput.ptr);
        }
    }
}

/// Uinput with event tracking.
#[derive(Debug, Clone)]
pub struct Uinput {
    uinput: RawUinput,
    event_tracker: InputEventTracker,
}

impl Uinput {
    pub fn new(name: &str, events: &EventsDescriptor) -> Result<Uinput, EvdevError> {
        if name.len() == 0 {
            return Err(EvdevError::UinputCreationError("Name must not be empty".to_string()));
        }
        unsafe {
            let file = fs::OpenOptions::new().read(true).write(true).open("/dev/uinput")?;
            let fd = file.as_raw_fd();

            let dev = native::libevdev_new();
            native::libevdev_set_name(dev, c_string_from_str(name).as_ptr());

            for (event_type, codes) in &events.events {
                match event_type {
                    ec::EventType::EV_SYN => continue,
                    ec::EventType::EV_REP => continue,
                    _ => {}
                }
                log::debug!("Enabling event type {:?}...", event_type);
                let ret = native::libevdev_enable_event_type(dev, (*event_type) as u32);
                if ret != 0 {
                    return Err(EvdevError::UinputCreationError("libevdev_enable_event_type failed".to_string()));
                }
                for code in codes {
                    // log::debug!("  {}...", code);
                    let ret = native::libevdev_enable_event_code(dev, (*event_type) as u32, (*code) as u32, std::ptr::null());
                    if ret != 0 {
                        return Err(EvdevError::UinputCreationError("libevdev_enable_event_code failed".to_string()));
                    }
                }
            }
            if events.abs_info.len() > 0 {
                let event_type = ec::EventType::EV_ABS;
                log::debug!("Enabling event type {:?}...", event_type);
                let ret = native::libevdev_enable_event_type(dev, event_type as u32);
                if ret != 0 {
                    return Err(EvdevError::UinputCreationError("libevdev_enable_event_type failed".to_string()));
                }
                for (code, absinfo) in &events.abs_info {
                    // log::debug!("  {}...", code);
                    let raw_absinfo = absinfo.to_raw_absinfo();
                    let ret = native::libevdev_enable_event_code(dev, event_type as u32, (*code) as u32, (&raw_absinfo) as *const _ as *const c_void);
                    if ret != 0 {
                        return Err(EvdevError::UinputCreationError("libevdev_enable_event_code failed".to_string()));
                    }
                }
            }

            let mut uinput: *mut native::libevdev_uinput = std::ptr::null_mut();
            let err = native::libevdev_uinput_create_from_device(dev, fd, &mut uinput);
            if err < 0 {
                return Err(EvdevError::ErrnoError(-err));
            }
            return Ok(Uinput {
                uinput: RawUinput::new(name.to_string(), file, fd, uinput),
                event_tracker: InputEventTracker::new(),
            });
        }
    }

    /// Send a single event. Doesn't automatically send a syn event.
    pub fn send_event(&mut self, ev: &crate::evdev::InputEvent) -> Result<(), EvdevError> {
        if !self.event_tracker.should_send(ev) {
            return Ok(()); // Redundant event, don't send.
        }
        self.uinput.send_event(ev)?;
        self.event_tracker.on_event_sent(ev);
        return Ok(());
    }

    pub fn send_syn_report(&mut self) -> Result<(), EvdevError> {
        self.send_event(&InputEvent::new_syn_report())?;
        return Ok(());
    }

    /// Send multiple events with a SYN_REPORT.
    pub fn send_events(&mut self, events: &[InputEvent]) -> Result<(), EvdevError> {
        let mut last_event_is_sync = false;
        for ev in events {
            self.send_event(ev)?;
            last_event_is_sync = ev.is_syn_report();
        }
        if !last_event_is_sync {
            self.send_syn_report()?;
        }
        return Ok(());
    }

    pub fn reset(&mut self) -> Result<(), EvdevError> {
        // let reset_events = self.event_tracker.reset();
        // for ev in &reset_events {
        //     self.send_event(ev)?;
        // }
        // return Ok(());
        let event_tracker = &mut self.event_tracker;
        let uinput = &mut self.uinput;

        event_tracker.reset_with_callback(|ev| {
            uinput.send_event(ev)?;
            Ok(())
        })?;
        return Ok(());
    }

    pub fn key_state(&self, code: i32) -> i32 {
        return self.event_tracker.key_state(code);
    }
}

/// Thread safe version of Uinput.
#[derive(Debug, Clone)]
pub struct SyncedUinput {
    lock: Arc<ReentrantMutex<()>>,
    uinput: Arc<RwLock<Uinput>>,
}

impl SyncedUinput {
    pub fn new(name: &str, events: &EventsDescriptor) -> Result<SyncedUinput, EvdevError> {
        Ok(SyncedUinput {
            lock: Arc::new(ReentrantMutex::new(())),
            uinput: Arc::new(RwLock::new(Uinput::new(name, events)?)),
        })
    }

    /// Send a single event with a SYN_REPORT.
    pub fn send_event(&self, ev: &crate::evdev::InputEvent) -> Result<(), EvdevError> {
        let _ = self.lock();
        let mut w = self.uinput.write();
        w.send_event(ev)?;
        w.send_syn_report()?;
        return Ok(());
    }

    pub fn send_syn_report(&self) -> Result<(), EvdevError> {
        let _ = self.lock();
        return self.uinput.write().send_syn_report();
    }

    /// Send multiple events with a SYN_REPORT.
    pub fn send_events(&self, events: &[InputEvent]) -> Result<(), EvdevError> {
        let _ = self.lock();
        return self.uinput.write().send_events(events);
    }

    pub fn reset(&self) -> Result<(), EvdevError> {
        let _ = self.lock();
        return self.uinput.write().reset();
    }

    pub fn key_state(&self, code: i32) -> i32 {
        let _ = self.lock();
        return self.uinput.read().key_state(code);
    }

    pub fn lock(&self) -> ReentrantMutexGuard<()> {
        return self.lock.lock();
    }
}
