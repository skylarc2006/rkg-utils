/// Errors that can occur while parsing a [`DPadButton`].
#[derive(thiserror::Error, Debug)]
pub enum DPadButtonError {
    /// The extracted button value did not correspond to any known D-pad direction.
    #[error("Non Existent DPad Button")]
    NonExistentDPadButton,
}

/// Represents every possible DPad input.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum DPadButton {
    #[default]
    None,
    Up,
    Left,
    Right,
    Down,
    /// Unknown DPad input; mask 0x80 in an input byte; likely unused.
    Unknown,
}

impl DPadButton {
    pub(crate) fn to_nibble(self) -> u8 {
        match self {
            DPadButton::None => 0,
            DPadButton::Up => 1,
            DPadButton::Down => 2,
            DPadButton::Left => 3,
            DPadButton::Right => 4,
            DPadButton::Unknown => 8,
        }
    }
}

impl TryFrom<u8> for DPadButton {
    type Error = DPadButtonError;

    /// Expects full DPad button byte
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & 0x80 != 0 {
            return Ok(DPadButton::Unknown);
        }

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
}
