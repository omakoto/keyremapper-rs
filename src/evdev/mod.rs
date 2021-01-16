use core::panic;
use ec::EventType;
use fmt::Display;
use glob::{self, PatternError};
use std::{collections::HashMap, fmt, fs::File, path::Path, str::Utf8Error, sync::RwLock};
use std::{error::Error, os::unix::io::AsRawFd, sync::Arc};

use crate::native::{self, string_from_c_str};

pub mod ec;
pub mod uinput;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEvent {
    pub time_sec: i64,
    pub time_usec: i64,
    pub event_type: ec::EventType,
    pub code: i32,
    pub value: i32,
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match ec::get_type_name(self.event_type as i32) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown type {}]", self.event_type as i32),
        };
        let code_str = match ec::get_code_name(self.event_type as i32, self.code) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown code {}]", self.code),
        };
        return write!(
            f,
            "{{InputEvent: time={}.{:06} type={} code={} value={}}}",
            self.time_sec, self.time_usec, type_str, code_str, self.value
        );
    }
}

impl InputEvent {
    pub fn with_timestamp(
        time_sec: i64,
        time_usec: i64,
        event_type: ec::EventType,
        code: i32,
        value: i32,
    ) -> InputEvent {
        return InputEvent {
            time_sec,
            time_usec,
            event_type,
            code,
            value,
        };
    }

    pub fn new(event_type: ec::EventType, code: i32, value: i32) -> InputEvent {
        return InputEvent {
            time_sec: 0,
            time_usec: 0,
            event_type,
            code,
            value,
        };
    }

    /// Return a new SYN_REPORT event.
    pub fn new_syn_report() -> InputEvent {
        return InputEvent::new(ec::EventType::EV_SYN, ec::SYN_REPORT, 0);
    }

    pub fn new_key_event(code: i32, value: i32) -> InputEvent {
        return InputEvent::new(ec::EventType::EV_KEY, code, value);
    }

    fn from_native_input_event(ie: &native::input_event) -> InputEvent {
        return InputEvent {
            time_sec: ie.time.tv_sec,
            time_usec: ie.time.tv_usec,
            event_type: ec::EventType::from_i32(ie.type_ as i32),
            code: ie.code as i32,
            value: ie.value as i32,
        };
    }

    /// Return true if it's a SYN_REPORT event.
    pub fn is_syn_report(&self) -> bool {
        return self.event_type == ec::EventType::EV_SYN
            && self.code == ec::SYN_REPORT
            && self.value == 0;
    }
}

#[test]
fn test_input_event_format_ev_key() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=KEY_A value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, ec::KEY_A, 1))
    )
}

#[test]
fn test_input_event_format_ev_time() {
    assert_eq!(
        "{InputEvent: time=1.000002 type=EV_KEY code=KEY_A value=1}",
        format!(
            "{}",
            InputEvent {
                time_sec: 1,
                time_usec: 2,
                event_type: ec::EventType::EV_KEY,
                code: ec::KEY_A,
                value: 1,
            }
        )
    )
}

#[test]
fn test_input_event_format_ev_key_btn() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=BTN_1 value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, ec::BTN_1, 1))
    )
}

#[test]
fn test_input_event_format_ev_key_unknown() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=[Unknown code 999999] value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, 999999, 1))
    )
}

// TODO Add more tests

#[derive(Debug, Clone)]
struct InputEventTrackerInner {
    key_states: HashMap<i32, i32>,
    syn_report_pending: bool,
}

#[derive(Debug, Clone)]
pub struct InputEventTracker {
    inner: Arc<RwLock<InputEventTrackerInner>>,
}

impl InputEventTracker {
    pub fn new() -> InputEventTracker {
        return InputEventTracker {
            inner: Arc::new(RwLock::new(InputEventTrackerInner {
                key_states: HashMap::new(),
                syn_report_pending: false,
            })),
        };
    }

    pub fn key_state(&self, code: i32) -> i32 {
        let ks = self.inner.read().unwrap();
        return *ks.key_states.get(&code).unwrap_or(&0);
    }

    pub fn should_send(&self, ev: &InputEvent) -> bool {
        let inner = self.inner.read().unwrap();
        return InputEventTracker::should_send_no_lock(&inner, ev);
    }

