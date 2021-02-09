use std::{
    cell::RefCell,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use keyremapper::evdev::{ec, uinput::SyncedUinput, InputEvent};
use parking_lot::{Condvar, Mutex};

#[derive(Debug)]
struct Inner {
    vwheel_speed: i32,
    hwheel_speed: i32,
}

#[derive(Debug, Clone)]
pub struct Wheeler {
    inner: Arc<Mutex<RefCell<Inner>>>,
    uinput: Arc<SyncedUinput>,
    cond: Arc<Condvar>,

    normal_scroll_internal: Duration,
    fast_scroll_interval: Duration,
    first_scroll_delay: Duration,
}

impl Wheeler {
    pub fn new(uinput: SyncedUinput, normal_scroll_internal: Duration, fast_scroll_interval: Duration, first_scroll_delay: Duration) -> Wheeler {
        let inner = Inner {
            vwheel_speed: 0,
            hwheel_speed: 0,
        };

        return Wheeler {
            inner: Arc::new(Mutex::new(RefCell::new(inner))),
            uinput: Arc::new(uinput),
            cond: Arc::new(Condvar::new()),
            normal_scroll_internal,
            fast_scroll_interval,
            first_scroll_delay,
        };
    }

    fn with_lock<F>(&self, callback: F)
    where
        F: Fn(&mut Inner),
    {
        let inner = self.inner.lock();
        callback(&mut inner.borrow_mut());
        self.cond.notify_one();
    }

    pub fn reset(&self) {
        self.with_lock(|inner| {
            // log::debug!("Wheel reset");
            inner.vwheel_speed = 0;
            inner.hwheel_speed = 0;
        });
    }

    pub fn set_vwheel(&mut self, value: i32) {
        self.with_lock(|inner| {
            // log::debug!("Wheel v -> {}", 0);
            inner.vwheel_speed = value;
        });
    }

    pub fn set_hwheel(&mut self, value: i32) {
        self.with_lock(|inner| {
            // log::debug!("Wheel h -> {}", value);
            inner.hwheel_speed = value;
        });
    }

    /// Start the worker thread.
    pub fn start(&self) {
        let clone = self.clone();

        thread::Builder::new()
            .name(format!("{}-wheeler", super::NAME))
            .spawn(move || {
                clone.thread_main();
            })
            .expect("Unable to wheeler thread");
    }

    /// Entry point of the thread.
    fn thread_main(&self) {
        log::info!("Wheeler thread started...");

        let zero_instant: Instant = Instant::now();

        let mut last_started: Instant = zero_instant;
        loop {
            let mut v = 0;
            let mut h = 0;
            loop {
                let mut inner = self.inner.lock();
                {
                    let i = inner.borrow();
                    v = i.vwheel_speed;
                    h = i.hwheel_speed;
                }
                if v == 0 && h == 0 {
                    // log::debug!("Wheel stop");

                    // No wheel event. Wait until the next one...
                    last_started = zero_instant;
                    self.cond.wait(&mut inner);
                } else {
                    // Remeber the scholl start time.
                    if last_started == zero_instant {
                        last_started = Instant::now();
                    }
                    break;
                }
            }
            if v != 0 || h != 0 {
                // Make sure all modifiers are released when sending a wheel event.
                self.uinput.reset().unwrap();
            }
            if v != 0 {
                self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_WHEEL, v)).unwrap();
                self.uinput
                    .send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_WHEEL_HI_RES, v * 120))
                    .unwrap();
            }
            if h != 0 {
                self.uinput.send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_HWHEEL, h)).unwrap();
                self.uinput
                    .send_event(&InputEvent::new(ec::EventType::EV_REL, ec::REL_HWHEEL_HI_RES, h * 120))
                    .unwrap();
            }

            // Insert a delay.
            let wait = if Instant::now().duration_since(last_started) >= self.first_scroll_delay {
                self.fast_scroll_interval
            } else {
                self.normal_scroll_internal
            };
            thread::sleep(wait);
        }
    }
}
