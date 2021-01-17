//! Keyboard remapper for https://smile.amazon.com/gp/product/B01NC2LEYP
extern crate lazy_static;

use std::{cell::RefCell, error::Error, process, sync::Arc, time::Duration};

use clap::{value_t, Arg};
use keyremapper::{
    evdev::{self, ec},
    res::get_gio_resource_as_file,
    KeyRemapper, KeyRemapperConfiguration,
};
use parking_lot::ReentrantMutex;

const NAME: &str = "Shortcut-Remote-Remapper";
const DEVICE_RE: &str = r#"^UGEE TABLET TABLET KT01$"#;
const ID_RE: &str = "^";

const KEYS: &'static [i32] = &[
    ec::KEY_M,
    ec::KEY_P,
    ec::KEY_U,
    ec::KEY_B,
    ec::KEY_ENTER,
    ec::KEY_Z,
    ec::KEY_V,
    ec::KEY_I,
    ec::KEY_SPACE,
    ec::KEY_KPMINUS,
    ec::KEY_KPPLUS,
    ec::KEY_LEFTSHIFT,
];

const KEY_LABELS: &'static [&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "Left", "Right", "Button"];

fn find_key_index(key: i32) -> i32 {
    for (i, k) in KEYS.iter().enumerate() {
        if *k == key {
            return i as i32;
        }
    }
    return -1;
}

const HALF_TOGGLE: i32 = 0x1000;
const MODE1: (i32, &str) = (-1, "#Cursor mode");
const MODE2: (i32, &str) = (-2, "#Volume mode");
const MODE3: (i32, &str) = (-3, "#Scroll mode");

static CURSOR_MODE: &'static [(i32, &str)] = &[
    (ec::KEY_F, "F"),
    (ec::KEY_F11, "F11"),
    (ec::KEY_ENTER, "Enter"),
    (ec::KEY_VOLUMEDOWN, "Vol Down"),
    (ec::KEY_MUTE, "Mute"),
    (ec::KEY_VOLUMEUP, "Vol Up"),
    MODE1,
    MODE2,
    MODE3,
    (ec::KEY_LEFT, "Left"),
    (ec::KEY_RIGHT, "Right"),
    (ec::KEY_SPACE, "Space"),
];

static VOLUME_MODE: &'static [(i32, &str)] = &[
    (ec::KEY_F20, "Mic Mute"),
    (0, ""),
    (ec::KEY_F20 | HALF_TOGGLE, "Mic Mute PPT"),
    (ec::KEY_LEFT, "Left"),
    (ec::KEY_ENTER, "Enter"),
    (ec::KEY_RIGHT, "Right"),
    MODE1,
    MODE2,
    MODE3,
    (ec::KEY_VOLUMEDOWN, "Vol Down"),
    (ec::KEY_VOLUMEUP, "Vol Up"),
    (ec::KEY_MUTE, "Mute"),
];

static SCROLL_MODE: &'static [(i32, &str)] = &[
    (ec::KEY_BACK, "Back"),
    (ec::KEY_DOWN, "Down"),
    (ec::KEY_ENTER, "Enter"),
    (ec::KEY_LEFT, "Left"),
    (ec::KEY_UP, "Up"),
    (ec::KEY_RIGHT, "Right"),
    MODE1,
    MODE2,
    MODE3,
    (ec::KEY_PAGEUP, "Page Down"),
    (ec::KEY_PAGEDOWN, "Page Up"),
    (ec::KEY_SPACE, "Space"),
];

static ALL_MODES: &'static [&[(i32, &str)]] = &[CURSOR_MODE, VOLUME_MODE, SCROLL_MODE];

struct Remapper {
    mode: usize,
}

impl Remapper {
    fn new() -> Remapper {
        Remapper { mode: 0 }
    }

    fn show_help(&self, km: &KeyRemapper) {
        let mode = ALL_MODES[self.mode];
        let mut body = "".to_string();
        for (i, (_, desc)) in mode.iter().enumerate() {
            if i > 0 {
                body.push_str(&"\n");
            }
            body.push_str(KEY_LABELS[i]);
            body.push_str(":  ");
            body.push_str(desc)
        }
        km.show_notiication_with_timeout(&body, Duration::from_secs(5));
    }

    fn on_start(&self, km: &KeyRemapper) {
        self.show_help(km);
    }

    fn remap(&mut self, km: &KeyRemapper, _device: &evdev::EvdevDevice, ev: &evdev::InputEvent) {
        // Ignore non-key events.
        if ev.event_type != ec::EventType::EV_KEY {
            return;
        }

        // Ignore CTRL events, which the "6" key would send along with "z".
        if ev.code == ec::KEY_LEFTCTRL {
            return;
        }

        // Find the key index.
        let key_index = find_key_index(ev.code);
        if key_index < 0 {
            log::warn!("Unknown key detected: {}", ev);
            return;
        }

        // Find the "to" key.
        let mode = ALL_MODES[self.mode];
        let (to_key, ..) = mode[key_index as usize];

        // Change mode?
        if to_key < 0 {
            let new_mode = (-to_key as usize) - 1;
            log::info!("Chaning mode to {}", new_mode);
            self.mode = new_mode;
            self.show_help(km);
            return;
        }

        // Otherwise, convert to a key event.
        let half_toggle = (to_key & HALF_TOGGLE) != 0;
        let to_key = to_key & !HALF_TOGGLE;

        if half_toggle {
            if !(ev.value == 0 || ev.value == 1) {
                return;
            }
        } else {
            if ev.value == 0 {
                return;
            }
        }
        km.press_key(to_key, "");
    }
}

