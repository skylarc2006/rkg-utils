/// Errors that can occur while parsing a [`DPadButton`].
#[derive(thiserror::Error, Debug)]
pub enum DPadButtonError {
    /// The extracted button value did not correspond to any known D-pad direction.
    #[error("Non Existent DPad Button")]
    NonExistentDPadButton,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DPadButton {
    None,
    Up,
    Left,
    Right,
    Down,
    /// Unknown DPad input; mask 0x80 in an input byte; likely unused.
    Unknown,
}

impl TryFrom<u8> for DPadButton {
    type Error = DPadButtonError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & 0x80 == 1 {
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
