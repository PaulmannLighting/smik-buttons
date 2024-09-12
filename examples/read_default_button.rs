use smik_buttons::Button;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

fn main() {
    for event in Button::new(BUTTON.into())
        .listen()
        .expect("Failed to listen to button events")
    {
        println!("{event:?}");
    }
}
