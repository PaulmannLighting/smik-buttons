use evdev::{Device, InputEvent};
use log::{trace, warn};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver},
    Arc,
};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
pub struct Listener {
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
    rx: Receiver<InputEvent>,
}

impl Listener {
    /// Listen for input events from a device.
    ///
    /// # Panics
    /// This function may panic if it fails to send an event.
    #[must_use]
    pub fn spawn(mut device: Device) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (tx, rx) = channel();
        Self {
            running: running.clone(),
            thread: Some(spawn(move || {
                while running.load(Ordering::Relaxed) {
                    if let Ok(events) = device.fetch_events() {
                        for event in events {
                            tx.send(event).expect("Failed to send event");
                        }
                    }
                }
            })),
            rx,
        }
    }

    /// Stop listening for input events.
    ///
    /// # Panics
    /// This function may panic if it fails to join the listener thread.
    pub fn stop(mut self) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(thread) = self.thread.take() {
            thread.join().expect("Failed to join listener thread");
        }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(thread) = self.thread.take() {
            thread.join().expect("Failed to join listener thread");
        }
    }
}

impl Iterator for Listener {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx
            .recv()
            .inspect(|event| trace!("{event:?}"))
            .inspect_err(|error| warn!("{error}"))
            .ok()
    }
}
