// Generated from /usr/include/linux/input-event-codes.h

include! {"linux_input-event-codes.h.rs"}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum EventType {
    EV_SYN = EV_SYN,
    EV_KEY = EV_KEY,
    EV_REL = EV_REL,
    EV_ABS = EV_ABS,
    EV_MSC = EV_MSC,
    EV_SW = EV_SW,
    EV_LED = EV_LED,
    EV_SND = EV_SND,
    EV_REP = EV_REP,
    EV_FF = EV_FF,
    EV_PWR = EV_PWR,
    EV_FF_STATUS = EV_FF_STATUS,
}

impl EventType {
    pub fn from_i32(type_: i32) -> EventType {
        match type_ {
            EV_KEY => EventType::EV_KEY,
            EV_SYN => EventType::EV_SYN,
            EV_REL => EventType::EV_REL,
            EV_ABS => EventType::EV_ABS,
            EV_MSC => EventType::EV_MSC,
            EV_SW => EventType::EV_SW,
            EV_LED => EventType::EV_LED,
            EV_SND => EventType::EV_SND,
            EV_REP => EventType::EV_REP,
            EV_FF => EventType::EV_FF,
            EV_PWR => EventType::EV_PWR,
            EV_FF_STATUS => EventType::EV_FF_STATUS,
            _ => panic!("Unknown event type: {}", type_),
        }
    }
}
