use std::os::unix::io::AsRawFd;
use std::{collections::HashMap, fs::File, path::Path, sync::Arc};

use crate::native::{self, string_from_c_str};

use super::{
    ec::{self, EventType},
    EvdevError, InputEvent,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct EvdevAbsInfo {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}

impl EvdevAbsInfo {
    pub(crate) fn to_raw_absinfo(&self) -> native::input_absinfo {
        return native::input_absinfo {
            value: self.value,
            minimum: self.minimum,
            maximum: self.maximum,
            fuzz: self.fuzz,
            flat: self.flat,
            resolution: self.resolution,
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventsDescriptor {
    pub events: HashMap<EventType, Vec<i32>>,
    pub abs_info: HashMap<i32, EvdevAbsInfo>,
}

impl EventsDescriptor {
    pub fn new() -> EventsDescriptor {
        EventsDescriptor {
            events: HashMap::new(),
            abs_info: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.events.is_empty() && self.abs_info.is_empty();
    }

    fn add_codes(&mut self, device: *mut native::libevdev, event: ec::EventType, max_code: i32) {
        let mut codes: Vec<i32> = vec![];
        unsafe {
            if native::libevdev_has_event_type(device, event as u32) != 1 {
                return;
            }
            for code in 0..max_code + 1 {
                if native::libevdev_has_event_code(device, event as u32, code as u32) == 1 {
                    codes.push(code);
                }
            }
        }
        if codes.len() > 0 {
            self.events.insert(event, codes);
        }
    }

    fn add_absinfo(&mut self, device: *mut native::libevdev, code: i32) {
        unsafe {
            if native::libevdev_has_event_type(device, ec::EV_ABS as u32) != 1 {
                return;
            }
            let ai: *const native::input_absinfo = native::libevdev_get_abs_info(device, code as u32);
            if ai == std::ptr::null() {
                return;
            }
            let absinfo = EvdevAbsInfo {
                value: (*ai).value as i32,
                minimum: (*ai).minimum as i32,
                maximum: (*ai).maximum as i32,
                fuzz: (*ai).fuzz as i32,
                flat: (*ai).flat as i32,
                resolution: (*ai).resolution as i32,
            };
            self.abs_info.insert(code, absinfo);
        }
    }

    fn from_native_input_device(device: *mut native::libevdev) -> EventsDescriptor {
        let mut ret = EventsDescriptor::new();

        ret.add_codes(device, ec::EventType::EV_SYN, ec::SYN_MAX);
        ret.add_codes(device, ec::EventType::EV_KEY, ec::KEY_MAX);
        ret.add_codes(device, ec::EventType::EV_REL, ec::REL_MAX);
        ret.add_codes(device, ec::EventType::EV_MSC, ec::MSC_MAX);
        ret.add_codes(device, ec::EventType::EV_SW, ec::SW_MAX);
        ret.add_codes(device, ec::EventType::EV_LED, ec::LED_MAX);
        ret.add_codes(device, ec::EventType::EV_SND, ec::SND_MAX);
        ret.add_codes(device, ec::EventType::EV_REP, ec::REP_MAX);
        // ret.add_codes(device, ec::EventType::EV_FF, ec::FF_MAX);
        // ret.add_codes(device, ec::EventType::EV_PWR, ec::PWR_MAX);
        // ret.add_codes(device, ec::EventType::EV_FF_STATUS, ec::FF_STATUS_MAX);

        for code in 0..ec::ABS_MAX + 1 {
            ret.add_absinfo(device, code)
        }
        return ret;
    }

    pub fn with_all_key_events() -> EventsDescriptor {
        let mut ret = EventsDescriptor::new();
        ret.add_all_key_events();
        return ret;
    }

    pub fn add_all_key_events(&mut self) {
        let mut all_keys = vec![];
        for code in ec::ALL_KEYS {
            all_keys.push(*code);
        }

        self.events.insert(ec::EventType::EV_KEY, all_keys);
    }

    pub fn with_mouse_events() -> EventsDescriptor {
        let mut ret = EventsDescriptor::new();
        ret.add_mouse_events();
        return ret;
    }

    pub fn add_mouse_events(&mut self) {
        self.events.insert(
            ec::EventType::EV_KEY,
            vec![
                ec::BTN_MOUSE,
                ec::BTN_LEFT,
                ec::BTN_MIDDLE,
                ec::BTN_RIGHT,
                ec::BTN_SIDE,
                ec::BTN_EXTRA,
                ec::BTN_BACK,
                ec::BTN_FORWARD,
            ],
        );
        self.events.insert(
            ec::EventType::EV_REL,
            vec![
                ec::REL_X,
                ec::REL_Y,
                ec::REL_Z,
                ec::REL_RX,
                ec::REL_RY,
                ec::REL_RZ,
                ec::REL_WHEEL,
                ec::REL_HWHEEL,
                ec::REL_WHEEL_HI_RES,
                ec::REL_HWHEEL_HI_RES,
                ec::REL_MISC,
            ],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct EvdevPtr {
    ptr: *mut native::libevdev,
}

unsafe impl Send for EvdevPtr {}
unsafe impl Sync for EvdevPtr {}

#[derive(Debug, Clone)]
pub struct EvdevDevice {
    path: String,
    device: EvdevPtr,
    file: Arc<File>,
    fd: std::os::unix::io::RawFd,
    name: String,
    vendor: i32,
    product: i32,
    id_str: String,

    // physical_location: String,
    // uniq_id: String,
    events: EventsDescriptor,

    grabbed: bool,
}

impl EvdevDevice {
    pub fn with_path(path: &Path) -> Result<EvdevDevice, EvdevError> {
        log::debug!("Opening evdev device {:?}...", path);

        unsafe {
            let file = File::open(&path)?;
            let fd = file.as_raw_fd();

            let device = native::libevdev_new();
            native::libevdev_set_fd(device, fd);

            let events = EventsDescriptor::from_native_input_device(device);

            let vendor = native::libevdev_get_id_vendor(device);
            let product = native::libevdev_get_id_product(device);

            return Ok(EvdevDevice {
                path: String::from(path.to_str().unwrap()),
                device: EvdevPtr { ptr: device },
                file: Arc::new(file),
                fd: fd,
                name: string_from_c_str(native::libevdev_get_name(device)),
                // physical_location: String::from(CStr::from_ptr(native::libevdev_get_phys(device)).to_str()?),
                // uniq_id: String::from(CStr::from_ptr(native::libevdev_get_uniq(device)).to_str()?),
                vendor: vendor,
                product: product,
                id_str: format!("v{:04x} p{:04x}", vendor, product),
                events: events,
                grabbed: false,
            });
        }
    }

    pub fn path(&self) -> String {
        return self.path.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn vendor_id(&self) -> i32 {
        return self.vendor;
    }

    pub fn product_id(&self) -> i32 {
        return self.product;
    }

    pub fn id_str(&self) -> String {
        return self.id_str.clone();
    }

    pub fn supported_events(&self) -> EventsDescriptor {
        self.events.clone()
    }

    pub fn grab(&mut self, grab: bool) -> Result<(), EvdevError> {
        unsafe {
            if self.grabbed == grab {
                return Ok(());
            }
            let mode = if grab {
                native::libevdev_grab_mode_LIBEVDEV_GRAB
            } else {
                native::libevdev_grab_mode_LIBEVDEV_UNGRAB
            };
            let result = native::libevdev_grab(self.device.ptr, mode);
            if result == -libc::EBUSY {
                return Err(EvdevError::DeviceGrabError);
            }
            if result != 0 {
                return Err(EvdevError::ErrnoError(-result));
            }
            self.grabbed = grab;
            return Ok(());
        }
    }

    pub fn device_fd(&self) -> std::os::unix::io::RawFd {
        return self.fd;
    }

    pub fn has_event_pending(&self) -> Result<bool, EvdevError> {
        unsafe {
            let result = native::libevdev_has_event_pending(self.device.ptr);
            if result < 0 {
                return Err(EvdevError::ErrnoError(-result));
            }
            return Ok(result == 1);
        }
    }

    fn next_single_event(&self, sync: bool) -> Result<InputEvent, EvdevError> {
        let mut ie = native::input_event {
            time: native::timeval { tv_sec: 0, tv_usec: 0 },
            type_: 0,
            code: 0,
            value: 0,
        };
        let flags = if sync {
            native::libevdev_read_flag_LIBEVDEV_READ_FLAG_SYNC
        } else {
            native::libevdev_read_flag_LIBEVDEV_READ_FLAG_NORMAL
        };
        unsafe {
            loop {
                let status = native::libevdev_next_event(self.device.ptr, flags, &mut ie);
                if status == -libc::EAGAIN {
                    continue;
                }
                if status < 0 {
                    return Err(EvdevError::ErrnoError(-status));
                }
                if status == native::libevdev_read_status_LIBEVDEV_READ_STATUS_SUCCESS as i32 {
                    return Ok(InputEvent::from_native_input_event(&ie));
                }
                if status == native::libevdev_read_status_LIBEVDEV_READ_STATUS_SYNC as i32 {
                    return Err(EvdevError::InternalEventDropped);
                }
                panic!("libevdev_next_event returned unknown result: {}", status)
            }
        }
    }

    pub fn next_events(&self) -> Result<Vec<InputEvent>, EvdevError> {
        let mut ret = vec![];

        while self.has_event_pending()? {
            match self.next_single_event(false) {
                Ok(ie) => ret.push(ie),
                Err(EvdevError::InternalEventDropped) => todo!(),
                Err(e) => return Err(e),
            };
        }
        Ok(ret)
    }
}

impl Drop for EvdevDevice {
    fn drop(&mut self) {
        let file_rc = Arc::strong_count(&self.file);
        if file_rc > 1 {
            return;
        }
        log::debug!("Closing evdev device {:?}...", self.path);

        unsafe {
            native::libevdev_free(self.device.ptr);
        }
    }
}

pub fn list_devices() -> Result<Vec<EvdevDevice>, EvdevError> {
    return list_devices_from_path("/dev/input/event*");
}

pub fn list_devices_from_path(device_path_glob: &str) -> Result<Vec<EvdevDevice>, EvdevError> {
    return list_devices_from_path_with_filter(device_path_glob, |_device| true);
}

pub fn list_devices_from_path_with_filter<F>(device_path_glob: &str, filter: F) -> Result<Vec<EvdevDevice>, EvdevError>
where
    F: Fn(&EvdevDevice) -> bool,
{
    let mut ret: Vec<EvdevDevice> = vec![];

    for entry in glob::glob(device_path_glob).expect("Failed to list devices ") {
        match entry {
            Ok(path) => {
                let device = match EvdevDevice::with_path(path.as_path()) {
                    Ok(device) => device,
                    Err(e) => {
                        log::error!("Unable to open device {:?}: {}", path, e);
                        continue;
                    }
                };
                if !filter(&device) {
                    log::debug!("Skip: device={:?}", device);
                    continue;
                }
                log::debug!("Detectd: device={:?}", device);
                ret.push(device);
            }
            Err(e) => {
                log::error!("Glob failed: {:?}", e);
                continue;
            }
        }
    }
    ret.sort_by(|a, b| natord::compare(&a.path, &b.path));

    return Ok(ret);
}
