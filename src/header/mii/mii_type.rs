use std::fmt::Display;

/// Errors that can occur while constructing a [`MiiType`].
#[derive(thiserror::Error, Debug)]
pub enum MiiTypeError {
    /// The upper 3 bits of the Mii ID do not correspond to any known [`MiiType`].
    #[error("Invalid Mii type")]
    MiiTypeInvalid,
}

/// Represents the origin/ownership type of a Mii.
///
/// The type is encoded in the upper 3 bits of the first byte of the Mii ID.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MiiType {
    /// A Mii created and owned by the local console.
    Normal,
    /// A Mii received from another console or player.
    Foreign,
    /// A built-in special Mii (e.g. Nintendo staff Miis).
    Special,
}

/// Converts the upper 3 bits of the first byte of a Mii ID into a [`MiiType`].
///
/// The `value` parameter should be the first byte of the Mii ID shifted right by 5,
/// yielding a 3-bit value. `0x06` maps to [`MiiType::Foreign`]; values where bits
/// 0 and 2 are both clear map to [`MiiType::Special`]; all other values in the range
/// 0–7 map to [`MiiType::Normal`].
///
/// # Errors
///
/// Returns [`MiiTypeError::MiiTypeInvalid`] if `value` exceeds 7.
impl TryFrom<u8> for MiiType {
    type Error = MiiTypeError;

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

/// Converts a [`MiiType`] into its canonical upper-3-bit representation
/// for use in the first byte of a Mii ID.
impl From<MiiType> for u8 {
    fn from(value: MiiType) -> Self {
        match value {
            MiiType::Normal => 0x04,
            MiiType::Foreign => 0x06,
            MiiType::Special => 0x02,
        }
    }
}

/// Formats the [`MiiType`] as a human-readable string.
impl Display for MiiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiiType::Normal => write!(f, "Normal"),
            MiiType::Foreign => write!(f, "Foreign"),
            MiiType::Special => write!(f, "Special"),
        }
    }
}
