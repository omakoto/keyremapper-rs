//! Reampper for the main keyboard
extern crate lazy_static;

use std::{cell::RefCell, error::Error, sync::Arc};

use ec::EventType;
use evdev::uinput::SyncedUinput;
use keyremapper::{
    evdev::{self, ec, EventsDescriptor, InputEvent},
    res::get_gio_resource_as_file,
    KeyRemapper, KeyRemapperConfiguration,
};

use parking_lot::Mutex;

const NAME: &str = "Keyboard remapper";

// AT Translated Set 2 keyboard -> thinkpad internal keyboard
// Topre Corporation Realforce  -> Realforce
// P. I. Engineering XK-16 HID  -> An external 8-key keyboard
const DEVICE_RE: &str =
    r#"^(AT Translated Set 2 keyboard|Topre Corporation Realforce|P. I. Engineering XK-16 HID)"#;
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

#[derive(Debug, Default)]
struct Wheeler {
    uinput: Option<SyncedUinput>,
    vwheel_speed: i32,
    hwheel_speed: i32,
}

impl Wheeler {
    fn new(uinput: SyncedUinput) -> Wheeler {
        let mut ret = Wheeler::default();
        ret.uinput = Some(uinput);

        return ret;
    }

    fn start(&mut self) {}

    fn reset(&mut self) {
        self.vwheel_speed = 0;
        self.hwheel_speed = 0;
    }
}

#[derive(Debug, Default)]
struct State {
    pending_esc_pressed: bool,
    wheeler: Option<Wheeler>,
}

impl State {}

lazy_static::lazy_static! {
    static ref STATE: Arc<Mutex<RefCell<State>>> = Arc::new(Mutex::new(RefCell::new(State::default())));
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

        state.wheeler = Some(Wheeler::new(km.create_mouse_uinput("-wheel")));
    });

    config.on_event(|km, device, ev| {
        if ev.event_type != EventType::EV_KEY {
            return; // Ignore non-key events.
        }

        let lock = STATE.lock();
        let mut state = lock.borrow_mut();
        
        let is_thinkpad = device.name().starts_with("AT");
        let is_xkeys = device.name().starts_with("P. I.");

        let is_caps_pressed = km.is_capslock_pressed();

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

        let mut ev = ev.clone();

        if is_thinkpad && !is_caps_pressed {
            // Special for the thinkpad keyboard. Use INS/DEL as PAGEUP/DOWN, unless caps is pressed.
            match ev.code {
                ec::KEY_INSERT => {ev.code = ec::KEY_PAGEUP;},
                ec::KEY_DELETE => {ev.code = ec::KEY_PAGEDOWN;},
                _ => {},
            }
        }


        km.send_event(&ev);
    });

    keyremapper::start(config);

    return Ok(());
}
