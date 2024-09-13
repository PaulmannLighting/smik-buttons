use clap::Parser;
use evdev::Device;
use smik_buttons::Actions;

const BUTTON: &str = "/dev/input/by-path/platform-gpio-keys-event";

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = BUTTON)]
    button: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    for event in Actions::from(Device::open(args.button).expect("Failed to open button device")) {
        println!("Read event: {event:?}");
    }
}
