use std::{
    error::Error,
    io::{stdin, stdout, Write},
    path::Path,
};

use keyremapper::evdev::{self, EvdevDevice};

fn pick_device() -> Result<EvdevDevice, Box<dyn Error>> {
    let devices = evdev::list_devices()?;

    for device in &devices {
        println!("{} {}", device.path(), device.name())
    }
    loop {
        // Show the devices

        print!("Device number? ");
        stdout().flush().unwrap();
        let mut number_as_str = String::new();
        stdin().read_line(&mut number_as_str)?;
        let number: usize = number_as_str.trim().parse()?;

        let device_name = format!("/dev/input/event{}", number);
        let path = &Path::new(&device_name);
        return Ok(EvdevDevice::with_path(&path)?);
    }
}

fn main() {
    let device = pick_device().unwrap();

    log::info!(
        "Device selected: {} {} ({:?})",
        device.path(),
        device.name(),
        device
    );

    loop {
        let events = device.next_events().unwrap();
        for ev in &events {
            println!("{}", &ev)
        }
    }
}
