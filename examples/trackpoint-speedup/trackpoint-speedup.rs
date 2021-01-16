//! Speed up trackpoint.
extern crate lazy_static;

use std::{cell::RefCell, error::Error, sync::Arc};

use clap::{value_t, App, Arg};
use keyremapper::{
    evdev::{self, ec, EventsDescriptor, InputEvent},
    res::get_gio_resource_as_file,
    KeyRemapper, KeyRemapperConfiguration,
};
use parking_lot::Mutex;

const NAME: &str = "Trackpoint Spped-up";
const DEVICE_RE: &str = r#"^TPPS/2 Elan TrackPoint"#;
const ID_RE: &str = "^";

#[derive(Default, Debug, Copy, Clone)]
struct Settings {
    threshold: f64,
    add: f64,
    power: f64,
    scale: f64,
}

lazy_static::lazy_static! {
    static ref SETTINGS: Arc<Mutex<RefCell<Settings>>> = Arc::new(Mutex::new(RefCell::new(Settings::default())));
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Prepare the icon.
    let icon = get_gio_resource_as_file(NAME, "/keyremapper/resources/trackpoint.png", &|| {
        let data = glib::Bytes::from(include_bytes!("icons.bin"));
        return gio::Resource::from_data(&data).unwrap();
    });

    let mut supported_events = EventsDescriptor::default();
    supported_events.events.insert(
        ec::EventType::EV_KEY,
        vec![ec::BTN_LEFT, ec::BTN_RIGHT, ec::BTN_MIDDLE],
    );
    supported_events
        .events
        .insert(ec::EventType::EV_REL, vec![ec::REL_X, ec::REL_Y]);

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(icon)
        .set_id_regex(ID_RE)
        .set_grab(true)
        .set_use_non_keyboard(true)
        .set_write_to_uinput(true)
        .set_uinput_events(supported_events);

    config.on_init_args(|app| {
        let ret = app
            .arg(
                Arg::with_name("threshold")
                    .long("threshold")
                    .default_value(&"2")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("add")
                    .long("add")
                    .default_value(&"0")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("power")
                    .long("power")
                    .default_value(&"2.5")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("scale")
                    .long("scale")
                    .default_value(&"5")
                    .takes_value(true),
            );
        return ret;
    });
    config.on_args_parsed(|matches| {
        let locked_settings = SETTINGS.lock();
        let mut s = locked_settings.borrow_mut();
        s.threshold = value_t!(matches.value_of("threshold"), f64).unwrap_or_else(|e| e.exit());
        s.add = value_t!(matches.value_of("add"), f64).unwrap_or_else(|e| e.exit());
        s.power = value_t!(matches.value_of("power"), f64).unwrap_or_else(|e| e.exit());
        s.scale = value_t!(matches.value_of("scale"), f64).unwrap_or_else(|e| e.exit());

        log::debug!("{:#?}", s);
    });

    config.on_event(|km, _device, ev| {
        let locked_settings = SETTINGS.lock();
        let s = locked_settings.borrow();

        km.send_event(&speed_up(&s, ev));
    });

    keyremapper::start(config);

    return Ok(());
}

fn speed_up(s: &Settings, ev: &InputEvent) -> InputEvent {
    let mut new_ev = ev.clone();

    if ev.event_type == ec::EventType::EV_REL {
        let mut value = (ev.value as f64).abs() - s.threshold;
        if value < 1_f64 {
            value = ev.value as f64;
        } else {
            value = (value + s.add) / s.scale;
            value = ((1_f64 + value).powf(s.power) - 1_f64) * s.scale;
            value = value + s.threshold;

            if ev.value < 0 {
                value = -value;
            }
        }

        log::debug!("{}: {} -> {}", ev.code, ev.value, value);

        new_ev.value = value as i32
    }

    return new_ev;
}
