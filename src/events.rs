use crate::{ButtonPressEvent, Event, EventBuffer, InputEventExt, Listener};
use evdev::{Device, InputEvent};
use log::trace;
use ringbuffer::RingBuffer;

#[derive(Debug)]
pub struct Events {
    listener: Listener,
    events: EventBuffer,
    last_button_down_event: Option<InputEvent>,
}

impl Events {
    /// Listen for input events from a device.
    #[must_use]
    pub fn new(listener: Listener) -> Self {
        Self {
            listener,
            events: EventBuffer::new(),
            last_button_down_event: None,
        }
    }
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.listener.next()?;

            if event.is_button_down() {
                self.last_button_down_event.replace(event);
            } else if event.is_button_up() {
                if let Some(event) = self
                    .last_button_down_event
                    .take()
                    .and_then(|down| ButtonPressEvent::try_new(down, event))
                {
                    trace!("Button press event: {event:?}");
                    self.events.push(event);
                }
            }

            if let Ok(event) = Event::try_from(&self.events) {
                self.events.clear();
                trace!("Smik event: {event:?}");
                return Some(event);
            }
        }
    }
}

impl From<Device> for Events {
    fn from(device: Device) -> Self {
        Self::new(Listener::spawn(device))
    }
}
