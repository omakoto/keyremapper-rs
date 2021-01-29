//! Reampper for the main keyboard
extern crate lazy_static;

use std::{
    cell::RefCell,
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::{value_t, Arg};
use keyremapper::{
    evdev::{self, ec},
    res::{get_gio_resource_as_file, EmbeddedIcon},
    KeyRemapper, KeyRemapperConfiguration,
};

use parking_lot::Mutex;

const NAME: &str = "Keyboard remapper";

// AT Translated Set 2 keyboard -> thinkpad internal keyboard
// Topre Corporation Realforce  -> Realforce
// P. I. Engineering XK-16 HID  -> An external 8-key keyboard
const DEVICE_RE: &str = r#"^(AT Translated Set 2 keyboard|Topre Corporation Realforce|P. I. Engineering XK-16 HID)"#;
const ID_RE: &str = "^";

// Simulated mouse wheel speed.
const NORMAL_SCROLL_INTERNAL: &str = "30";
const FAST_SCROLL_INTERVAL: &str = "5";
const FIRST_SCROLL_DELAY: &str = "100";

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

// printable character keys on the 101 keyboard.
static PRINTABLE_KEYS: &[i32] = &[
    ec::KEY_0,
    ec::KEY_1,
    ec::KEY_2,
    ec::KEY_3,
    ec::KEY_4,
    ec::KEY_5,
    ec::KEY_6,
    ec::KEY_7,
    ec::KEY_8,
    ec::KEY_9,
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
    ec::KEY_GRAVE,
    ec::KEY_MINUS,
    ec::KEY_EQUAL,
    ec::KEY_LEFTBRACE,
    ec::KEY_RIGHTBRACE,
    ec::KEY_BACKSLASH,
    ec::KEY_SEMICOLON,
    ec::KEY_APOSTROPHE,
    ec::KEY_COMMA,
    ec::KEY_DOT,
    ec::KEY_SLASH,
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

const ARG_NORMAL_SCROLL_INTERVAL: &str = "normal_scroll_internal";
const ARG_FAST_SCROLL_INTERVAL: &str = "fast_scroll_interval";
const ARG_FIRST_SCROLL_DELAY: &str = "first_scroll_delay";

/// Worker thread that sends mouse wheel events repeatedly.
mod wheeler;

/// `State` stores the internal state.
#[derive(Debug, Default)]
struct State {
    pending_esc_pressed: bool,
    wheeler: Option<wheeler::Wheeler>,

    normal_scroll_internal: Duration,
    fast_scroll_interval: Duration,
    first_scroll_delay: Duration,

    alt_mode: bool,
}

impl State {}

lazy_static::lazy_static! {
    static ref STATE: Arc<Mutex<RefCell<State>>> = Arc::new(Mutex::new(RefCell::new(State::default())));
}

// Returns true if the active window is Chrome.
fn is_chrome() -> bool {
    let active_window = match keyremapper::ui::WindowInfo::from_active_window() {
        Ok(res) => res,
        Err(e) => {
            log::warn!("Unable to get active window info: {}", e);
            return false;
        }
    };
    log::debug!("Active window={:?}", active_window);
    return active_window.class_group_name == "Google-chrome";
}

/// Entry point.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // // Prepare the icon.
    // let icon = get_gio_resource_as_file(NAME, "/keyremapper/resources/keyboard.png", &|| {
    //     let data = glib::Bytes::from(include_bytes!("icons.bin"));
    //     return gio::Resource::from_data(&data).unwrap();
    // });

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(EmbeddedIcon::from_bytes(
            NAME,
            "/keyremapper/resources/keyboard.png",
            include_bytes!("icons.bin"),
        ))
        .set_id_regex(ID_RE)
        .set_use_non_keyboard(true)
        .set_grab(true)
        .set_write_to_uinput(true);

    // Set up arguments.
    config.on_init_args(|app| {
        return app
            .arg(
                Arg::with_name(ARG_NORMAL_SCROLL_INTERVAL)
                    .long("normal-scroll-interval-ms")
                    .value_name("MILLIS")
                    .default_value(NORMAL_SCROLL_INTERNAL)
                    .help(r#"Simulated mouse wheel event interval for scrolling"#)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(ARG_FAST_SCROLL_INTERVAL)
                    .long("fast-scroll-interval-ms")
                    .value_name("MILLIS")
                    .default_value(FAST_SCROLL_INTERVAL)
                    .help(r#"Simulated mouse wheel event interval for fast scrolling"#)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(ARG_FIRST_SCROLL_DELAY)
                    .long("fast-scroll-delay-ms")
                    .value_name("MILLIS")
                    .default_value(FIRST_SCROLL_DELAY)
                    .help(r#"Delay before fast mouse wheel events kick in"#)
                    .takes_value(true),
            );
    });

    // Parse arguments.
    config.on_args_parsed(|matches| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        let get_arg = |arg: &str| -> Duration {
            let val = value_t!(matches.value_of(arg), u64).unwrap_or_else(|e| e.exit());
            return Duration::from_millis(val.max(1));
        };

        state.normal_scroll_internal = get_arg(ARG_NORMAL_SCROLL_INTERVAL);
        state.fast_scroll_interval = get_arg(ARG_FAST_SCROLL_INTERVAL);
        state.first_scroll_delay = get_arg(ARG_FIRST_SCROLL_DELAY);
    });

    config.on_start(|km| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        let wheeler = wheeler::Wheeler::new(
            km.create_mouse_uinput("-wheel"),
            state.normal_scroll_internal,
            state.fast_scroll_interval,
            state.first_scroll_delay,
        );
        wheeler.start();
        state.wheeler = Some(wheeler);

        log::debug!("{:#?}", state);
    });

    config.on_devices_lost(|_km| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        state.wheeler.as_mut().unwrap().reset();
    });

    config.on_event(|km, device, ev| {
        if !ev.is_key_event() {
            return; // Ignore non-key events.
        }

        let lock = STATE.lock();
        let mut state = lock.borrow_mut();

        let is_thinkpad = device.name().starts_with("AT");
        let is_xkeys = device.name().starts_with("P. I.");

        // For x-keys. Convert to Shift+Ctrl+[number]
        if is_xkeys {
            match 0 {
                // Special casing the first two keys -- convert to ALT+left / right (i.e. back / forward)
                _ if ev.is_key_on(ec::KEY_1, "") => km.press_key(ec::KEY_LEFT, "a"),
                _ if ev.is_key_on(ec::KEY_2, "") => km.press_key(ec::KEY_RIGHT, "a"),

                // Send the other keys with alt+ctrl+shift+meta.
                _ if ev.is_key_down_event() => km.press_key(ev.code, "sacw"),
                _ => log::warn!("Unexpected event {}", ev),
            }
            return;
        }

        let mut ev = &mut ev.clone();

        // Special for the thinkpad keyboard. Use INS/DEL as PAGEUP/DOWN, unless caps is pressed.
        if is_thinkpad && !km.is_key_on(ec::KEY_CAPSLOCK) {
            match ev.code {
                ec::KEY_INSERT => ev.code = ec::KEY_PAGEUP,
                ec::KEY_DELETE => ev.code = ec::KEY_PAGEDOWN,
                _ => {}
            }
        }

        // Special handling for ESC: Don't send "ESC-press" on key-down, but instead send it on key-*up*, unless
        // any keys are pressed between the down and up.
        // This allows to make "ESC + BACKSPACE" act as a DEL press without sending ESC.
        if !state.alt_mode && ev.code == ec::KEY_ESC {
            // Ctrl + ESC -> Enter ALT mode
            if ev.is_key_down(ec::KEY_ESC, "ce") {
                state.alt_mode = true;
                km.show_notiication_with_timeout("ALT mode", Duration::from_secs(60 * 60 * 24));
                return;
            }

            if ev.is_key_down_event() {
                state.pending_esc_pressed = true;
            }
            if ev.is_key_up_event() && state.pending_esc_pressed {
                state.pending_esc_pressed = false;
                km.press_key(ec::KEY_ESC, "*");
            }
            return;
        }

        // If other keys (than ESC) are pressed, clear pending_esc_pressed, but don't do so on modifier key presses, in order to
        // allow combos like "ESC+ctrl+Backspace".
        if state.pending_esc_pressed && !MODIFIER_KEYS.contains(&ev.code) {
            state.pending_esc_pressed = false;
        }

        // ESC or ENTER will finish the ALT mode.
        if state.alt_mode && ev.is_any_key_down(&[ec::KEY_ENTER, ec::KEY_ESC], "*") {
            state.alt_mode = false;
            km.show_notiication_with_timeout("Left ALT mode", Duration::from_millis(100));
            return;
        }

        match 0 {
            // ESC + H / J / K / L -> emulate wheel. Also support ESC+SPACE / C for left-hand-only scrolling.
            _ if ev.is_any_key(&[ec::KEY_J, ec::KEY_K, ec::KEY_SPACE, ec::KEY_C], "*") && (state.alt_mode || km.is_esc_on()) => {
                let speed = match 0 {
                    _ if ev.is_key_up_event() => 0,
                    _ if ev.is_any_key_down(&[ec::KEY_K, ec::KEY_C], "*") => 1,
                    _ if ev.is_any_key_down(&[ec::KEY_J, ec::KEY_SPACE], "*") => -1,
                    _ => return,
                };
                state.wheeler.as_mut().unwrap().set_vwheel(speed);
            }
            _ if ev.is_any_key(&[ec::KEY_L, ec::KEY_H], "*") && (state.alt_mode || km.is_esc_on()) => {
                let speed = match 0 {
                    _ if ev.is_key_up_event() => 0,
                    _ if ev.is_any_key_down(&[ec::KEY_L], "*") => 1,
                    _ if ev.is_any_key_down(&[ec::KEY_H], "*") => -1,
                    _ => return,
                };
                state.wheeler.as_mut().unwrap().set_hwheel(speed);
            }

            // Chrome only -- F5 / F6 as BACK / FORWARD
            _ if ev.is_key_on(ec::KEY_F5, "") && is_chrome() => km.press_key(ec::KEY_BACK, ""),
            _ if ev.is_key_on(ec::KEY_F6, "") && is_chrome() => km.press_key(ec::KEY_FORWARD, ""),

            // ESC or shift + backspace -> delete
            _ if ev.is_key_on(ec::KEY_BACKSPACE, "e") => km.press_key(ec::KEY_DELETE, ""),
            _ if ev.is_key_on(ec::KEY_BACKSPACE, "s") => km.press_key(ec::KEY_DELETE, ""),

            // See VERSATILE_KEYS.
            _ if ev.is_any_key_on(VERSATILE_KEYS, "e") => km.press_key(ev.code, "acsw"),

            // ESC + home/end -> ATL+Left/Right (back / forward)
            _ if ev.is_key_on(ec::KEY_HOME, "e") => km.press_key(ec::KEY_LEFT, "a"),
            _ if ev.is_key_on(ec::KEY_END, "e") => km.press_key(ec::KEY_RIGHT, "a"),

            // ESC + Pageup -> ctrl + pageup (prev tab)
            // ESC + Pagedown -> ctrl + pagedown (next tab)
            // (meaning ESC + ins/del act as them too on thinkpad.)
            _ if ev.is_any_key_on(&[ec::KEY_PAGEUP, ec::KEY_PAGEDOWN], "e") => km.press_key(ev.code, "c"),

            // ESC + caps lock -> caps lock, in case I ever need it.
            _ if ev.is_key_on(ec::KEY_CAPSLOCK, "e*") => km.press_key(ec::KEY_CAPSLOCK, "c"),

            // ESC + other alphabet -> ctrl + shift + the key.
            _ if ev.is_any_key_on(PRINTABLE_KEYS, "e") => km.press_key(ev.code, "cs"),

            // Don't use capslock alone.
            _ if ev.code == ec::KEY_CAPSLOCK => {}

            // In alt-mode, don't use printable char keys as-is.
            _ if state.alt_mode && ev.is_any_key_on(PRINTABLE_KEYS, "") => {}

            // Default: Just send the original key event.
            _ => km.send_event(&ev),
        };
    });

    keyremapper::start(config);

    return Ok(());
}
