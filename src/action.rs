mod cycle_buffer_ext;
mod errors;

use crate::CycleBuffer;
use cycle_buffer_ext::CycleBufferExt;
pub use errors::TryFromEventBufferError;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Reset,
    LogDump,
}

impl TryFrom<&CycleBuffer> for Action {
    type Error = TryFromEventBufferError;

    fn try_from(events: &CycleBuffer) -> Result<Self, Self::Error> {
        if events.is_reset_event() {
            Ok(Self::Reset)
        } else if events.is_log_dump_event() {
            Ok(Self::LogDump)
        } else {
            Err(TryFromEventBufferError::NotASmikEvent)
        }
    }
}
