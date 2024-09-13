#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryFromEventBufferError {
    NotASmikEvent,
}
