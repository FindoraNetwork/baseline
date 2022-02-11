use crate::types;

pub trait Event {
    fn to_event(self) -> types::Event;
}
