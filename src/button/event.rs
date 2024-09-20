use super::{Press, Release};
pub use error::TryFromInputEventError;
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

mod error {
    use std::error::Error;
    use std::fmt::Display;

    #[allow(clippy::module_name_repetitions)]
    #[derive(Clone, Copy, Debug)]
    pub enum TryFromInputEventError {
        InvalidEvent,
        NotAButtonEvent,
    }

    impl Display for TryFromInputEventError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::InvalidEvent => write!(f, "invalid event"),
                Self::NotAButtonEvent => write!(f, "not a button event"),
            }
        }
    }

    impl Error for TryFromInputEventError {}
}
