/// Errors that can occur while parsing a [`FaceButton`] combination.
#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    /// The face button byte was non-zero but did not match any known button bit.
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FaceButton {
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

pub(crate) struct FaceButtons(pub Vec<FaceButton>);
impl TryFrom<u8> for FaceButtons {
    type Error = FaceButtonError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

        Ok(FaceButtons(buttons))
    }
}
