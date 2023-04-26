use std::fmt::{self, Display};

use crate::{native, validate_modifiers};

use super::ec;

pub enum KeyEventType {
    Pressed,
    Released,
    Down,
}

impl KeyEventType {
    #[inline]
    pub fn match_event(&self, ev: &InputEvent) -> bool {
        return match self {
            KeyEventType::Pressed => ev.is_key_down_event(),
            KeyEventType::Released => ev.is_key_up_event(),
            KeyEventType::Down => ev.is_key_on_event(),
        };
    }
}

pub(crate) type Modifers = u32;

pub(crate) const MODIFIER_ALT: Modifers = 1 << 0;
pub(crate) const MODIFIER_CTRL: Modifers = 1 << 1;
pub(crate) const MODIFIER_SHIFT: Modifers = 1 << 2;
pub(crate) const MODIFIER_WIN: Modifers = 1 << 3;
pub(crate) const MODIFIER_ESC: Modifers = 1 << 4;

/// Represents a single event. See https://www.kernel.org/doc/html/latest/input/input.html#event-interface
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputEvent {
    pub time_sec: i64,
    pub time_usec: i64,
    pub event_type: ec::EventType,
    pub code: i32,
    pub value: i32,
    modifiers: Modifers,
}

impl Display for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_syn_report() {
            return write!(f, "{{InputEvent: time={}.{:06} ===== SYN_REPORT =====}}", self.time_sec, self.time_usec);
        }
        let modifiers = if self.modifiers == 0 {
            String::new()
        } else {
            format!(
                " modifiers=[{}{}{}{}{}]",
                if self.with_alt() { "a" } else { "" },
                if self.with_ctrl() { "c" } else { "" },
                if self.with_shift() { "s" } else { "" },
                if self.with_winkey() { "w" } else { "" },
                if self.with_esc() { "e" } else { "" },
            )
        };

        return write!(
            f,
            "{{InputEvent: time={}.{:06} type={} code={} value={} [{:x}]{}}}",
            self.time_sec,
            self.time_usec,
            self.type_name(),
            self.code_name(),
            self.value,
            self.value,
            modifiers,
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
            modifiers: 0,
        };
    }

    pub fn new(event_type: ec::EventType, code: i32, value: i32) -> InputEvent {
        return InputEvent {
            time_sec: 0,
            time_usec: 0,
            event_type,
            code,
            value,
            modifiers: 0,
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
            modifiers: 0,
        };
    }

    pub fn type_name(&self) -> String {
        return match ec::get_type_name(self.event_type as i32) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown type {}]", self.event_type as i32),
        };
    }

    pub fn code_name(&self) -> String {
        return match ec::get_code_name(self.event_type as i32, self.code) {
            v if v.len() > 0 => v.to_string(),
            _ => format!("[Unknown code {}]", self.code),
        };
    }

    /// Set the modifiers. This struct itself doesn't set `modifiers` and instead it expects the
    /// upper layer sets them via this function. This is because we want to support multiple keyboards
    /// and pressing some modifiers on keyboard A and then press a key on keyboard B.
    pub(crate) fn set_modifiers(&mut self, alt: bool, ctrl: bool, shift: bool, winkey: bool, esc: bool) {
        self.modifiers = (if alt { MODIFIER_ALT } else { 0 })
            | (if ctrl { MODIFIER_CTRL } else { 0 })
            | (if shift { MODIFIER_SHIFT } else { 0 })
            | (if winkey { MODIFIER_WIN } else { 0 })
            | (if esc { MODIFIER_ESC } else { 0 })
            | 0;
    }

    pub fn with_alt(&self) -> bool {
        return (self.modifiers & MODIFIER_ALT) != 0;
    }

    pub fn with_ctrl(&self) -> bool {
        return (self.modifiers & MODIFIER_CTRL) != 0;
    }

    pub fn with_shift(&self) -> bool {
        return (self.modifiers & MODIFIER_SHIFT) != 0;
    }

    pub fn with_winkey(&self) -> bool {
        return (self.modifiers & MODIFIER_WIN) != 0;
    }

    pub fn with_esc(&self) -> bool {
        return (self.modifiers & MODIFIER_ESC) != 0;
    }

    pub fn with_modifiers(&self, modifiers: &str) -> bool {
        validate_modifiers(modifiers, "acswe*");

        let ignore_other_modifiers = modifiers.contains('*');

        let alt = modifiers.contains('a');
        let ctrl = modifiers.contains('c');
        let shift = modifiers.contains('s');
        let win = modifiers.contains('w');
        let esc = modifiers.contains('e');

        if self.with_alt() != alt && (alt || !ignore_other_modifiers) {
            return false;
        }

        if self.with_ctrl() != ctrl && (ctrl || !ignore_other_modifiers) {
            return false;
        }

        if self.with_shift() != shift && (shift || !ignore_other_modifiers) {
            return false;
        }

        if self.with_winkey() != win && (win || !ignore_other_modifiers) {
            return false;
        }

        if self.with_esc() != esc && (esc || !ignore_other_modifiers) {
            return false;
        }

        return true;
    }

    /// Return true if it's a SYN_REPORT event.
    #[inline]
    pub fn is_syn_report(&self) -> bool {
        return self.event_type == ec::EventType::EV_SYN && self.code == ec::SYN_REPORT && self.value == 0;
    }

    /// Return true if it's a key event.
    #[inline]
    pub fn is_key_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_KEY;
    }

    /// Return true if it's a key event of the given key.
    #[inline]
    pub fn is_key(&self, key: i32, modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && key == self.code && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of any of the given keys.
    #[inline]
    pub fn is_any_key(&self, keys: &[i32], modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && keys.contains(&self.code) && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of the given key and the value is 1.
    #[inline]
    pub fn is_key_down(&self, key: i32, modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 1 && key == self.code && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of the given key and the value is 0.
    #[inline]
    pub fn is_key_up(&self, key: i32, modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 0 && key == self.code && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of the given key and the value is 1 or 2.
    #[inline]
    pub fn is_key_on(&self, key: i32, modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value > 0 && key == self.code && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of any the given keys and the value is 1.
    #[inline]
    pub fn is_any_key_down(&self, keys: &[i32], modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 1 && keys.contains(&self.code) && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of any the given keys and the value is 0.
    #[inline]
    pub fn is_any_key_up(&self, keys: &[i32], modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 0 && keys.contains(&self.code) && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event of any the given keys and the value is 1 or 2.
    #[inline]
    pub fn is_any_key_on(&self, keys: &[i32], modifiers: &str) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value > 0 && keys.contains(&self.code) && self.with_modifiers(modifiers);
    }

    /// Return true if it's a key event and the value is 1.
    #[inline]
    pub fn is_key_down_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 1;
    }

    /// Return true if it's a key event and the value is 1 or 2.
    #[inline]
    pub fn is_key_on_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value > 0;
    }

    /// Return true if it's a key event and the value is 0.
    #[inline]
    pub fn is_key_up_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_KEY && self.value == 0;
    }

    #[inline]
    pub fn is_rep_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_REP;
    }

    #[inline]
    pub fn is_msc_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_MSC;
    }

    #[inline]
    pub fn is_rel_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_REL;
    }

    #[inline]
    pub fn is_abs_event(&self) -> bool {
        return self.event_type == ec::EventType::EV_ABS;
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
                modifiers: 0,
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
