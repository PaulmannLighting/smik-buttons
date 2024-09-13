use super::{Press, Release, TryNewError};
use std::time::{Duration, SystemTime, SystemTimeError};

/// A button cycle consisting of a press and a release event.
#[derive(Debug)]
pub struct Cycle {
    press: Press,
    release: Release,
}

impl Cycle {
    /// Create a new button cycle.
    ///
    /// # Errors
    /// This function may return an error if the button press events are invalid.
    pub fn try_new(press: Press, release: Release) -> Result<Self, TryNewError> {
        if press.0 < release.0 {
            Ok(Self { press, release })
        } else {
            Err(TryNewError::PressNotBeforeRelease)
        }
    }

    #[must_use]
    pub const fn start(&self) -> SystemTime {
        self.press.0
    }

    #[must_use]
    pub const fn end(&self) -> SystemTime {
        self.release.0
    }

    /// Calculate the duration between the button press and release events.
    ///
    /// # Errors
    /// This function may return an error if the duration calculation fails.
    pub fn duration(&self) -> Result<Duration, SystemTimeError> {
        self.end().duration_since(self.start())
    }
}
