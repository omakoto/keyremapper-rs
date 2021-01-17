#![allow(dead_code)]

use std::os::unix::io::RawFd;
use std::{
    cell::RefCell,
    error::Error,
    mem::MaybeUninit,
    process::{self, Command},
    sync::Arc,
    thread,
    time::Duration,
};

use clap::{App, Arg};
use parking_lot::ReentrantMutex;
use rand::prelude::*;

use evdev::{
    ec::{self, EventType},
    uinput::SyncedUinput,
    EventsDescriptor, InputEventTracker,
};

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use notify_rust::{Notification, NotificationHandle, Timeout};
use select::{pselect, FdSet};
use singleton::ensure_singleton;
use udev::UdevMonitor;

pub mod config;
pub mod evdev;
pub(crate) mod native;
pub mod res;
pub(crate) mod select;
pub(crate) mod singleton;
pub mod udev;

pub use crate::config::*;

const UINPUT_DEVICE_NAME_PREFIX: &str = "key-remapper";

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Find all the evdev devices matching the given `KeyRemapperConfiguration`.
fn find_devices(config: &KeyRemapperConfiguration) -> Result<Vec<evdev::EvdevDevice>> {
    log::debug!("Looking for devices...");
    let mut selected: Vec<evdev::EvdevDevice> = vec![];

    let device_name_re = config.device_name_regex_re.as_ref().unwrap();
    let id_re = config.id_regex_re.as_ref().unwrap();

    let filter = |device: &evdev::EvdevDevice| {
        if device.name().starts_with(UINPUT_DEVICE_NAME_PREFIX) {
            return false; // Don't use uinput devices from our own and other instances.
        }
        // Filter by name and id.
        if !device_name_re.is_match(&device.name()) {
            return false;
        }
        if !id_re.is_match(&device.id_str()) {
            return true;
        }

        // Filter by supported event types.
        if config.use_non_keyboard {
            return true; // Select all devices.
        }
        if device.supported_events().abs_info.len() > 0 {
            return false; // Don't use devices with abs_info.
        }
        let mut select = false;
        for event_type in device.supported_events().events.keys() {
            match event_type {
                EventType::EV_KEY => {
                    select = true;
                }
                EventType::EV_SYN | EventType::EV_MSC | EventType::EV_LED | EventType::EV_REP => {
                    // They are okay for a keyboard device to have.
                }
                _ => {
                    return false;
                }
            }
        }
        return select;
    };

    for mut device in evdev::list_devices_from_path_with_filter("/dev/input/event*", filter)? {
        log::info!("Using device {} {}...", device.name(), device.id_str());
        if config.grab_devices {
            match device.grab(true) {
                Ok(_) => {}
                Err(evdev::EvdevError::DeviceGrabError) => {
                    log::warn!("Unable to grab device {}. Already grabbed?", device.name());
                    continue;
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }

        selected.push(device);
    }

    return Ok(selected);
}

pub struct KeyRemapperInput {
    config: KeyRemapperConfiguration,
    devices: Vec<evdev::EvdevDevice>,
}

impl KeyRemapperInput {
    fn new(config: KeyRemapperConfiguration) -> Result<KeyRemapperInput> {
        // TODO Need a udev too.

        // Find the target devices.
        return Ok(KeyRemapperInput {
            config: config,
            devices: vec![],
        });
    }

    fn refresh_devices(&mut self) -> Result<()> {
        self.devices = find_devices(&self.config)?;
        Ok(())
    }

    fn release_devices(&mut self) {
        self.devices.clear();
    }

    fn find_device_by_fd(&self, fd: std::os::unix::io::RawFd) -> &evdev::EvdevDevice {
        for device in &self.devices {
            if fd == device.device_fd() {
                return device;
            }
        }
        panic!("Device for fd {} not found", fd);
    }
}

pub struct KeyRemapperUi {
    app_indicator: AppIndicator,
    notification: NotificationHandle,
}

unsafe impl Send for KeyRemapperUi {}
unsafe impl Sync for KeyRemapperUi {}

fn restart_process() {
    let args: Vec<String> = std::env::args().collect();
    use std::os::unix::process::CommandExt;
    let err = Command::new(&args[0]).args(&args[1..]).exec();
    panic!("Unable to restart process: {}", err);
}

impl KeyRemapperUi {
    fn new(config: &KeyRemapperConfiguration) -> Result<KeyRemapperUi> {
        let mut indicator = AppIndicator::new(&config.name, "");
        indicator.set_status(AppIndicatorStatus::Active);

        // Set up for GUIss

        // Set the icon.
        let icon = match &config.icon {
            Some(path) => path.clone(),
            None => {
                res::get_gio_resource_as_file("keyremapper-rs", res::DEFAULT_ICON_NAME, &|| {
                    return res::load_gio_resources();
                })
            }
        };

        indicator.set_icon(&(icon.into_os_string().into_string().unwrap()));

        // Set up the menu.
        let mut m = gtk::Menu::new();
        let menu_quit = gtk::MenuItem::with_label(&format!("Exit {}", config.name));
        menu_quit.connect_activate(|_| {
            gtk::main_quit();
        });
        m.append(&menu_quit);

        let menu_restart = gtk::MenuItem::with_label(&format!("Restart {}", config.name));
        menu_restart.connect_activate(|_| {
            restart_process();
        });
        m.append(&menu_restart);

        indicator.set_menu(&mut m);
        m.show_all();

        let mut notification = Notification::new();
        let notification = notification
            .summary(&config.name)
            .body(&format!("{} started", config.name))
            .show()
            .unwrap();

        return Ok(KeyRemapperUi {
            app_indicator: indicator,
            notification: notification,
        });
    }

    fn show_notiication_with_timeout(&mut self, message: &str, timeout: Duration) {
        let notification = &mut self.notification;
        notification
            .body(message)
            .timeout(Timeout::Milliseconds(timeout.as_millis() as u32));
        notification.update();
    }
}

/// Create a new uinput device using the given `KeyRemapperConfiguration` with a suffix.
fn create_uinput(
    config: &KeyRemapperConfiguration,
    name_suffix: &str,
    supported_events: &EventsDescriptor,
) -> Result<SyncedUinput> {
    let mut name = UINPUT_DEVICE_NAME_PREFIX.to_string();
    name.push_str(&config.uinput_device_name_suffix);
    name.push_str(name_suffix);

    let ui = SyncedUinput::new(&name, supported_events)?;
    return Ok(ui);
}

// #[derive(Debug, Clone)]
pub struct KeyRemapper {
    config: KeyRemapperConfiguration,
    uinput: Option<SyncedUinput>,
    input: Arc<ReentrantMutex<RefCell<KeyRemapperInput>>>,
    input_event_tracker: Arc<ReentrantMutex<RefCell<InputEventTracker>>>,
    ui: Arc<ReentrantMutex<RefCell<KeyRemapperUi>>>,

    all_uinputs: Arc<ReentrantMutex<RefCell<Vec<SyncedUinput>>>>,
}

const MODIFIER_COUNT: usize = 8; // We need this for ModifierState as a const.

static MODIFIER_KEYS: &'static [i32; MODIFIER_COUNT] = &[
    ec::KEY_LEFTALT,
    ec::KEY_RIGHTALT,
    ec::KEY_LEFTCTRL,
    ec::KEY_RIGHTCTRL,
    ec::KEY_LEFTSHIFT,
    ec::KEY_RIGHTSHIFT,
    ec::KEY_LEFTMETA,
    ec::KEY_RIGHTMETA,
];

type ModifierState = [bool; MODIFIER_COUNT];

impl KeyRemapper {
    fn new(config: KeyRemapperConfiguration) -> KeyRemapper {
        let ui = KeyRemapperUi::new(&config).unwrap();

        // Set up uinput
        // let uinput = if config.builder.
        let uinput = if config.write_to_uinput {
            Some(
                create_uinput(&config, "", &config.uinput_events)
                    .expect("failed to create uinput device"),
            )
        } else {
            None
        };

        let input =
            KeyRemapperInput::new(config.clone()).expect("failed to initialize input devices");

        let ret = KeyRemapper {
            config: config,
            uinput: uinput,
            input: Arc::new(ReentrantMutex::new(RefCell::new(input))),
            ui: Arc::new(ReentrantMutex::new(RefCell::new(ui))),
            all_uinputs: Arc::new(ReentrantMutex::new(RefCell::new(vec![]))),
            input_event_tracker: Arc::new(ReentrantMutex::new(RefCell::new(
                InputEventTracker::new(),
            ))),
        };
        if let Some(u) = ret.uinput.as_ref() {
            ret.add_uinput(&u);
        }
        return ret;
    }

    /// Create a new uinput device supporting given events using the with a suffix.
    pub fn create_uinput(
        &self,
        name_suffix: &str,
        supported_events: &EventsDescriptor,
    ) -> SyncedUinput {
        let u = create_uinput(&self.config, name_suffix, supported_events)
            .expect("failed to create uinput device");
        self.add_uinput(&u);
        return u;
    }

    /// Create a new uinput device supporting mouse events using the with a suffix.
    pub fn create_mouse_uinput(&self, name_suffix: &str) -> SyncedUinput {
        let mut supported_events = EventsDescriptor::default();
        supported_events.events.insert(
            ec::EventType::EV_KEY,
            vec![
                ec::BTN_LEFT,
                ec::BTN_MIDDLE,
                ec::BTN_RIGHT,
                ec::BTN_SIDE,
                ec::BTN_EXTRA,
                ec::BTN_BACK,
                ec::BTN_FORWARD,
            ],
        );
        supported_events.events.insert(
            ec::EventType::EV_REL,
            vec![
                ec::REL_X,
                ec::REL_Y,
                ec::REL_WHEEL,
                ec::REL_HWHEEL,
                ec::REL_WHEEL_HI_RES,
                ec::REL_HWHEEL_HI_RES,
            ],
        );

        return self.create_uinput(name_suffix, &supported_events);
    }

    fn add_uinput(&self, uinput: &SyncedUinput) {
        let all_uinputs = self.all_uinputs.lock();
        all_uinputs.borrow_mut().push(uinput.clone());
    }

    pub fn show_notiication(&self, message: &str) {
        self.show_notiication_with_timeout(message, Duration::from_secs(3))
    }

    pub fn show_notiication_with_timeout(&self, message: &str, timeout: Duration) {
        let ui = self.ui.lock();
        ui.borrow_mut()
            .show_notiication_with_timeout(message, timeout);
    }

    fn ensure_uinput(&self) {
        if self.uinput.is_none() {
            panic!("uinput device is not available");
        }
    }

    pub fn send_syn_report(&self) {
        self.ensure_uinput();
        self.uinput
            .as_ref()
            .unwrap()
            .send_event(&evdev::InputEvent::new_syn_report())
            .unwrap();
    }

    pub fn send_event(&self, event: &evdev::InputEvent) {
        self.ensure_uinput();
        self.uinput.as_ref().unwrap().send_event(event).unwrap();
    }

    pub fn send_events(&self, events: &[evdev::InputEvent]) {
        self.ensure_uinput();
        self.uinput.as_ref().unwrap().send_events(events).unwrap();
    }

    pub fn send_key_event(&self, code: i32, value: i32) {
        self.send_event(&evdev::InputEvent::new_key_event(code, value));
    }

    pub fn send_key_events(&self, code_and_values: &[(i32, i32)]) {
        let mut events: Vec<evdev::InputEvent> = vec![];
        for cv in code_and_values {
            events.push(evdev::InputEvent::new_key_event(cv.0, cv.1));
        }
        self.send_events(&events);
    }

    pub fn reset_out(&self) {
        let all_uinputs = self.all_uinputs.lock();
        for uinput in all_uinputs.borrow_mut().iter() {
            uinput.reset().unwrap();
        }
    }

    pub fn get_in_key_state(&self, code: i32) -> i32 {
        let tracker = self.input_event_tracker.lock();
        return tracker.borrow().key_state(code);
    }

    pub fn is_key_pressed(&self, code: i32) -> bool {
        self.get_in_key_state(code) > 0
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_LEFTALT) || self.is_key_pressed(ec::KEY_RIGHTALT)
    }

    pub fn is_control_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_LEFTCTRL) || self.is_key_pressed(ec::KEY_RIGHTCTRL)
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_LEFTSHIFT) || self.is_key_pressed(ec::KEY_RIGHTSHIFT)
    }

    pub fn is_winkey_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_LEFTMETA) || self.is_key_pressed(ec::KEY_RIGHTMETA)
    }

    pub fn is_esc_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_ESC)
    }

    pub fn is_capslock_pressed(&self) -> bool {
        self.is_key_pressed(ec::KEY_CAPSLOCK)
    }

    pub fn get_out_key_state(&self, code: i32) -> i32 {
        self.uinput.as_ref().unwrap().key_state(code)
    }

    pub fn is_out_key_pressed(&self, code: i32) -> bool {
        self.get_out_key_state(code) > 0
    }

    fn reset_out_modifiers(&self) {
        for key in MODIFIER_KEYS.iter() {
            self.send_key_event(*key, 0);
        }
        self.send_syn_report();
    }

    fn save_out_modifier_state(&self) -> ModifierState {
        let mut ret = [false; MODIFIER_COUNT];
        for (i, key) in MODIFIER_KEYS.iter().enumerate() {
            ret[i] = self.is_out_key_pressed(*key);
        }
        ret
    }

    fn restore_out_modifier_state(&self, state: ModifierState) {
        for (i, value) in state.iter().enumerate() {
            self.send_key_event(MODIFIER_KEYS[i], if *value { 1 } else { 0 });
        }
        self.send_syn_report();
    }

    pub fn press_key(&self, code: i32, modifiers: &str) {
        self.ensure_uinput();
        let _ = self.uinput.as_ref().unwrap().lock();
        if modifiers == "*" {
            // Simple case -- don't reset the modifiers.
            self.send_key_events(&[(code, 1), (code, 0)]);
            return;
        }

        KeyRemapper::validate_modifiers(modifiers, "acsw");

        // Save the pressed modifier state and release them all.
        let out_modifier_state = self.save_out_modifier_state();
        self.reset_out_modifiers();

        // Then press the requested modifiers, and the actual key.
        if modifiers.contains('a') {
            self.send_key_event(ec::KEY_LEFTALT, 1);
        }
        if modifiers.contains('c') {
            self.send_key_event(ec::KEY_LEFTCTRL, 1);
        }
        if modifiers.contains('s') {
            self.send_key_event(ec::KEY_LEFTSHIFT, 1);
        }
        if modifiers.contains('w') {
            self.send_key_event(ec::KEY_LEFTMETA, 1);
        }

        self.send_key_events(&[(code, 1), (code, 0)]);

        // Restore the modifiers.
        self.restore_out_modifier_state(out_modifier_state);
    }

    pub fn are_modifiers_pressed(&self, modifiers: &str) -> bool {
        KeyRemapper::validate_modifiers(modifiers, "acswep*");

        let ignore_other_modifiers = modifiers.contains('*');
        let alt = modifiers.contains('a');
        let ctrl = modifiers.contains('c');
        let shift = modifiers.contains('s');
        let win = modifiers.contains('w');
        let esc = modifiers.contains('e'); // Allow ESC to be used as a modifier.
        let caps = modifiers.contains('p'); // Allow CAPS to be used as a modifier.

        if self.is_alt_pressed() != alt && (alt || !ignore_other_modifiers) {
            return false;
        }

        if self.is_control_pressed() != ctrl && (ctrl || !ignore_other_modifiers) {
            return false;
        }

        if self.is_shift_pressed() != shift && (shift || !ignore_other_modifiers) {
            return false;
        }

        if self.is_winkey_pressed() != win && (win || !ignore_other_modifiers) {
            return false;
        }

        if self.is_esc_pressed() != esc && (esc || !ignore_other_modifiers) {
            return false;
        }

        if self.is_capslock_pressed() != caps && (caps || !ignore_other_modifiers) {
            return false;
        }
        return true;
    }

    fn validate_modifiers(in_modifiers: &str, valid_modifiers: &str) {
        for m in in_modifiers.chars() {
            let mut ok = false;
            for v in valid_modifiers.chars() {
                if m == v {
                    ok = true;
                    break;
                }
            }
            if !ok {
                panic!(
                    r#"Modifier "{}" contains an invalid character "{}""#,
                    in_modifiers, m
                );
            }
        }
    }

    pub fn key_pressed(
        &self,
        event: &evdev::InputEvent,
        keys: &[i32],
        values: &[i32],
        modifiers: &str,
    ) -> bool {
        (event.event_type == ec::EventType::EV_KEY)
            && keys.contains(&event.code)
            && values.contains(&event.value)
            && self.are_modifiers_pressed(modifiers)
    }

    // TODO Support changing the tray icon.
    // TODO Support adding menu items.
}

