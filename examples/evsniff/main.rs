//! Reampper for the main keyboard
extern crate lazy_static;

use std::error::Error;

use keyremapper::KeyRemapperConfiguration;

const NAME: &str = "EvSniff";

/// Entry point.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, "");
    config
        .set_use_non_keyboard(true) // Read from all devices.
        .set_grab(false) // This allows to read from othre keyremapper uinput devices.
        .set_write_to_uinput(false); // No need to create a uinput device.

    config.on_devices_detected(|_km, devices| {
        println!("Device(s) detected:");
        for device in devices {
            println!("  {:20} {}: {}", device.path(), device.id_str(), device.name());
        }
    });

    config.on_devices_not_found(|_km| {
        println!("No devices found.");
    });

    config.on_events_batch(|_km, device, events| {
        println!("From device [{}]: {} ({})", device.id_str(), device.name(), device.path());
        for ev in events {
            println!("  {}", ev);
        }
    });

    keyremapper::start(config);

    return Ok(());
}
