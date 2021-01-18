//! Reampper for the main keyboard
extern crate lazy_static;

use std::{cell::RefCell, error::Error, sync::Arc, time::Duration};

use ec::EventType;
use keyremapper::{
    evdev::{self, ec},
    res::get_gio_resource_as_file,
    KeyRemapper, KeyRemapperConfiguration,
};

use parking_lot::Mutex;

const NAME: &str = "Keyboard remapper";

// AT Translated Set 2 keyboard -> thinkpad internal keyboard
// Topre Corporation Realforce  -> Realforce
// P. I. Engineering XK-16 HID  -> An external 8-key keyboard
const DEVICE_RE: &str = r#"^(AT Translated Set 2 keyboard|Topre Corporation Realforce|P. I. Engineering XK-16 HID)"#;
const ID_RE: &str = "^";

// ESC + These keys will generate SHIFT+ALT+CTRL+META+[THE KEY]. I launch apps using them -- e.g. ESC+ENTER to launch
// Chrome.
static VERSATILE_KEYS: &[i32] = &[
    ec::KEY_F1,
    ec::KEY_F2,
    ec::KEY_F3,
    ec::KEY_F4,
    ec::KEY_F5,
    ec::KEY_F6,
    ec::KEY_F7,
    ec::KEY_F8,
    ec::KEY_F9,
    ec::KEY_F10,
    ec::KEY_F11,
    ec::KEY_F12,
    ec::KEY_ENTER,
];

static ALPHABET_KEYS: &[i32] = &[
    ec::KEY_A,
    ec::KEY_B,
    ec::KEY_C,
    ec::KEY_D,
    ec::KEY_E,
    ec::KEY_F,
    ec::KEY_G,
    ec::KEY_H,
    ec::KEY_I,
    ec::KEY_J,
    ec::KEY_K,
    ec::KEY_L,
    ec::KEY_M,
    ec::KEY_N,
    ec::KEY_O,
    ec::KEY_P,
    ec::KEY_Q,
    ec::KEY_R,
    ec::KEY_S,
    ec::KEY_T,
    ec::KEY_U,
    ec::KEY_V,
    ec::KEY_W,
    ec::KEY_X,
    ec::KEY_Y,
    ec::KEY_Z,
];

static MODIFIER_KEYS: &[i32] = &[
    ec::KEY_LEFTALT,
    ec::KEY_RIGHTALT,
    ec::KEY_LEFTCTRL,
    ec::KEY_RIGHTCTRL,
    ec::KEY_LEFTSHIFT,
    ec::KEY_RIGHTSHIFT,
    ec::KEY_LEFTMETA,
    ec::KEY_RIGHTMETA,
    ec::KEY_ESC, // In this remapper, ESC is used as a modifier.
];

const WHEEL_REPEAT_DELAY_NORMAL: Duration = Duration::from_millis(20);
const WHEEL_REPEAT_DELAY_FAST: Duration = Duration::from_millis(5);
const WHEEL_MAKE_FAST_AFTER_THIS_MANY_EVENTS: u32 = 10;

mod wheeler {
    use std::{cell::RefCell, sync::Arc, thread};

    use keyremapper::evdev::{InputEvent, ec, uinput::SyncedUinput};
    use parking_lot::{Condvar, Mutex};

    use crate::{WHEEL_MAKE_FAST_AFTER_THIS_MANY_EVENTS, WHEEL_REPEAT_DELAY_FAST, WHEEL_REPEAT_DELAY_NORMAL};

    #[derive(Debug)]
    struct Inner {
        vwheel_speed: i32,
        hwheel_speed: i32,
    }

    #[derive(Debug, Clone)]
    pub struct Wheeler {
        inner: Arc<Mutex<RefCell<Inner>>>,
        uinput: Arc<SyncedUinput>,
        cond: Arc<Condvar>,
    }

    impl Wheeler {
        pub fn new(uinput: SyncedUinput) -> Wheeler {
            let inner = Inner {
                vwheel_speed: 0,
                hwheel_speed: 0,
            };

            return Wheeler {
                inner: Arc::new(Mutex::new(RefCell::new(inner))),
                uinput: Arc::new(uinput),
                cond: Arc::new(Condvar::new()),
            };
        }

        fn with_lock<F>(&self, callback: F)
        where
            F: Fn(&mut Inner),
        {
            let inner = self.inner.lock();
            callback(&mut inner.borrow_mut());
            self.cond.notify_one();
        }

