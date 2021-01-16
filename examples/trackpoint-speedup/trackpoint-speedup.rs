//! Speed up trackpoint.
extern crate lazy_static;

use std::{cell::RefCell, error::Error, sync::Arc};

use clap::{App, Arg, value_t};
use keyremapper::{
    evdev::{self, ec},
    res::get_gio_resource_as_file,
    KeyRemapper, KeyRemapperConfiguration,
};
use parking_lot::Mutex;

const NAME: &str = "Trackpoint Spped-up";
const DEVICE_RE: &str = r#"^TPPS/2 Elan TrackPoint"#;
const ID_RE: &str = "^";

#[derive(Default)]
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

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(icon)
        .set_id_regex(ID_RE)
        .set_grab(true)
        .set_use_non_keyboard(true)
        .set_write_to_uinput(true);


    config.on_init_args(|app| {
        let ret = app.arg(
            Arg::with_name("threshold")
                .long("threshold")
                .default_value(&"2")
                .takes_value(true),
		).arg(
            Arg::with_name("add")
                .long("add")
                .default_value(&"0")
                .takes_value(true),
		).arg(
            Arg::with_name("power")
                .long("power")
                .default_value(&"2.5")
                .takes_value(true),
		).arg(
            Arg::with_name("scale")
                .long("scale")
                .default_value(&"5")
                .takes_value(true),
		);
		return ret;
    });
    config.on_args_parsed(|matches| {
        let locked_settings = SETTINGS.lock();
        let s = locked_settings.borrow_mut();
        s.threshold = value_t!(matches.value_of("threshold"), f64).unwrap_or_else(|e| e.exit());
        s.add = value_t!(matches.value_of("add"), f64).unwrap_or_else(|e| e.exit());
        s.power = value_t!(matches.value_of("power"), f64).unwrap_or_else(|e| e.exit());
        s.scale = value_t!(matches.value_of("scale"), f64).unwrap_or_else(|e| e.exit());
    });

    config.on_event(|km, device, event| {
    });

    keyremapper::start(config);

    return Ok(());
}
