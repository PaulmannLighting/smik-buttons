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
    type Error = ();

    fn try_from(events: &EventBuffer) -> Result<Self, Self::Error> {
        if events.is_full() {
            if let Ok(duration) = events
                .iter()
                .last()
                .ok_or(())?
                .end()
                .duration_since(events.iter().next().ok_or(())?.start())
            {
                if duration < RESET_TIMEFRAME {
                    return Ok(Self::Reset);
                }
            }
        }

        if let Some(duration) = events.iter().last().and_then(|event| event.duration().ok()) {
            if LOG_DUMP_SPAN.contains(&duration) {
                return Ok(Self::LogDump);
            }
        }

        Err(())
    }
}
