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

#[derive(Debug, Default)]
struct State {
    wheel_uinput: Option<SyncedUinput>,
    vwheel_speed: i32,
    hwheel_speed: i32,
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

        state.wheel_uinput = Some(km.create_mouse_uinput("-wheel"));
    });

    config.on_event(|km, _device, ev| {
        if ev.event_type != EventType::EV_KEY {
            return; // Ignore non-key events.
        }
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        km.send_event(ev);
    });

    keyremapper::start(config);

    return Ok(());
}
