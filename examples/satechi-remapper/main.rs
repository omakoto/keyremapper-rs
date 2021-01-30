//! Remapper for https://www.amazon.com/gp/product/B00RM75NL0

use std::error::Error;

use ec::EventType;
use keyremapper::{
    evdev::{self, ec, EventsDescriptor, InputEvent},
    res::Resources,
    KeyRemapper, KeyRemapperConfiguration,
};

const NAME: &str = "Satechi Media Buttons remapper";
const DEVICE_RE: &str = r#"^Satechi Media Button Consumer Control"#;
const ID_RE: &str = "^";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, DEVICE_RE);
    config
        .set_icon(Resources::from_bytes(NAME, include_bytes!("icons.bin")).get_icon("/keyremapper/resources/circle.png"))
        .set_id_regex(ID_RE)
        .set_use_non_keyboard(true)
        .set_grab(true)
        .set_write_to_uinput(true);

    config.on_event(|km, _device, ev| {
        if ev.event_type != EventType::EV_KEY {
            return;
        }
        let mapped = match ev.code {
            ec::KEY_VOLUMEUP => ec::KEY_VOLUMEUP,
            ec::KEY_VOLUMEDOWN => ec::KEY_VOLUMEDOWN,
            ec::KEY_PLAYPAUSE => ec::KEY_SPACE,
            ec::KEY_PREVIOUSSONG => ec::KEY_LEFT,
            ec::KEY_NEXTSONG => ec::KEY_RIGHT,
            known => {
                log::warn!("Unknown input: code={}", known);
                return;
            }
        };
        km.press_key(mapped, "")
    });

    keyremapper::start(config);

    return Ok(());
}
