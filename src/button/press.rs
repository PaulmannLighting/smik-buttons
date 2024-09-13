use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Press(pub(crate) SystemTime);
