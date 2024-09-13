mod action;
mod button;
mod events;
mod listener;

pub use action::{Action, TryFromEventBufferError};
use button::Cycle;
pub use events::Events;
use ringbuffer::ConstGenericRingBuffer;

const MAX_EVENTS: usize = 5;
type CycleBuffer = ConstGenericRingBuffer<Cycle, MAX_EVENTS>;
