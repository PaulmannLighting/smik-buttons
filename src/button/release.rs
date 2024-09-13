use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Release(pub(crate) SystemTime);
