#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// An error that occurs when trying to parse an [`crate::Action`].
pub enum TryFromEventBufferError {
    /// The event is not a smik event.
    NotASmikEvent,
}
