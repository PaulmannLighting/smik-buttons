use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Button {
    event_device: PathBuf,
}

impl Button {
    #[must_use]
    pub const fn new(event_device: PathBuf) -> Self {
        Self { event_device }
    }

    pub fn listen(&self) -> std::io::Result<Listener> {
        OpenOptions::new()
            .read(true)
            .open(&self.event_device)
            .map(Listener::new)
    }
}

#[derive(Debug)]
pub struct Listener {
    file: File,
    buffer: [u8; 5],
}

impl Listener {
    #[must_use]
    pub const fn new(file: File) -> Self {
        Self {
            file,
            buffer: [0; 5],
        }
    }
}

impl Iterator for Listener {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.file.read_exact(&mut self.buffer).ok()?;
        Some(Event::Raw(self.buffer.into()))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Raw(Box<[u8]>),
}
