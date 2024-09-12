use evdev::Device;
use smik_buttons::Listener;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    for event in Listener::spawn(Device::open(BUTTON).expect("Failed to open button device")) {
        println!("Read event: {event:?}");
    }
}
