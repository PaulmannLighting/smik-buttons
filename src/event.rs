use crate::EventBuffer;
use ringbuffer::RingBuffer;
use std::ops::Range;
use std::time::Duration;

const LOG_DUMP_SPAN: Range<Duration> = Duration::from_secs(5)..Duration::from_secs(20);
const RESET_TIMEFRAME: Duration = Duration::from_secs(10);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Reset,
    LogDump,
}

impl TryFrom<&EventBuffer> for Event {
    type Error = TryFromEventBufferError;

    fn try_from(events: &EventBuffer) -> Result<Self, Self::Error> {
        if events.is_reset_event() {
            Ok(Self::Reset)
        } else if events.is_log_dump_event() {
            Ok(Self::LogDump)
        } else {
            Err(TryFromEventBufferError::NotASmikEvent)
        }
    }
}

trait EventBufferExt {
    fn is_reset_event(&self) -> bool;
    fn is_log_dump_event(&self) -> bool;
}

impl EventBufferExt for EventBuffer {
    fn is_reset_event(&self) -> bool {
        if !self.is_full() {
            return false;
        }

        if let Some(duration) = self.iter().last().and_then(|last| {
            self.iter()
                .next()
                .and_then(|first| last.end().duration_since(first.start()).ok())
        }) {
            return duration < RESET_TIMEFRAME;
        }

        false
    }

    fn is_log_dump_event(&self) -> bool {
        if let Some(duration) = self.iter().last().and_then(|event| event.duration().ok()) {
            return LOG_DUMP_SPAN.contains(&duration);
        }

        false
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryFromEventBufferError {
    NotASmikEvent,
}
