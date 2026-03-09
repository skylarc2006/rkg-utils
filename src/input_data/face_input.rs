/// Errors that can occur while parsing a [`FaceButton`] combination.
#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    /// The face button byte was non-zero but did not match any known button bit.
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
}

/// A face button that can be active during a single input entry.
///
/// Multiple buttons may be simultaneously active in a single [`FaceInput`];
/// see [`parse_face_buttons`] for the bitmask layout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceButton {
    /// Accelerator button (bit 0, mask `0x01`).
    Accelerator,
    /// Brake button (bit 1, mask `0x02`).
    Brake,
    /// Drift button (bit 3, mask `0x08`).
    Drift,
    /// Combined brake and drift (bit 4, mask `0x10`).
    BrakeDrift,
    /// Item button (bit 2, mask `0x04`).
    Item,
    /// An unrecognized button bit was set in the upper nibble.
    Unknown,
}

/// Parses all active [`FaceButton`]s from a packed face-button byte.
///
/// Each bit in the byte corresponds to one or more buttons. Bit 6 (`0x40`) is
/// reserved for the CTGP pause flag and is ignored here. If the upper nibble
/// has any bits set (excluding bit 6) a [`FaceButton::Unknown`] entry is
/// appended to signal unrecognized input.
///
/// # Errors
///
/// Returns [`FaceButtonError::NonExistentFaceButton`] if the byte is non-zero,
/// bit 6 is not set, and no known button bits were matched.
pub fn parse_face_buttons(value: u8) -> Result<Vec<FaceButton>, FaceButtonError> {
    let mut buttons = Vec::new();

    if value & 0x01 != 0 {
        buttons.push(FaceButton::Accelerator);
    }

    if value & 0x02 != 0 {
        buttons.push(FaceButton::Brake);
    }

    if value & 0x08 != 0 {
        buttons.push(FaceButton::Drift);
    }

    if value & 0x10 != 0 {
        buttons.push(FaceButton::BrakeDrift);
    }

    if value & 0x04 != 0 {
        buttons.push(FaceButton::Item);
    }

    // 0x40 is the CTGP pause mask and would trigger this otherwise
    if value & 0xF0 != 0 && value & 0x40 == 0 {
        buttons.push(FaceButton::Unknown);
    }

    if value != 0x00 && value & 0x40 == 0 && buttons.is_empty() {
        return Err(FaceButtonError::NonExistentFaceButton);
    }

    Ok(buttons)
}

/// Errors that can occur while parsing a [`FaceInput`].
#[derive(thiserror::Error, Debug)]
pub enum FaceInputError {
    /// The raw bytes did not form a valid face input entry.
    #[error("Invalid Face Input")]
    InvalidFaceInput,
    /// The face button byte could not be parsed.
    #[error("Invalid Face Button: {0}")]
    InvalidButton(#[from] FaceButtonError),
}

/// A single encoded face-button input entry from a Mario Kart Wii ghost file.
///
/// Each entry records which face buttons were held and for how many consecutive
/// frames they were held. Unlike D-pad inputs, the frame duration is stored as
/// a single byte and does not support multi-byte encoding.
#[derive(Debug)]
pub struct FaceInput {
    /// The set of face buttons active during this input entry.
    buttons: Vec<FaceButton>,
    /// The number of frames this button state was held.
    frame_duration: u32,
}

impl FaceInput {
    /// Returns the set of face buttons active during this input entry.
    pub fn buttons(&self) -> &Vec<FaceButton> {
        &self.buttons
    }

    /// Returns the number of frames this button state was held.
    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    /// Sets the number of frames this button state was held.
    pub fn set_frame_duration(&mut self, frame_duration: u32) {
        self.frame_duration = frame_duration;
    }
}

/// Two [`FaceInput`] values are equal if they share the same set of active
/// buttons, regardless of frame duration.
impl PartialEq for FaceInput {
    fn eq(&self, other: &Self) -> bool {
        self.buttons == other.buttons
    }
}

/// Parses a [`FaceInput`] from a 2-byte slice.
///
/// # Errors
///
/// Returns [`FaceInputError::InvalidButton`] if the button byte does not map
/// to any known face button combination.
impl TryFrom<&[u8]> for FaceInput {
    type Error = FaceInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buttons = parse_face_buttons(value[0])?;
        let frame_duration = value[1] as u32;

        Ok(Self {
            buttons,
            frame_duration,
        })
    }
}
