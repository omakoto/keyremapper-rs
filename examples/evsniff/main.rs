//! Evsniff is like evtest(1) but reads all the input devices.
use clap::{value_t, Arg};
use libc::{self, aio_write};
use std::{
    cell::RefCell,
    error::Error,
    iter::Copied,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use keyremapper::KeyRemapperConfiguration;
use parking_lot::Mutex;

const NAME: &str = "EvSniff";

#[derive(Debug, Clone, Copy, PartialEq)]
enum ColorMode {
    Always,
    Never,
    Auto,
}

impl Default for ColorMode {
    fn default() -> Self {
        return ColorMode::Auto;
    }
}

impl FromStr for ColorMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ColorMode::Always),
            "never" => Ok(ColorMode::Never),
            "auto" => Ok(ColorMode::Auto),
            _ => Err("no match"),
        }
    }
}

impl ColorMode {
    pub fn resolve(&self) -> ColorMode {
        return match self {
            ColorMode::Auto => {
                let isatty = unsafe { libc::isatty(1 as libc::c_int) };
                // log::debug!("isatty={}", isatty);
                if isatty == 1 {
                    ColorMode::Always
                } else {
                    ColorMode::Never
                }
            }
            other => other.clone(),
        };
    }

    pub fn to_name(&self) -> &'static str {
        return match self {
            ColorMode::Always => "always",
            ColorMode::Never => "never",
            ColorMode::Auto => "auto",
        };
    }

    pub fn device_line(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[36m",
            _ => "",
        };
    }

    pub fn device_id(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[93m",
            _ => "",
        };
    }

    pub fn device_name(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[92m",
            _ => "",
        };
    }

    pub fn syn_report(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[90m",
            _ => "",
        };
    }

    pub fn key_event(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[96;1m",
            _ => "",
        };
    }

    pub fn rel_event(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[93;1m",
            _ => "",
        };
    }

    pub fn abs_event(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[95;1m",
            _ => "",
        };
    }

    pub fn other_event(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[36m",
            _ => "",
        };
    }

    pub fn reset(&self) -> &'static str {
        return match self {
            ColorMode::Always => "\x1b[0m",
            _ => "",
        };
    }
}

#[derive(Debug)]
struct State {
    color_mode: ColorMode,
    last_was_syn_report: bool,
    last_event_time: Option<Instant>,
}

impl Default for State {
    fn default() -> Self {
        return State {
            color_mode: ColorMode::Auto,
            last_was_syn_report: false,
            last_event_time: None,
        };
    }
}

lazy_static::lazy_static! {
    static ref STATE: Arc<Mutex<RefCell<State>>> = Arc::new(Mutex::new(RefCell::new(State::default())));
}

/// Entry point.
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Set up the config.
    let mut config = KeyRemapperConfiguration::new(NAME, "");
    config
        .set_use_non_keyboard(true) // Read from all devices.
        .set_grab(false) // This allows to read from othre keyremapper uinput devices.
        .set_write_to_uinput(false) // No need to create a uinput device.
        .set_use_system_tray(false);

    config.on_init_args(|app| {
        return app.arg(
            Arg::with_name("color_mode")
                .long("colors")
                .default_value(ColorMode::Auto.to_name())
                .help(r#"Use colored output"#)
                .possible_values(&[ColorMode::Always.to_name(), ColorMode::Never.to_name(), ColorMode::Auto.to_name()])
                .takes_value(true),
        );
    });

    // Parse arguments.
    config.on_args_parsed(|m| {
        let options = STATE.lock();
        let color_mode_str = m.value_of("color_mode").unwrap();
        let c: ColorMode = ColorMode::from_str(color_mode_str).unwrap();
        options.borrow_mut().color_mode = c.resolve();
    });

    config.on_devices_detected(|_km, devices| {
        println!("Device(s) detected:");
        for device in devices {
            println!("  {:20} {}: {}", device.path(), device.id_str(), device.name());
        }
    });

    config.on_devices_not_found(|_km| {
        println!("No devices found.");
    });

    config.on_event(|_km, device, ev| {
        let lock = STATE.lock();
        let mut state = lock.borrow_mut();
        let c: ColorMode = state.color_mode;

        let show_device = match state.last_event_time {
            None => true,
            Some(time) => state.last_was_syn_report && Instant::now().duration_since(time) > Duration::from_millis(200),
        };
        if show_device {
            println!(
                "{}# From device [{}{}{}]: {}{}{} ({}){}",
                c.device_line(),
                c.device_id(),
                device.id_str(),
                c.device_line(),
                c.device_name(),
                device.name(),
                c.device_line(),
                device.path(),
                c.reset()
            );
        }

        let line_color = match 0 {
            _ if ev.is_syn_report() => c.syn_report(),
            _ if ev.is_key_event() => c.key_event(),
            _ if ev.is_rel_event() => c.rel_event(),
            _ if ev.is_abs_event() => c.abs_event(),
            _ => c.other_event(),
        };
        println!("{}{}{}", line_color, ev, c.reset());

        state.last_event_time = Some(Instant::now());
        state.last_was_syn_report = ev.is_syn_report();
    });

    keyremapper::start(config);

    return Ok(());
}
