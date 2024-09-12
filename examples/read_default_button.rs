use evdev::Device;
use smik_buttons::listen;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    for event in listen(
        Device::open(BUTTON).expect("Failed to open button device"),
        Arc::new(AtomicBool::new(true)),
    ) {
        println!("Read event: {event:?}");
    }
}