        pub fn reset(&self) {
            self.with_lock(|inner| {
                // log::debug!("Wheel reset");
                inner.vwheel_speed = 0;
                inner.hwheel_speed = 0;
            });
        }

        pub fn set_vwheel(&mut self, value: i32) {
            self.with_lock(|inner| {
                // log::debug!("Wheel v -> {}", 0);
                inner.vwheel_speed = value;
            });
        }

        pub fn set_hwheel(&mut self, value: i32) {
            self.with_lock(|inner| {
                // log::debug!("Wheel h -> {}", value);
                inner.hwheel_speed = value;
            });
        }

        pub fn start(&self) {
            let clone = self.clone();

            thread::Builder::new()
                .name(format!("{}-wheeler", super::NAME))
                .spawn(move || {
                    clone.thread_main();
                })
                .expect("Unable to wheeler thread");
        }

        fn thread_main(&self) {
            log::info!("Wheeler thread started...");

            let mut consecutive_event_count = 0;
            loop {
                let mut v = 0;
                let mut h = 0;
                loop {
                    let mut inner = self.inner.lock();
                    {
                        let i = inner.borrow();
                        v = i.vwheel_speed;
                        h = i.hwheel_speed;
                    }
                    if v == 0 && h == 0 {
                        // log::debug!("Wheel stop");
                        consecutive_event_count = 0;
                        self.cond.wait(&mut inner);
                    } else {
                        break;
                    }
                }
                consecutive_event_count += 1;
                // log::debug!("WHEEL! {} {} @ {}", v, h, consecutive_event_count);
                if v != 0 {
                    self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_WHEEL, v)).unwrap();
                    self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_WHEEL_HI_RES, v * 120)).unwrap();
                }
                if h != 0 {
                    self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_HWHEEL, h)).unwrap();
                    self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_HWHEEL_HI_RES, h * 120)).unwrap();
                }

                let wait = if consecutive_event_count >= WHEEL_MAKE_FAST_AFTER_THIS_MANY_EVENTS {
                    WHEEL_REPEAT_DELAY_FAST
                } else {
                    WHEEL_REPEAT_DELAY_NORMAL
                };
                thread::sleep(wait);
            }
        }
    }
}

#[derive(Debug, Default)]
struct State {
    pending_esc_pressed: bool,
    wheeler: Option<wheeler::Wheeler>,
}

impl State {}

