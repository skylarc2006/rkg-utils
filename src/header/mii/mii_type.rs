use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum MiiTypeError {
    #[error("Invalid Mii type")]
    MiiTypeInvalid,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MiiType {
    Normal,
    Foreign,
    Special,
}

impl TryFrom<u8> for MiiType {
    type Error = MiiTypeError;

    /// Expects first 3 bits of Mii ID (first byte of Mii ID shifted right by 5)
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0x06 {
            Ok(MiiType::Foreign)
        } else if (value & 0x05) == 0 {
            Ok(MiiType::Special)
        } else if value <= 0x07 {
            Ok(MiiType::Normal)
        } else {
            Err(MiiTypeError::MiiTypeInvalid)
        }
    }
}

impl From<MiiType> for u8 {
    fn from(value: MiiType) -> Self {
        match value {
            MiiType::Normal => 0x04,
            MiiType::Foreign => 0x06,
            MiiType::Special => 0x02,
        }
    }
}

impl Display for MiiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiiType::Normal => write!(f, "Normal"),
            MiiType::Foreign => write!(f, "Foreign"),
            MiiType::Special => write!(f, "Special"),
        }
    }
}
