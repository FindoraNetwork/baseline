use alloc::{string::String, vec::Vec};

pub struct EventAttributes {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

pub struct Event {
    pub ty: String,
    pub attributes: Vec<EventAttributes>,
}