    fn should_send_no_lock(inner: &InputEventTrackerInner, ev: &InputEvent) -> bool {
        if ev.is_syn_report() && !inner.syn_report_pending {
            return false;
        }
        if ev.event_type == ec::EventType::EV_KEY {
            let current = *inner.key_states.get(&ev.code).unwrap_or(&0);
            if ev.value == 0 {
                if current == 0 {
                    return false; // Don't send if not pressed.
                }
            } else if ev.value == 1 {
                if current != 0 {
                    return false; // Don't send if already pressed.
                }
            } else if ev.value == 2 {
                if current == 0 {
                    return false; // Don't send if not pressed.
                }
            } else {
                panic!("Invalid value for EV_KEY: {}", ev.value);
            }
        }
        return true;
    }

    pub fn on_event_sent(&self, ev: &InputEvent) {
        let mut inner = self.inner.write().unwrap();
        if ev.event_type == ec::EventType::EV_KEY
            && InputEventTracker::should_send_no_lock(&inner, ev)
        {
            inner.key_states.insert(ev.code, ev.value);
        }
        inner.syn_report_pending = !ev.is_syn_report();
    }

    pub fn reset(&self) -> Vec<InputEvent> {
        return self.reset_with_callback(|_| Ok(())).unwrap();
    }

    pub fn reset_with_callback<F>(&self, mut callback: F) -> Result<Vec<InputEvent>, Box<dyn Error>>
    where
        F: FnMut(&InputEvent) -> Result<(), Box<dyn Error>>,
    {
        let mut inner = self.inner.write().unwrap();

        let mut reset_events = vec![];

        use itertools::Itertools;
        for (code, value) in inner.key_states.iter().sorted() {
            if *value > 0 {
                let ev = InputEvent::new(ec::EventType::EV_KEY, *code, 0);
                reset_events.push(ev);
                reset_events.push(InputEvent::new_syn_report());
                callback(&ev)?;
            }
        }
        inner.key_states.clear();
        inner.syn_report_pending = false;
        return Ok(reset_events);
    }
}

#[test]
fn test_input_event_tracker() {
    let et = InputEventTracker::new();

    assert_eq!(0, et.key_state(0));
    assert_eq!(0, et.key_state(1));

    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0))
    );
    assert_eq!(
        false,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1))
    );
    assert_eq!(
        false,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1))
    );

    et.on_event_sent(&InputEvent::new(EventType::EV_KEY, 1, 1));
    et.on_event_sent(&InputEvent::new_syn_report());

    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0))
    );
    assert_eq!(
        false,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1))
    );

    {
        let et2 = et.clone();

        et2.on_event_sent(&InputEvent::new(EventType::EV_KEY, 1, 2));
        et2.on_event_sent(&InputEvent::new_syn_report());
    }

    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0))
    );
    assert_eq!(
        false,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2))
    );
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1))
    );

    et.on_event_sent(&InputEvent::new(EventType::EV_KEY, 3, 1));
    et.on_event_sent(&InputEvent::new_syn_report());

    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_REL, 5, 1))
    );
    et.on_event_sent(&InputEvent::new(EventType::EV_REL, 5, 1));
    et.on_event_sent(&InputEvent::new_syn_report());
    assert_eq!(
        true,
        et.should_send(&InputEvent::new(EventType::EV_REL, 5, 1))
    );

    {
        let reset_events = et.reset();

        let expected: Vec<InputEvent> = vec![
            InputEvent::new(EventType::EV_KEY, 1, 0),
            InputEvent::new_syn_report(),
            InputEvent::new(EventType::EV_KEY, 3, 0),
            InputEvent::new_syn_report(),
        ];
        assert_eq!(expected, reset_events);
    }

    {
        let reset_events = et.reset();

        let expected: Vec<InputEvent> = vec![];
        assert_eq!(expected, reset_events);
    }
}

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
            let ai: *const native::input_absinfo =
                native::libevdev_get_abs_info(device, code as u32);
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

        let mut all_keys = vec![];
        for code in ec::ALL_KEYS {
            all_keys.push(*code);
        }

        ret.events.insert(ec::EventType::EV_KEY, all_keys);

        return ret;
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
                id_str: format!("{:04x} {:04x}", vendor, product),
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
            time: native::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
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

pub fn list_devices_from_path_with_filter<F>(
    device_path_glob: &str,
    filter: F,
) -> Result<Vec<EvdevDevice>, EvdevError>
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
