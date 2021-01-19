use core::fmt::Debug;
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use std::{path::PathBuf, sync::Arc};

use crate::{
    evdev::{self, EventsDescriptor},
    KeyRemapper, UINPUT_DEVICE_NAME_PREFIX,
};

/// Stores callbacks for `KeyRemapperCallbacks`.
/// It's extracted o implement the `Debug` trait, which `Fn` doesn't have.
/// It uses `Arc` to make it `Clone`.
#[derive(Clone)]
pub(crate) struct KeyRemapperCallbacks<TState> where TState: Clone + Send + Sync {
    pub(crate) on_init_args: Arc<dyn for<'a, 'b> Fn(&mut TState, clap::App<'a, 'b>) -> clap::App<'a, 'b> + Send + Sync + 'static>,
    pub(crate) on_args_parsed: Arc<dyn Fn(&mut TState, &clap::ArgMatches) + Send + Sync + 'static>,

    pub(crate) on_start: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>,

    pub(crate) on_devices_detected: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>, &[evdev::EvdevDevice]) + Send + Sync + 'static>,
    pub(crate) on_devices_not_found: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>,
    pub(crate) on_devices_lost: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>,

    pub(crate) on_stop: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>,

    pub(crate) on_events_batch: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>, &evdev::EvdevDevice, &[evdev::InputEvent]) + Send + Sync + 'static>,
    pub(crate) on_event: Arc<dyn Fn(&mut TState, &KeyRemapper<TState>, &evdev::EvdevDevice, &evdev::InputEvent) + Send + Sync + 'static>,
}

impl<TState> Debug for KeyRemapperCallbacks<TState> where TState: Clone + Send + Sync {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "KeyRemapperCallbacks{{...}}")
    }
}

impl<TState> KeyRemapperCallbacks<TState> where TState: Clone + Send + Sync {
    fn new() -> KeyRemapperCallbacks<TState> {
        KeyRemapperCallbacks {
            on_init_args: Arc::new(|_, app| app),
            on_args_parsed: Arc::new(|_, _| {}),
            on_start: Arc::new(|_, _| {}),
            on_devices_detected: Arc::new(|_, _, _| {}),
            on_devices_not_found: Arc::new(|_, _| {}),
            on_devices_lost: Arc::new(|_, _| {}),
            on_stop: Arc::new(|_, _| {}),
            on_events_batch: Arc::new(|_, _, _, _| {}),
            on_event: Arc::new(|_, _, _, _| {}),
        }
    }
}

/// All the configurations passed from the client app.
#[derive(Debug, Clone)]
pub struct KeyRemapperConfiguration<TState> where TState: Send + Sync {
    pub(crate) name: String,
    pub(crate) icon: Option<PathBuf>,

    pub(crate) device_name_regex: String,
    pub(crate) id_regex: String,

    pub(crate) device_name_regex_re: Option<Regex>,
    pub(crate) id_regex_re: Option<Regex>,

    pub(crate) use_system_tray: bool,

    pub(crate) use_non_keyboard: bool,
    pub(crate) grab_devices: bool,
    pub(crate) write_to_uinput: bool,
    pub(crate) uinput_events: EventsDescriptor,
    pub(crate) global_lock_name: String,
    pub(crate) uinput_device_name_suffix: String,

    pub(crate) uinput_devices_prefix: String,

    pub(crate) callbacks: Arc<RwLock<KeyRemapperCallbacks<TState>>>,
    pub(crate) state: Arc<Mutex<TState>>,
}

