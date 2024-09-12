use evdev::Device;
use smik_buttons::Listener;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    let (_listener, events) =
        Listener::spawn(Device::open(BUTTON).expect("Failed to open button device"));

    for event in events {
        println!("Read event: {event:?}");
    }
}
