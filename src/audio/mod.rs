use std::fmt::{Display, Formatter, Write};
use rom_loaders_rs::multimedia::WavContent;

pub mod mixer;
mod sound_driver;
pub use mixer::{SoundMixer, Sound, SoundId};

#[derive(Debug, Clone, Copy)]
/// error produced when creating the [`SoundDriver`]
pub enum SoundError {
    /// sound initialization was a success
    NoError,
    /// no sound device was found
    NoDevice,
    /// could not create an output stream
    OutputStream,
    /// unsupported output stream format
    UnknownStreamFormat,
}
impl Display for SoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundError::NoError => f.write_str("No error")?,
            SoundError::NoDevice => f.write_str("No device!")?,
            SoundError::OutputStream => f.write_str("Failed on output stream creation!")?,
            SoundError::UnknownStreamFormat => f.write_str("Unknown stream format!")?,
        }
        Ok(())
    }
}