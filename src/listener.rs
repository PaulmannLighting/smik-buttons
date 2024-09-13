use evdev::{Device, InputEvent};
use log::error;
use std::fmt::Debug;
use std::vec::IntoIter;

pub struct Listener {
    device: Device,
    events: Option<IntoIter<InputEvent>>,
}

impl Listener {
    #[must_use]
    pub const fn new(device: Device) -> Self {
        Self {
            device,
            events: None,
        }
    }
}

impl Debug for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Listener")
            .field("device", &"...")
            .field("events", &self.events)
            .finish()
    }
}

impl Iterator for Listener {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(events) = &mut self.events {
                if let Some(event) = events.next() {
                    return Some(event);
                }
            }

            self.events.replace(
                self.device
                    .fetch_events()
                    .inspect_err(|error| error!("Failed to fetch input events: {error}"))
                    .ok()?
                    .collect::<Vec<_>>()
                    .into_iter(),
            );
        }
    }
}