lazy_static::lazy_static! {
    static ref REMAPPER: Arc<ReentrantMutex<RefCell<Remapper>>> =
        Arc::new(ReentrantMutex::new(RefCell::new(Remapper::new())));
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Prepare the icon.
    let icon = get_gio_resource_as_file(NAME, "/keyremapper/resources/10key.png", &|| {
        let data = glib::Bytes::from(include_bytes!("icons.bin"));
        return gio::Resource::from_data(&data).unwrap();
    });

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(icon)
        .set_id_regex(ID_RE)
        .set_grab(true)
        .set_use_non_keyboard(true)
        .set_write_to_uinput(true);

    config.on_init_args(|app| {
        return app.arg(
            Arg::with_name("initial_mode")
                .short("m")
                .long("mode")
                .value_name("MODE")
                .default_value(&"0")
                .help(r#"Select initial mode from 0: Cursor mode 1: Volume mode 2: Scroll mode"#)
                .takes_value(true),
        );
    });
    config.on_args_parsed(|matches| {
        let mode = value_t!(matches.value_of("initial_mode"), usize).unwrap_or_else(|e| e.exit());
        if mode > ALL_MODES.len() {
            eprintln!("Initial mode must be between 0..{}", ALL_MODES.len());
            process::exit(1);
        }
        REMAPPER.lock().borrow_mut().mode = mode;
        log::debug!("Initial mode={}", mode);
    });

    config.on_start(|km| {
        log::debug!("{}.on_start", NAME);
        REMAPPER.lock().borrow().on_start(km);
    });
    config.on_stop(|_| {
        log::debug!("{}.on_stop", NAME);
    });

    config.on_devices_not_found(|_| {
        log::debug!("{}.on_devices_not_found", NAME);
    });
    config.on_devices_detected(|_, devices| {
        log::debug!("{}.on_devices_detected: {:?}", NAME, devices);
    });

    config.on_devices_lost(|_| {
        log::debug!("{}.on_init_args", NAME);
    });

    config.on_event(|km, device, event| {
        REMAPPER.lock().borrow_mut().remap(km, device, event);
    });

    keyremapper::start(config);

    return Ok(());
}

// # 1
// # Event: time 1605849332.729321, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70010
// # Event: time 1605849332.729321, type 1 (EV_KEY), code 50 (KEY_M), value 1
// # Event: time 1605849332.729321, -------------- SYN_REPORT ------------
// # Event: time 1605849332.825196, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70010
// # Event: time 1605849332.825196, type 1 (EV_KEY), code 50 (KEY_M), value 0
// # Event: time 1605849332.825196, -------------- SYN_REPORT ------------
// #
// # 2
// # Event: time 1605849333.141265, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70013
// # Event: time 1605849333.141265, type 1 (EV_KEY), code 25 (KEY_P), value 1
// # Event: time 1605849333.141265, -------------- SYN_REPORT ------------
// # Event: time 1605849333.229098, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70013
// # Event: time 1605849333.229098, type 1 (EV_KEY), code 25 (KEY_P), value 0
// # Event: time 1605849333.229098, -------------- SYN_REPORT ------------
// #
// # 3
// # Event: time 1605849333.425338, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70018
// # Event: time 1605849333.425338, type 1 (EV_KEY), code 22 (KEY_U), value 1
// # Event: time 1605849333.425338, -------------- SYN_REPORT ------------
// # Event: time 1605849333.533252, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70018
// # Event: time 1605849333.533252, type 1 (EV_KEY), code 22 (KEY_U), value 0
// # Event: time 1605849333.533252, -------------- SYN_REPORT ------------
// #
// # 4
// # Event: time 1605849334.165310, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70005
// # Event: time 1605849334.165310, type 1 (EV_KEY), code 48 (KEY_B), value 1
// # Event: time 1605849334.165310, -------------- SYN_REPORT ------------
// # Event: time 1605849334.257216, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70005
// # Event: time 1605849334.257216, type 1 (EV_KEY), code 48 (KEY_B), value 0
// # Event: time 1605849334.257216, -------------- SYN_REPORT ------------
// #
// # 5
// # Event: time 1605849334.525236, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70028
// # Event: time 1605849334.525236, type 1 (EV_KEY), code 28 (KEY_ENTER), value 1
// # Event: time 1605849334.525236, -------------- SYN_REPORT ------------
// # Event: time 1605849334.592810, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70028
// # Event: time 1605849334.592810, type 1 (EV_KEY), code 28 (KEY_ENTER), value 0
// # Event: time 1605849334.592810, -------------- SYN_REPORT ------------
// #
// # 6
// # Event: time 1605849334.921274, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849334.921274, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 1
// # Event: time 1605849334.921274, type 4 (EV_MSC), code 4 (MSC_SCAN), value 7001d
// # Event: time 1605849334.921274, type 1 (EV_KEY), code 44 (KEY_Z), value 1
// # Event: time 1605849334.921274, -------------- SYN_REPORT ------------
// #
// # 7
// # Event: time 1605849364.224065, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70019
// # Event: time 1605849364.224065, type 1 (EV_KEY), code 47 (KEY_V), value 1
// # Event: time 1605849364.224065, -------------- SYN_REPORT ------------
// # Event: time 1605849364.324053, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70019
// # Event: time 1605849364.324053, type 1 (EV_KEY), code 47 (KEY_V), value 0
// # Event: time 1605849364.324053, -------------- SYN_REPORT ------------
// #
// # 8
// # Event: time 1605849364.732044, type 4 (EV_MSC), code 4 (MSC_SCAN), value 7000c
// # Event: time 1605849364.732044, type 1 (EV_KEY), code 23 (KEY_I), value 1
// # Event: time 1605849364.732044, -------------- SYN_REPORT ------------
// # iEvent: time 1605849364.847988, type 4 (EV_MSC), code 4 (MSC_SCAN), value 7000c
// # Event: time 1605849364.847988, type 1 (EV_KEY), code 23 (KEY_I), value 0
// # Event: time 1605849364.847988, -------------- SYN_REPORT ------------
// #
// # 9
// # Event: time 1605849365.024025, type 4 (EV_MSC), code 4 (MSC_SCAN), value 7002c
// # Event: time 1605849365.024025, type 1 (EV_KEY), code 57 (KEY_SPACE), value 1
// # Event: time 1605849365.024025, -------------- SYN_REPORT ------------
// # Event: time 1605849365.139992, type 4 (EV_MSC), code 4 (MSC_SCAN), value 7002c
// # Event: time 1605849365.139992, type 1 (EV_KEY), code 57 (KEY_SPACE), value 0
// #
// #
// # left turn
// # Event: time 1605849413.746328, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849413.746328, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 1
// # Event: time 1605849413.746328, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70056
// # Event: time 1605849413.746328, type 1 (EV_KEY), code 74 (KEY_KPMINUS), value 1
// # Event: time 1605849413.746328, -------------- SYN_REPORT ------------
// # Event: time 1605849413.749899, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849413.749899, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 0
// # Event: time 1605849413.749899, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70056
// # Event: time 1605849413.749899, type 1 (EV_KEY), code 74 (KEY_KPMINUS), value 0
// # Event: time 1605849413.749899, -------------- SYN_REPORT ------------
// #
// #
// # right turn
// # Event: time 1605849414.946283, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849414.946283, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 1
// # Event: time 1605849414.946283, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70057
// # Event: time 1605849414.946283, type 1 (EV_KEY), code 78 (KEY_KPPLUS), value 1
// # Event: time 1605849414.946283, -------------- SYN_REPORT ------------
// # Event: time 1605849414.949907, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849414.949907, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 0
// # Event: time 1605849414.949907, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70057
// # Event: time 1605849414.949907, type 1 (EV_KEY), code 78 (KEY_KPPLUS), value 0
// # Event: time 1605849414.949907, -------------- SYN_REPORT ------------
// # Event: time 1605849415.758268, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849415.758268, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 1
// # Event: time 1605849415.758268, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70057
// # Event: time 1605849415.758268, type 1 (EV_KEY), code 78 (KEY_KPPLUS), value 1
// # Event: time 1605849415.758268, -------------- SYN_REPORT ------------
// # Event: time 1605849415.761837, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e0
// # Event: time 1605849415.761837, type 1 (EV_KEY), code 29 (KEY_LEFTCTRL), value 0
// # Event: time 1605849415.761837, type 4 (EV_MSC), code 4 (MSC_SCAN), value 70057
// # Event: time 1605849415.761837, type 1 (EV_KEY), code 78 (KEY_KPPLUS), value 0
// # Event: time 1605849415.761837, -------------- SYN_REPORT ------------
// #
// # center
// # Event: time 1605849415.789813, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e1
// # Event: time 1605849415.789813, type 1 (EV_KEY), code 42 (KEY_LEFTSHIFT), value 1
// # Event: time 1605849415.789813, -------------- SYN_REPORT ------------
// # Event: time 1605849415.910189, type 4 (EV_MSC), code 4 (MSC_SCAN), value 700e1
// # Event: time 1605849415.910189, type 1 (EV_KEY), code 42 (KEY_LEFTSHIFT), value 0
// # Event: time 1605849415.910189, -------------- SYN_REPORT ------------

// # wheel
// # Event: time 1606370588.253422, type 2 (EV_REL), code 8 (REL_WHEEL), value -1
// # Event: time 1606370588.253422, type 2 (EV_REL), code 11 (REL_WHEEL_HI_RES), value -120
