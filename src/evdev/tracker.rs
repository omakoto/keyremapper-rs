use std::{collections::HashMap, error::Error, sync::Arc};

use parking_lot::RwLock;

use super::{ec, InputEvent};

#[derive(Debug, Clone)]
struct InputEventTrackerInner {
    key_states: HashMap<i32, i32>,
    syn_report_pending: bool,
}

#[derive(Debug, Clone)]
pub struct InputEventTracker {
    inner: Arc<RwLock<InputEventTrackerInner>>,
}

impl InputEventTracker {
    pub fn new() -> InputEventTracker {
        return InputEventTracker {
            inner: Arc::new(RwLock::new(InputEventTrackerInner {
                key_states: HashMap::new(),
                syn_report_pending: false,
            })),
        };
    }

    pub fn key_state(&self, code: i32) -> i32 {
        let ks = self.inner.read();
        return *ks.key_states.get(&code).unwrap_or(&0);
    }

    pub fn should_send(&self, ev: &InputEvent) -> bool {
        let inner = self.inner.read();
        return InputEventTracker::should_send_no_lock(&inner, ev);
    }

    fn should_send_no_lock(inner: &InputEventTrackerInner, ev: &InputEvent) -> bool {
        if ev.is_syn_report() && !inner.syn_report_pending {
            return false;
        }
        if ev.event_type == ec::EventType::EV_KEY {
            let current = *inner.key_states.get(&ev.code).unwrap_or(&0);
            if ev.value == 0 {
                if current == 0 {
                    return false; // Don't send if not pressed.
                }
            } else if ev.value == 1 {
                if current != 0 {
                    return false; // Don't send if already pressed.
                }
            } else if ev.value == 2 {
                if current == 0 {
                    return false; // Don't send if not pressed.
                }
            } else {
                panic!("Invalid value for EV_KEY: {}", ev.value);
            }
        }
        return true;
    }

    pub fn on_event_sent(&self, ev: &InputEvent) {
        let mut inner = self.inner.write();
        if ev.event_type == ec::EventType::EV_KEY && InputEventTracker::should_send_no_lock(&inner, ev) {
            inner.key_states.insert(ev.code, ev.value);
        }
        inner.syn_report_pending = !ev.is_syn_report();
    }

    pub fn reset(&self) -> Vec<InputEvent> {
        return self.reset_with_callback(|_| Ok(())).unwrap();
    }

    pub fn reset_with_callback<F>(&self, mut callback: F) -> Result<Vec<InputEvent>, Box<dyn Error>>
    where
        F: FnMut(&InputEvent) -> Result<(), Box<dyn Error>>,
    {
        let mut inner = self.inner.write();

        let mut reset_events = vec![];

        use itertools::Itertools;
        for (code, value) in inner.key_states.iter().sorted() {
            if *value > 0 {
                let ev = InputEvent::new(ec::EventType::EV_KEY, *code, 0);
                reset_events.push(ev);
                reset_events.push(InputEvent::new_syn_report());
                callback(&ev)?;
            }
        }
        inner.key_states.clear();
        inner.syn_report_pending = false;
        return Ok(reset_events);
    }
}

#[test]
fn test_input_event_tracker() {
    let et = InputEventTracker::new();

    assert_eq!(0, et.key_state(0));
    assert_eq!(0, et.key_state(1));

    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0)));
    assert_eq!(false, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1)));
    assert_eq!(false, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1)));

    et.on_event_sent(&InputEvent::new(EventType::EV_KEY, 1, 1));
    et.on_event_sent(&InputEvent::new_syn_report());

    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0)));
    assert_eq!(false, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1)));

    {
        let et2 = et.clone();

        et2.on_event_sent(&InputEvent::new(EventType::EV_KEY, 1, 2));
        et2.on_event_sent(&InputEvent::new_syn_report());
    }

    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_SYN, 0, 0)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 0)));
    assert_eq!(false, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 1)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 1, 2)));
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_KEY, 2, 1)));

    et.on_event_sent(&InputEvent::new(EventType::EV_KEY, 3, 1));
    et.on_event_sent(&InputEvent::new_syn_report());

    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_REL, 5, 1)));
    et.on_event_sent(&InputEvent::new(EventType::EV_REL, 5, 1));
    et.on_event_sent(&InputEvent::new_syn_report());
    assert_eq!(true, et.should_send(&InputEvent::new(EventType::EV_REL, 5, 1)));

    {
        let reset_events = et.reset();

        let expected: Vec<InputEvent> = vec![
            InputEvent::new(EventType::EV_KEY, 1, 0),
            InputEvent::new_syn_report(),
            InputEvent::new(EventType::EV_KEY, 3, 0),
            InputEvent::new_syn_report(),
        ];
        assert_eq!(expected, reset_events);
    }

    {
        let reset_events = et.reset();

        let expected: Vec<InputEvent> = vec![];
        assert_eq!(expected, reset_events);
    }
}