#[test]
fn test_check_modifiers_ok() {
    KeyRemapper::validate_modifiers("", "a");
    KeyRemapper::validate_modifiers("a", "a");

    KeyRemapper::validate_modifiers("", "sa");
    KeyRemapper::validate_modifiers("a", "sa");
    KeyRemapper::validate_modifiers("s", "sa");
    KeyRemapper::validate_modifiers("as", "sa");
}

#[test]
#[should_panic(expected = r#"Modifier "c" contains an invalid character "c""#)]
fn test_check_modifiers_fail_1() {
    KeyRemapper::validate_modifiers("c", "sa");
}
#[test]
#[should_panic(expected = r#"Modifier "sca" contains an invalid character "c""#)]
fn test_check_modifiers_fail_2() {
    KeyRemapper::validate_modifiers("sca", "sa");
}

fn select(fds: &Vec<RawFd>) -> Result<RawFd> {
    if fds.len() == 0 {
        panic!("fds can't be empty");
    }
    unsafe {
        loop {
            let mut fd_set = FdSet::new();
            for fd in fds {
                fd_set.set(*fd);
            }
            let max = fds.iter().max().unwrap();

            let mut sigmask: libc::sigset_t = MaybeUninit::zeroed().assume_init();
            libc::sigemptyset(&mut sigmask as *mut libc::sigset_t);

            pselect(max + 1, Some(&mut fd_set), None, None, None, Some(&sigmask))?;
            for i in 0..(max + 1) {
                if fd_set.is_set(i) {
                    return Ok(i);
                }
            }
            log::warn!("No fds selected after pselect()!");
        }
    }
}

