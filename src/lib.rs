//! Handle events from pressing the smik gateway's button.
mod action;
mod actions;
mod button;
mod listener;

pub use action::{Action, TryFromEventBufferError};
pub use actions::Actions;
use button::Cycle;
use ringbuffer::ConstGenericRingBuffer;

const MAX_EVENTS: usize = 5;
type CycleBuffer = ConstGenericRingBuffer<Cycle, MAX_EVENTS>;