impl<TState> KeyRemapperConfiguration<TState>  where TState: Clone + Send + Sync {
    pub fn new(name: &str, device_name_regex: &str, state: TState) -> KeyRemapperConfiguration<TState> {
        KeyRemapperConfiguration {
            name: name.to_string(),
            icon: None,
            device_name_regex: device_name_regex.to_string(),
            id_regex: "".to_string(),
            use_system_tray: true,
            use_non_keyboard: true,
            grab_devices: true,
            write_to_uinput: true,
            uinput_events: EventsDescriptor::default(),
            global_lock_name: String::new(),
            uinput_device_name_suffix: String::new(),
            uinput_devices_prefix: String::new(),
            callbacks: Arc::new(RwLock::new(KeyRemapperCallbacks::new())),
            device_name_regex_re: None,
            id_regex_re: None,
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn set_icon<T>(&mut self, path: T) -> &mut KeyRemapperConfiguration<TState>
    where
        T: Into<PathBuf>,
    {
        self.icon = Some(path.into());
        self
    }

    pub fn set_device_name_regex(&mut self, value: &str) -> &mut KeyRemapperConfiguration<TState> {
        self.device_name_regex = value.to_string();
        self
    }

    pub fn set_id_regex(&mut self, value: &str) -> &mut KeyRemapperConfiguration<TState> {
        self.id_regex = value.to_string();
        self
    }

    pub fn set_use_system_tray(&mut self, value: bool) -> &mut KeyRemapperConfiguration<TState> {
        self.use_system_tray = value;
        self
    }

    pub fn set_grab(&mut self, value: bool) -> &mut KeyRemapperConfiguration<TState> {
        self.grab_devices = value;
        self
    }

    pub fn set_write_to_uinput(&mut self, value: bool) -> &mut KeyRemapperConfiguration<TState> {
        self.write_to_uinput = value;
        self
    }

    pub fn set_uinput_events(&mut self, events: EventsDescriptor) -> &mut KeyRemapperConfiguration<TState> {
        self.uinput_events = events;
        self
    }

    pub fn set_use_non_keyboard(&mut self, value: bool) -> &mut KeyRemapperConfiguration<TState> {
        self.use_non_keyboard = value;
        self
    }

    pub fn on_init_args<F>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState>
    where
        F: for<'a, 'b> Fn(&mut TState, clap::App<'a, 'b>) -> clap::App<'a, 'b> + Send + Sync + 'static,
    {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_init_args = Arc::new(callback);
        }
        self
    }

    pub fn on_args_parsed<F: Fn(&mut TState, &clap::ArgMatches) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_args_parsed = Arc::new(callback);
        }
        self
    }

    pub fn on_start<F: Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_start = Arc::new(callback);
        }
        self
    }

    pub fn on_devices_detected<F: Fn(&mut TState, &KeyRemapper<TState>, &[evdev::EvdevDevice]) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_devices_detected = Arc::new(callback);
        }
        self
    }

    pub fn on_devices_not_found<F: Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_devices_not_found = Arc::new(callback);
        }
        self
    }

    pub fn on_devices_lost<F: Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_devices_lost = Arc::new(callback);
        }
        self
    }

    pub fn on_stop<F: Fn(&mut TState, &KeyRemapper<TState>) + Send + Sync + 'static>(&mut self, callback: F) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_stop = Arc::new(callback);
        }
        self
    }

    pub fn on_events_batch<F: Fn(&mut TState, &KeyRemapper<TState>, &evdev::EvdevDevice, &[evdev::InputEvent]) + Send + Sync + 'static>(
        &mut self,
        callback: F,
    ) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_events_batch = Arc::new(callback);
        }
        self
    }

    pub fn on_event<F: Fn(&mut TState, &KeyRemapper<TState>, &evdev::EvdevDevice, &evdev::InputEvent) + Send + Sync + 'static>(
        &mut self,
        callback: F,
    ) -> &mut KeyRemapperConfiguration<TState> {
        {
            let mut callbacks = self.callbacks.write();
            callbacks.on_event = Arc::new(callback);
        }
        self
    }

    pub(crate) fn set_defaults(&mut self) -> &mut KeyRemapperConfiguration<TState> {
        let name_cleansed = Regex::new(r#"\s+"#).unwrap().replace(&self.name, "_").to_string();
        if self.global_lock_name.is_empty() {
            self.global_lock_name = name_cleansed.clone();
        }
        if self.uinput_device_name_suffix.is_empty() {
            self.uinput_device_name_suffix = {
                let mut suffix = "-".to_string();
                suffix.push_str(&name_cleansed);
                suffix
            }
        }
        self.uinput_devices_prefix = {
            let mut name = UINPUT_DEVICE_NAME_PREFIX.to_string();
            name.push_str(&self.uinput_device_name_suffix);
            name
        };

        if self.uinput_events.is_empty() {
            self.uinput_events = EventsDescriptor::with_all_key_events();
        }
        self
    }

    pub(crate) fn update_regexes(&mut self) {
        self.device_name_regex_re = Some(Regex::new(&self.device_name_regex).expect("Invalid regex detected"));
        self.id_regex_re = Some(Regex::new(&self.id_regex).expect("Invalid regex detected"));
    }

    pub(crate) fn callbacks_cloned(&self) -> KeyRemapperCallbacks<TState> {
        let callbacks = self.callbacks.read();
        callbacks.clone()
    }
}