lazy_static::lazy_static! {
    static ref STATE: Arc<Mutex<RefCell<State>>> = Arc::new(Mutex::new(RefCell::new(State::default())));
    // static ref STATE: State = State::default();
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Prepare the icon.
    let icon = get_gio_resource_as_file(NAME, "/keyremapper/resources/keyboard.png", &|| {
        let data = glib::Bytes::from(include_bytes!("icons.bin"));
        return gio::Resource::from_data(&data).unwrap();
    });

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(icon)
        .set_id_regex(ID_RE)
        .set_use_non_keyboard(false)
        .set_grab(true)
        .set_write_to_uinput(true);

    config.on_start(|km| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        let wheeler = wheeler::Wheeler::new(km.create_mouse_uinput("-wheel"));
        wheeler.start();
        state.wheeler = Some(wheeler);
    });

    config.on_devices_lost(|_km| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        state.wheeler.as_mut().unwrap().reset();
    });

    config.on_event(|km, device, ev| {
        if ev.event_type != EventType::EV_KEY {
            return; // Ignore non-key events.
        }

        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        let is_thinkpad = device.name().starts_with("AT");
        let is_xkeys = device.name().starts_with("P. I.");

        // For x-keys. Convert to Shift+Ctrl+[number]
        if is_xkeys {
            // Special casing the first two keys.
            match ev {
                _ if km.key_pressed(ev, &[ec::KEY_1], &[1], "") => km.press_key(ec::KEY_LEFT, "a"),
                _ if km.key_pressed(ev, &[ec::KEY_2], &[1], "") => km.press_key(ec::KEY_LEFT, "a"),
                _ if ev.value == 1 => km.press_key(ev.value, "sacw"),
                _ => log::warn!("Unexpected event {}", ev),
            }
            return;
        }

        let mut ev = &mut ev.clone();

        // Special for the thinkpad keyboard. Use INS/DEL as PAGEUP/DOWN, unless caps is pressed.
        if is_thinkpad && !km.is_key_pressed(ec::KEY_CAPSLOCK) {
            match ev.code {
                ec::KEY_INSERT => ev.code = ec::KEY_PAGEUP,
                ec::KEY_DELETE => ev.code = ec::KEY_PAGEDOWN,
                _ => {}
            }
        }

        // Special ESC handling: Don't send "ESC-press" on key-down, but instead send it on key-*up*, unless
        // any keys are pressed between the down and up.
        // This allows to make "ESC + BACKSPACE" act as a DEL press without sending ESC.
        if ev.code == ec::KEY_ESC {
            if ev.value == 1 {
                state.pending_esc_pressed = true;
            }
            if ev.value == 0 && state.pending_esc_pressed {
                state.pending_esc_pressed = false;
                km.press_key(ec::KEY_ESC, "");
            }
            return;
        }

        // If other keys (than ESC) are pressed, clear pending_esc_pressed, but don't do so on modifier key presses, in order to
        // allow combos like "ESC+ctrl+Backspace".
        if !MODIFIER_KEYS.contains(&ev.code) {
            state.pending_esc_pressed = false;
        }

        match ev {
            // ESC or shift + backspace -> delete
            _ if km.key_pressed(ev, &[ec::KEY_BACKSPACE], &[1, 2], "e") => km.press_key(ec::KEY_DELETE, ""),
            _ if km.key_pressed(ev, &[ec::KEY_BACKSPACE], &[1, 2], "s") => km.press_key(ec::KEY_DELETE, ""),

            // See VERSATILE_KEYS.
            _ if km.key_pressed(ev, VERSATILE_KEYS, &[1, 2], "e") => km.press_key(ev.code, "acsw"),

            // ESC + home/end -> ATL+Left/Right (back / forward)
            _ if km.key_pressed(ev, &[ec::KEY_HOME], &[1, 2], "e") => km.press_key(ec::KEY_LEFT, "a"),
            _ if km.key_pressed(ev, &[ec::KEY_END], &[1, 2], "e") => km.press_key(ec::KEY_RIGHT, "a"),

            // ESC + Pageup -> ctrl + pageup (prev tab)
            // ESC + Pagedown -> ctrl + pagedown (next tab)
            // (meaning ESC + ins/del act as them too on thinkpad.)
            _ if km.key_pressed(ev, &[ec::KEY_PAGEUP, ec::KEY_PAGEDOWN], &[1, 2], "e") => km.press_key(ev.code, "c"),

            // ESC + caps lock -> caps lock, in case I ever need it.
            _ if km.key_pressed(ev, &[ec::KEY_CAPSLOCK], &[1, 2], "e*") => km.press_key(ec::KEY_CAPSLOCK, "c"),

            // ESC + H / J / K / L -> emulate wheel. Also support ESC+SPACE / C for left-hand-only scrolling.
            _ if km.key_pressed(ev, &[ec::KEY_J, ec::KEY_K, ec::KEY_SPACE, ec::KEY_C], &[0, 1, 2], "e*") => {
                if ev.value == 0 {
                    state.wheeler.as_mut().unwrap().set_vwheel(0);
                } else if ev.value == 1 {
                    if [ec::KEY_K, ec::KEY_C].contains(&ev.code) {
                        state.wheeler.as_mut().unwrap().set_vwheel(1);
                    } else if [ec::KEY_J, ec::KEY_SPACE].contains(&ev.code) {
                        state.wheeler.as_mut().unwrap().set_vwheel(-1);
                    }
                }
                return;
            }
            _ if km.key_pressed(ev, &[ec::KEY_L, ec::KEY_H], &[0, 1, 2], "e*") => {
                if ev.value == 0 {
                    state.wheeler.as_mut().unwrap().set_hwheel(0);
                } else if ev.value == 1 {
                    if [ec::KEY_L].contains(&ev.code) {
                        state.wheeler.as_mut().unwrap().set_hwheel(1);
                    } else if [ec::KEY_H].contains(&ev.code) {
                        state.wheeler.as_mut().unwrap().set_hwheel(-1);
                    }
                }
                return;
            }

            // ESC + other alphabet -> ctrl + shift + the key.
            _ if km.key_pressed(ev, ALPHABET_KEYS, &[1, 2], "e") => km.press_key(ev.code, "cs"),

            // Don't use capslock alone.
            _ if ev.code == ec::KEY_CAPSLOCK => return,

            // Default: Just send the original key event.
            _ => km.send_event(&ev),
        };
    });

    keyremapper::start(config);

    return Ok(());
}
