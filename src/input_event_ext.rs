use evdev::{InputEvent, InputEventKind};

pub trait InputEventExt {
    fn is_button_event(&self) -> bool;
    fn is_button_down(&self) -> bool;
    fn is_button_up(&self) -> bool;
}

impl InputEventExt for InputEvent {
    fn is_button_event(&self) -> bool {
        matches!(self.kind(), InputEventKind::Key(_))
    }

    fn is_button_down(&self) -> bool {
        self.is_button_event() && self.value() == 1
    }

    fn is_button_up(&self) -> bool {
        self.is_button_event() && self.value() == 0
    }
}
