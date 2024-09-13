use super::{Press, Release, TryFromInputEventError};
use evdev::{InputEvent, InputEventKind};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Press(Press),
    Release(Release),
}

impl TryFrom<InputEvent> for Event {
    type Error = TryFromInputEventError;

    fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
        if !matches!(event.kind(), InputEventKind::Key(_)) {
            return Err(TryFromInputEventError::NotAButtonEvent);
        }

        match event.value() {
            0 => Ok(Self::Release(Release(event.timestamp()))),
            1 => Ok(Self::Press(Press(event.timestamp()))),
            _ => Err(TryFromInputEventError::InvalidEvent),
        }
    }
}
