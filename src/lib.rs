mod button_press_event;
mod input_event_ext;
mod listener;
mod smik_event;
mod smik_events;

pub use button_press_event::ButtonPressEvent;
pub use input_event_ext::InputEventExt;
pub use listener::Listener;
use ringbuffer::ConstGenericRingBuffer;
pub use smik_event::SmikEvent;
pub use smik_events::SmikEvents;

const MAX_EVENTS: usize = 5;
pub type EventBuffer = ConstGenericRingBuffer<ButtonPressEvent, MAX_EVENTS>;