/// Main loop, which runs on the I/O thread.
fn main_loop(key_remapper: &KeyRemapper) {
    let config = &key_remapper.config;
    let callbacks = config.callbacks_cloned();

    (*callbacks.on_start)(&key_remapper);

    let udev = UdevMonitor::new("input").expect("Udev setup failed");
    let udev_fd = udev.udev_fd();

    let mut rng = rand::thread_rng();

    'with_device_detection: loop {
        // First, find the target input devices.
        let input_lock = key_remapper.input.lock();
        let mut input = input_lock.borrow_mut();
        input
            .refresh_devices()
            .expect("Unable to detect input devices");

        if input.devices.len() == 0 {
            log::info!("No device found");
            key_remapper.show_notiication("No device found");
            (*callbacks.on_devices_not_found)(&key_remapper);
        } else {
            let mut message = "Device(s) detected".to_string();
            log::info!("{}", message);

            for device in &input.devices {
                message.push_str("\n - ");
                message.push_str(&device.name());
            }
            key_remapper.show_notiication(&message);

            (*callbacks.on_devices_detected)(&key_remapper, &input.devices);
        }

        // Create a list of FDs to select from.
        let mut fds = vec![udev_fd];

        for device in &input.devices {
            fds.push(device.device_fd());
        }

        // Actual event loop.
        'event_loop: loop {
            let ready_fd = select(&fds).expect("pselect() failed");

            // Handle udev events
            if ready_fd == udev_fd {
                let event = udev.next_event().expect("Unable to read udev events");

                // Ignore events for uinput device creation.
                if event.path.starts_with("/devices/virtual/") {
                    continue 'event_loop;
                }

                let msg = "Devices connected or disconnected";
                log::info!("{}", msg);
                key_remapper.show_notiication(msg);
                (*callbacks.on_devices_lost)(&key_remapper);

                input.release_devices(); // Close all the input devices.

                key_remapper.reset_out();

                thread::sleep(Duration::from_millis(rng.gen_range(1000..2000)));

                udev.drain_events();

                continue 'with_device_detection;
            }

            // Handle input events.

            let device = input.find_device_by_fd(ready_fd);
            let events = match device.next_events() {
                Ok(event) => event,
                Err(_) => {
                    log::warn!("Unable to read event; device closed?");
                    continue;
                }
            };

            for ev in &events {
                log::debug!("Input event: {}", ev);
            }

            (*callbacks.on_events_batch)(&key_remapper, &device, &events);
            for ev in &events {
                {
                    let tracker = key_remapper.input_event_tracker.lock();
                    tracker.borrow_mut().on_event_sent(ev);
                }
                (*callbacks.on_event)(&key_remapper, &device, ev);
            }
        }
    }
}

