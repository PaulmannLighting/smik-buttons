mod button_press_event;
mod event;
mod events;
mod input_event_ext;
mod listener;

pub use button_press_event::ButtonPressEvent;
pub use event::Event;
pub use events::Events;
pub use input_event_ext::InputEventExt;
pub use listener::Listener;
use ringbuffer::ConstGenericRingBuffer;

const MAX_EVENTS: usize = 5;
pub type EventBuffer = ConstGenericRingBuffer<ButtonPressEvent, MAX_EVENTS>;
