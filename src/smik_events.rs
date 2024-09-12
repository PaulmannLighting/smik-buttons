use crate::{ButtonPressEvent, EventBuffer, InputEventExt, Listener, SmikEvent};
use evdev::{Device, InputEvent};
use log::trace;
use ringbuffer::RingBuffer;

#[derive(Debug)]
pub struct SmikEvents {
    listener: Listener,
    events: EventBuffer,
    last_button_down_event: Option<InputEvent>,
    last_button_up_event: Option<InputEvent>,
}

impl SmikEvents {
    /// Listen for input events from a device.
    #[must_use]
    pub fn new(listener: Listener) -> Self {
        Self {
            listener,
            events: EventBuffer::new(),
            last_button_down_event: None,
            last_button_up_event: None,
        }
    }
}

impl Iterator for SmikEvents {
    type Item = Option<SmikEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.listener.next()?;

        if event.is_button_down() {
            self.last_button_down_event.replace(event);
        } else if event.is_button_up() {
            self.last_button_up_event.replace(event);
        }

        if let Some(event) = self.last_button_up_event.take().and_then(|up| {
            self.last_button_down_event
                .take()
                .and_then(|down| ButtonPressEvent::try_new(down, up))
        }) {
            trace!("Button press event: {event:?}");
            self.events.push(event);
        }

        if let Ok(event) = SmikEvent::try_from(&self.events) {
            self.events.clear();
            trace!("Smik event: {event:?}");
            Some(Some(event))
        } else {
            Some(None)
        }
    }
}

impl From<Device> for SmikEvents {
    fn from(device: Device) -> Self {
        Self::new(Listener::spawn(device))
    }
}
