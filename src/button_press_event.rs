use evdev::InputEvent;
use std::time::{Duration, SystemTime, SystemTimeError};

#[derive(Debug)]
pub struct ButtonPressEvent {
    button_down_event: InputEvent,
    button_up_event: InputEvent,
}

impl ButtonPressEvent {
    /// Create a new button press event.
    #[must_use]
    pub fn try_new(button_down_event: InputEvent, button_up_event: InputEvent) -> Option<Self> {
        if button_down_event.value() == 1
            && button_up_event.value() == 0
            && button_down_event.timestamp() < button_up_event.timestamp()
        {
            Some(Self {
                button_down_event,
                button_up_event,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn start(&self) -> SystemTime {
        self.button_down_event.timestamp()
    }

    #[must_use]
    pub fn end(&self) -> SystemTime {
        self.button_up_event.timestamp()
    }

    /// Calculate the duration of the button press event.
    ///
    /// # Errors
    /// This function may return an error if the duration calculation fails.
    pub fn duration(&self) -> Result<Duration, SystemTimeError> {
        self.button_up_event
            .timestamp()
            .duration_since(self.button_down_event.timestamp())
    }
}
