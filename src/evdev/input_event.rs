use std::fmt::{self, Display};

use crate::native;

use super::ec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEvent {
    pub time_sec: i64,
    pub time_usec: i64,
    pub event_type: ec::EventType,
    pub code: i32,
    pub value: i32,
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match ec::get_type_name(self.event_type as i32) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown type {}]", self.event_type as i32),
        };
        let code_str = match ec::get_code_name(self.event_type as i32, self.code) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown code {}]", self.code),
        };
        return write!(
            f,
            "{{InputEvent: time={}.{:06} type={} code={} value={}}}",
            self.time_sec, self.time_usec, type_str, code_str, self.value
        );
    }
}

impl InputEvent {
    pub fn with_timestamp(time_sec: i64, time_usec: i64, event_type: ec::EventType, code: i32, value: i32) -> InputEvent {
        return InputEvent {
            time_sec,
            time_usec,
            event_type,
            code,
            value,
        };
    }

    pub fn new(event_type: ec::EventType, code: i32, value: i32) -> InputEvent {
        return InputEvent {
            time_sec: 0,
            time_usec: 0,
            event_type,
            code,
            value,
        };
    }

    /// Return a new SYN_REPORT event.
    pub fn new_syn_report() -> InputEvent {
        return InputEvent::new(ec::EventType::EV_SYN, ec::SYN_REPORT, 0);
    }

    pub fn new_key_event(code: i32, value: i32) -> InputEvent {
        return InputEvent::new(ec::EventType::EV_KEY, code, value);
    }

    pub(crate) fn from_native_input_event(ie: &native::input_event) -> InputEvent {
        return InputEvent {
            time_sec: ie.time.tv_sec,
            time_usec: ie.time.tv_usec,
            event_type: ec::EventType::from_i32(ie.type_ as i32),
            code: ie.code as i32,
            value: ie.value as i32,
        };
    }

    /// Return true if it's a SYN_REPORT event.
    pub fn is_syn_report(&self) -> bool {
        return self.event_type == ec::EventType::EV_SYN && self.code == ec::SYN_REPORT && self.value == 0;
    }
}

#[test]
fn test_input_event_format_ev_key() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=KEY_A value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, ec::KEY_A, 1))
    )
}

#[test]
fn test_input_event_format_ev_time() {
    assert_eq!(
        "{InputEvent: time=1.000002 type=EV_KEY code=KEY_A value=1}",
        format!(
            "{}",
            InputEvent {
                time_sec: 1,
                time_usec: 2,
                event_type: ec::EventType::EV_KEY,
                code: ec::KEY_A,
                value: 1,
            }
        )
    )
}

#[test]
fn test_input_event_format_ev_key_btn() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=BTN_1 value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, ec::BTN_1, 1))
    )
}

#[test]
fn test_input_event_format_ev_key_unknown() {
    assert_eq!(
        "{InputEvent: time=0.000000 type=EV_KEY code=[Unknown code 999999] value=1}",
        format!("{}", InputEvent::new(ec::EventType::EV_KEY, 999999, 1))
    )
}

// TODO Add more tests
