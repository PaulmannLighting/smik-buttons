mod button;
mod event;
mod events;
mod listener;

pub use button::{Cycle, Press, TryFromInputEventError, TryNewError};
pub use event::{Event, TryFromEventBufferError};
pub use events::Events;
pub use listener::Listener;
use ringbuffer::ConstGenericRingBuffer;

const MAX_EVENTS: usize = 5;
pub type EventBuffer = ConstGenericRingBuffer<Cycle, MAX_EVENTS>;
