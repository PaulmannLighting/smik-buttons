#[derive(Clone, Copy, Debug)]
pub enum TryFromInputEventError {
    InvalidEvent,
    NotAButtonEvent,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryNewError {
    PressNotBeforeRelease,
}
