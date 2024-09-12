use evdev::{Device, InputEvent};
use std::sync::mpsc::Sender;
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
}

impl Listener {
    /// Listen for input events from a device.
    ///
    /// # Panics
    /// This function may panic if it fails to send an event.
    #[must_use]
    pub fn spawn(device: Device) -> (Self, Receiver<InputEvent>) {
        let (tx, rx) = channel();
        (Self::new(Arc::new(AtomicBool::new(true)), device, tx), rx)
    }

    fn new(running: Arc<AtomicBool>, mut device: Device, tx: Sender<InputEvent>) -> Self {
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