pub fn handle_args(config: &mut KeyRemapperConfiguration) {
    let device_name_regex = config.device_name_regex.clone();
    let id_regex = config.id_regex.clone();

    let mut app = App::new(&config.name)
        .arg(
            Arg::with_name("device_name_regex")
                .short("d")
                .long("match-device-name")
                .value_name("DEVICE")
                .default_value(&device_name_regex)
                .help(
                    r#"Select by device name using this regex. Use evtest(1) to list device names"#,
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("id_regex")
                .short("i")
                .long("match-id")
                .value_name("ID")
                .default_value(&id_regex)
                .help(r#"Select by vendor/product ID, in "vXXXX pXXXX" format, using this regex"#)
                .takes_value(true),
        );

    let callbacks = config.callbacks_cloned();
    app = (*callbacks.on_init_args)(app);

    let matches = app.get_matches();

    config.device_name_regex = matches.value_of("device_name_regex").unwrap().to_string();
    config.id_regex = matches.value_of("id_regex").unwrap().to_string();

    (*callbacks.on_args_parsed)(&matches);

    config.update_regexes();
}

fn setup_panic_hook() {
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));
}

/// Entry point.
pub fn start(mut config: KeyRemapperConfiguration) {
    config.set_defaults();

    setup_panic_hook();

    ensure_singleton(&config.global_lock_name);
    gtk::init().unwrap();

    handle_args(&mut config);

    let name = config.name.clone();
    log::info!("KeyRemapper started for {}", name);
    log::debug!("Config={:#?}", config);

    let key_remapper = KeyRemapper::new(config.clone());

    thread::Builder::new()
        .name(format!("keyremapper-{}-io", name))
        .spawn(move || {
            log::debug!("I/O thread started...");

            main_loop(&key_remapper);
        })
        .expect("Unable to start I/O thread");

    gtk::main();

    log::info!("KeyRemapper stopping for {}", name);
}
