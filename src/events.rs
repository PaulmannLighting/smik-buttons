use crate::{Cycle, Event, EventBuffer, Listener, Press};
use evdev::Device;
use log::trace;
use ringbuffer::RingBuffer;

#[derive(Debug)]
pub struct Events {
    listener: Listener,
    events: EventBuffer,
    last_button_down_event: Option<Press>,
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
            let Ok(press) = self.listener.next()?.try_into() else {
                continue;
            };

            match press {
                Press::Down(timestamp) => {
                    self.last_button_down_event.replace(Press::Down(timestamp));
                }
                Press::Up(timestamp) => {
                    if let Some(cycle) = self
                        .last_button_down_event
                        .take()
                        .and_then(|down| Cycle::try_new(down, Press::Up(timestamp)).ok())
                    {
                        trace!("Button press event: {cycle:?}");
                        self.events.push(cycle);
                    }
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
