use evdev::Device;
use smik_buttons::Events;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    env_logger::init();

    for event in Events::from(Device::open(BUTTON).expect("Failed to open button device")) {
        println!("Read event: {event:?}");
    }
}
