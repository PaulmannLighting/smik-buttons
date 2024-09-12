use evdev::Device;
use smik_buttons::SmikEvents;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    for event in
        SmikEvents::from(Device::open(BUTTON).expect("Failed to open button device")).flatten()
    {
        println!("Read event: {event:?}");
    }
}
