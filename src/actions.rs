use crate::button::{Cycle, Event, Press};
use crate::listener::Listener;
use crate::{Action, CycleBuffer};
use evdev::Device;
use log::trace;
use ringbuffer::RingBuffer;

/// An iterator that yields actions from the smik gateway's button.
#[derive(Debug)]
pub struct Actions {
    listener: Listener,
    cycles: CycleBuffer,
    last_press: Option<Press>,
}

impl Actions {
    /// Listen for input events from a device.
    #[must_use]
    pub const fn new(listener: Listener) -> Self {
        Self {
            listener,
            cycles: CycleBuffer::new(),
            last_press: None,
        }
    }
}

impl Iterator for Actions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Ok(cycle) = self.listener.next()?.try_into() else {
                continue;
            };

            match cycle {
                Event::Press(press) => {
                    self.last_press.replace(press);
                }
                Event::Release(release) => {
                    if let Some(cycle) = self
                        .last_press
                        .take()
                        .and_then(|press| Cycle::try_new(press, release))
                    {
                        trace!("Button cycle: {cycle:?}");
                        self.cycles.push(cycle);
                    }
                }
            }

            if let Ok(event) = Action::try_from(&self.cycles) {
                self.cycles.clear();
                trace!("Smik event: {event:?}");
                return Some(event);
            }
        }
    }
}

impl From<Device> for Actions {
    fn from(device: Device) -> Self {
        Self::new(Listener::new(device))
    }
}
