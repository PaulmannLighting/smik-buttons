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
    type Error = ();

    fn try_from(event: InputEvent) -> Result<Self, Self::Error> {
        if !matches!(event.kind(), InputEventKind::Key(_)) {
            return Err(());
        }

        match event.value() {
            1 => Ok(Self::Down(event.timestamp())),
            0 => Ok(Self::Up(event.timestamp())),
            _ => Err(()),
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
    #[must_use]
    pub fn try_new(down: Press, up: Press) -> Option<Self> {
        if let Press::Down(down) = down {
            if let Press::Up(up) = up {
                if down < up {
                    return Some(Self {
                        down: Press::Down(down),
                        up: Press::Up(up),
                    });
                }
            }
        }
        None
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
