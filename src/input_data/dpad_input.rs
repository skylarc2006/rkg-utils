/// Errors that can occur while parsing a [`DPadButton`].
#[derive(thiserror::Error, Debug)]
pub enum DPadButtonError {
    /// The extracted button value did not correspond to any known D-pad direction.
    #[error("Non Existent DPad Button")]
    NonExistentDPadButton,
}

/// A D-pad button state for a single input entry in a Mario Kart Wii ghost file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPadButton {
    /// No D-pad button is pressed.
    None,
    Up,
    Down,
    Left,
    Right,
}

/// Extracts a [`DPadButton`] from the upper nibble of a packed input byte.
///
/// The button ID is stored in bits 4–6 of the byte (mask `0x70`).
///
/// # Errors
///
/// Returns [`DPadButtonError::NonExistentDPadButton`] if the extracted 3-bit
/// value does not map to a known D-pad direction (valid range is 0–4).
pub fn parse_dpad_button(value: u8) -> Result<DPadButton, DPadButtonError> {
    let button = (value & 0x70) >> 4;

    match button {
        0 => Ok(DPadButton::None),
        1 => Ok(DPadButton::Up),
        2 => Ok(DPadButton::Down),
        3 => Ok(DPadButton::Left),
        4 => Ok(DPadButton::Right),
        _ => Err(DPadButtonError::NonExistentDPadButton),
    }
}

/// Errors that can occur while parsing a [`DPadInput`].
#[derive(thiserror::Error, Debug)]
pub enum DPadInputError {
    /// The raw bytes did not form a valid D-pad input entry.
    #[error("Invalid DPad Input")]
    InvalidDpadInput,
    /// The D-pad button value could not be parsed.
    #[error("Invalid DPad Button: {0}")]
    InvalidButton(#[from] DPadButtonError),
}

/// A single run-length encoded D-pad input entry from a Mario Kart Wii ghost file.
///
/// Each entry records which D-pad button was held and for how many consecutive
/// frames it was held. Frame durations exceeding 255 frames are encoded across
/// two fields: the lower nibble of the first byte stores the number of
/// additional 256-frame intervals, and the second byte stores the remainder.
#[derive(Debug)]
pub struct DPadInput {
    /// The D-pad button that was held during this input entry.
    button: DPadButton,
    /// The number of frames this D-pad state was held.
    frame_duration: u32,
}

impl DPadInput {
    /// Returns the D-pad button held during this input entry.
    pub fn button(&self) -> DPadButton {
        self.button
    }

    /// Returns the number of frames this D-pad state was held.
    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }
}

/// Two [`DPadInput`] values are equal if they share the same button state,
/// regardless of frame duration.
impl PartialEq for DPadInput {
    fn eq(&self, other: &DPadInput) -> bool {
        self.button == other.button
    }
}

/// Parses a [`DPadInput`] from a 2-byte slice.
///
/// # Errors
///
/// Returns [`DPadInputError::InvalidButton`] if the button bits do not map to
/// a known D-pad direction.
impl TryFrom<&[u8]> for DPadInput {
    type Error = DPadInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let button = parse_dpad_button(value[0])?;
        // 0x0F bit mask used if this input state was held longer than 255 frames, gives amount of 256-frame intervals to add to frame duration
        let previous_full_byte_presses: u32 = (value[0] & 0x0F).into();
        let frame_duration = value[1] as u32 + (previous_full_byte_presses * 256);

        Ok(Self {
            button,
            frame_duration,
        })
    }
}
