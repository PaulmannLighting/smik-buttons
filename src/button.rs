use evdev::{InputEvent, InputEventKind};
use std::time::{Duration, SystemTime, SystemTimeError};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Press {
    Down(SystemTime),
    Up(SystemTime),
}

impl Press {
    #[must_use]
    pub const fn timestamp(&self) -> SystemTime {
        match self {
            Self::Down(timestamp) | Self::Up(timestamp) => *timestamp,
        }
    }
}

impl TryFrom<InputEvent> for Press {
    type Error = TryFromInputEventError;

    fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
        if !matches!(event.kind(), InputEventKind::Key(_)) {
            return Err(TryFromInputEventError::NotAButtonEvent(event));
        }

        match event.value() {
            1 => Ok(Self::Down(event.timestamp())),
            0 => Ok(Self::Up(event.timestamp())),
            _ => Err(TryFromInputEventError::InvalidEventType(event)),
        }
    }
}

#[derive(Debug)]
pub struct Cycle {
    down: Press,
    up: Press,
}

impl Cycle {
    /// Create a new button press event.
    ///
    /// # Errors
    /// This function may return an error if the button press events are invalid.
    pub fn try_new(down: Press, up: Press) -> Result<Self, TryNewError> {
        if let Press::Down(down_time) = down {
            if let Press::Up(up_time) = up {
                if down_time < up_time {
                    Ok(Self { down, up })
                } else {
                    Err(TryNewError::DownNotBeforeUp(down, up))
                }
            } else {
                Err(TryNewError::NotAnUpEvent(up))
            }
        } else {
            Err(TryNewError::NotADownEvent(down))
        }
    }

    #[must_use]
    pub const fn start(&self) -> SystemTime {
        self.down.timestamp()
    }

    #[must_use]
    pub const fn end(&self) -> SystemTime {
        self.up.timestamp()
    }

    /// Calculate the duration of the button press event.
    ///
    /// # Errors
    /// This function may return an error if the duration calculation fails.
    pub fn duration(&self) -> Result<Duration, SystemTimeError> {
        self.up.timestamp().duration_since(self.down.timestamp())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TryFromInputEventError {
    NotAButtonEvent(InputEvent),
    InvalidEventType(InputEvent),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryNewError {
    NotADownEvent(Press),
    NotAnUpEvent(Press),
    DownNotBeforeUp(Press, Press),
}
