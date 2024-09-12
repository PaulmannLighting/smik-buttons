use evdev::{Device, InputEvent};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver},
    Arc,
};
use std::thread::spawn;

/// Listen for input events from a device.
///
/// # Panics
/// This function may panic if it fails to send an event.
pub fn listen(mut device: Device, running: Arc<AtomicBool>) -> Receiver<InputEvent> {
    let (tx, rx) = channel();

    spawn(move || {
        while running.load(Ordering::Relaxed) {
            if let Ok(events) = device.fetch_events() {
                for event in events {
                    tx.send(event).expect("Failed to send event");
                }
            }
        }
    });

    rx
}
