use std::fmt::Display;

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

/// Errors that can occur while deserializing an [`InGameTime`].
#[derive(thiserror::Error, Debug)]
pub enum InGameTimeError {
    /// An in game time element is too large to store in ghost data.
    #[error("In game time element too large (exceeds maximum amount which can be stored)")]
    InGameTimeElementTooLarge,
    /// The input iterator did not contain enough bytes to extract a time value.
    #[error("Insufficiently Long Iterator")]
    InsufficientlyLongIterator,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// A lap or finish time recorded in a Mario Kart Wii ghost file, expressed as
/// minutes, seconds, and milliseconds.
///
/// Not all combinations of fields are semantically meaningful; use
/// [`is_technically_valid`](InGameTime::is_technically_valid) to check whether
/// the time falls within the bounds expected during normal gameplay.
///
/// [`InGameTime`] values can be added together and summed via the standard
/// [`Add`](std::ops::Add) and [`Sum`](std::iter::Sum) traits, which convert
/// through total milliseconds to avoid per-field overflow.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct InGameTime {
    /// Minutes component (0–99 in normal play).
    minutes: u8,
    /// Seconds component (0–59 in normal play).
    seconds: u8,
    /// Milliseconds component (0–999 in normal play).
    milliseconds: u16,
}

impl InGameTime {
    /// Creates a new [`InGameTime`] from raw minutes, seconds, and milliseconds.
    ///
    /// Returns [`InGameTimeError::InGameTimeElementTooLarge`] if one or more of the given arguments
    /// is too large to fit inside its bit representation in a ghost file
    /// (if minutes > 127, if seconds > 127, or if milliseconds > 1023).
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Result<Self, InGameTimeError> {
        if minutes > 127 || seconds > 127 || milliseconds > 1023 {
            Err(InGameTimeError::InGameTimeElementTooLarge)
        } else {
            Ok(Self {
                minutes,
                seconds,
                milliseconds,
            })
        }
    }

    /// Creates a new, valid [`InGameTime`] from milliseconds.
    ///
    /// Returns [`InGameTimeError::InGameTimeElementTooLarge`] if the resulting time has a minutes value over 127.
    pub fn from_milliseconds(milliseconds: u32) -> Result<Self, InGameTimeError> {
        let millis = (milliseconds % 1000) as u16;
        let seconds = ((milliseconds / 1000) % 60) as u8;
        let minutes = ((milliseconds / 60000) % 60) as u8;

        Self::new(minutes, seconds, millis)
    }

    /// Returns the minutes component of the time.
    pub fn minutes(self) -> u8 {
        self.minutes
    }

    /// Returns the seconds component of the time.
    pub fn seconds(self) -> u8 {
        self.seconds
    }

    /// Returns the milliseconds component of the time.
    pub fn milliseconds(self) -> u16 {
        self.milliseconds
    }

    /// Returns `true` if the time falls within possible in-game bounds.
    ///
    /// A time is considered valid when minutes ≤ 99, seconds ≤ 59,
    /// and milliseconds ≤ 999. Ghost files that have a time outside of these
    /// bounds crash Mario Kart Wii upon loading them either by racing or watching them.
    pub fn is_valid(self) -> bool {
        self.minutes <= 99 && self.seconds <= 59 && self.milliseconds <= 999
    }

    /// Converts the time to a total number of milliseconds.
    pub fn to_milliseconds(self) -> u32 {
        (self.milliseconds as u32) + (self.seconds as u32) * 1000 + (self.minutes as u32) * 60000
    }
}

/// Converts a [`InGameTime`] to its raw-data representation.
///
/// The bits are laid out as follows, where `M` = minutes, `S` = seconds,
/// and `C` = milliseconds:
/// ```text
/// Byte 1: MMMMMMMMS
/// Byte 2: SSSSSSCC
/// Byte 3: CCCCCCCC
/// ```
/// Minutes occupy 7 bits, seconds 7 bits, and milliseconds 10 bits.
impl From<InGameTime> for [u8; 3] {
    fn from(value: InGameTime) -> Self {
        let minutes = value.minutes();
        let seconds = value.seconds();
        let milliseconds = value.milliseconds();

        [
            (minutes << 1) | (seconds >> 6),
            ((seconds & 0x3F) << 2) | (milliseconds >> 8) as u8,
            (milliseconds & 0xFF) as u8,
        ]
    }
}

/// Formats the time as `MM:SS.mmm` (e.g. `"01:23.456"`).
impl Display for InGameTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:03}",
            self.minutes, self.seconds, self.milliseconds
        )
    }
}

/// Deserializes an [`InGameTime`] from 3 packed bytes.
///
/// The bits are laid out as follows, where `M` = minutes, `S` = seconds,
/// and `C` = milliseconds:
/// ```text
/// Byte 1: MMMMMMMMS
/// Byte 2: SSSSSSCC
/// Byte 3: CCCCCCCC
/// ```
/// Minutes occupy 7 bits, seconds 7 bits, and milliseconds 10 bits.
impl FromByteHandler for InGameTime {
    type Err = InGameTimeError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        handler.shift_right(1);
        let minutes = handler.copy_byte(0);
        let seconds = handler.copy_byte(1) >> 1;
        handler.shift_left(9);

        Ok(Self {
            minutes,
            seconds,
            milliseconds: handler.copy_word(0) & 0x3FF,
        })
    }
}

/// Adds two [`InGameTime`] values by converting to total milliseconds and back.
impl std::ops::Add for InGameTime {
    type Output = InGameTime;

    fn add(self, rhs: InGameTime) -> InGameTime {
        let total_millis = self.to_milliseconds() + rhs.to_milliseconds();

        let milliseconds = (total_millis % 1000) as u16;
        let total_seconds = total_millis / 1000;
        let seconds = (total_seconds % 60) as u8;
        let minutes = (total_seconds / 60) as u8;

        InGameTime::new(minutes, seconds, milliseconds).unwrap_or_default()
    }
}

/// Sums an iterator of [`InGameTime`] values, starting from [`InGameTime::default`] (zero).
impl std::iter::Sum for InGameTime {
    fn sum<I: Iterator<Item = InGameTime>>(iter: I) -> Self {
        iter.fold(InGameTime::default(), |a, b| a + b)
    }
}
