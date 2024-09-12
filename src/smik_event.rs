use crate::ButtonPressEvent;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use std::ops::Range;
use std::time::Duration;

const LOG_DUMP_SPAN: Range<Duration> = Duration::from_secs(5)..Duration::from_secs(20);
const RESET_TIMEFRAME: Duration = Duration::from_secs(10);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SmikEvent {
    Reset,
    LogDump,
}

impl TryFrom<&ConstGenericRingBuffer<ButtonPressEvent, 5>> for SmikEvent {
    type Error = ();

    fn try_from(events: &ConstGenericRingBuffer<ButtonPressEvent, 5>) -> Result<Self, Self::Error> {
        if events.len() == 5 {
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

        Err(())
    }
}

impl TryFrom<&ButtonPressEvent> for SmikEvent {
    type Error = ();

    fn try_from(event: &ButtonPressEvent) -> Result<Self, Self::Error> {
        if let Ok(duration) = event.duration() {
            if LOG_DUMP_SPAN.contains(&duration) {
                return Ok(Self::LogDump);
            }
        }

        Err(())
    }
}
