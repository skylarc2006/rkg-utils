/// Errors that can occur while parsing a [`StickInput`].
#[derive(thiserror::Error, Debug)]
pub enum StickInputError {
    /// One or both raw axis values exceeded the maximum encoded value of 14.
    #[error("Invalid Stick Input")]
    InvalidStickInput,
}

/// A single encoded analog stick input entry from a Mario Kart Wii ghost file.
///
/// Each entry records the stick position and for how many consecutive frames it
/// was held. Both axes are encoded in a single byte as 4-bit unsigned values
/// (0–14), then shifted to the signed range −7 to +7 for intuitive use.
///
/// Negative `x` values represent left; positive values represent right.
/// Negative `y` values represent down; positive values represent up.
#[derive(Debug)]
pub struct StickInput {
    /// Horizontal axis position in the range −7 (full left) to +7 (full right).
    x: i8,
    /// Vertical axis position in the range −7 (full down) to +7 (full up).
    y: i8,
    /// The number of frames this stick position was held.
    frame_duration: u32,
}

impl StickInput {
    /// Returns the horizontal axis position (−7 to +7).
    pub fn x(&self) -> i8 {
        self.x
    }

    /// Returns the vertical axis position (−7 to +7).
    pub fn y(&self) -> i8 {
        self.y
    }

    /// Returns the number of frames this stick position was held.
    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    /// Sets the number of frames this stick position was held.
    pub fn set_frame_duration(&mut self, frame_duration: u32) {
        self.frame_duration = frame_duration;
    }
}

/// Two [`StickInput`] values are equal if their `x` and `y` positions match,
/// regardless of frame duration.
impl PartialEq for StickInput {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Compares a [`StickInput`] against a `[i8; 2]` array of `[x, y]`.
impl PartialEq<[i8; 2]> for StickInput {
    fn eq(&self, other: &[i8; 2]) -> bool {
        self.x == other[0] && self.y == other[1]
    }
}

/// Parses a [`StickInput`] from a 2-byte slice.
///
/// # Errors
///
/// Returns [`StickInputError::InvalidStickInput`] if either axis value exceeds 14.
impl TryFrom<&[u8]> for StickInput {
    type Error = StickInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let x = (value[0] & 0xF0) >> 4;
        let y = value[0] & 0x0F;

        if x > 14 || y > 14 {
            return Err(StickInputError::InvalidStickInput);
        }

        // store x and y as ranging from -7 to +7, as that's more intuitive for left/right or up/down
        let x = x as i8 - 7;
        let y = y as i8 - 7;

        let frame_duration = value[1] as u32;

        Ok(Self {
            x,
            y,
            frame_duration,
        })
    }
}
