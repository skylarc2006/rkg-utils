use std::{convert::Infallible, fmt::Display};

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum TransmissionModError {
    #[error("Invalid transmission mod ID")]
    InvalidTransmissionMod,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Represents the different transmission mods present in Retro Rewind (Pulsar) ghost data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransmissionMod {
    Vanilla,
    AllInside,
    AllBikeInside,
    AllOutside,
}

impl TryFrom<u8> for TransmissionMod {
    type Error = TransmissionModError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Vanilla),
            0x01 => Ok(Self::AllInside),
            0x02 => Ok(Self::AllBikeInside),
            0x03 => Ok(Self::AllOutside),
            _ => Err(TransmissionModError::InvalidTransmissionMod),
        }
    }
}

impl From<TransmissionMod> for u8 {
    fn from(value: TransmissionMod) -> Self {
        match value {
            TransmissionMod::Vanilla => 0x00,
            TransmissionMod::AllInside => 0x01,
            TransmissionMod::AllBikeInside => 0x02,
            TransmissionMod::AllOutside => 0x03,
        }
    }
}

impl FromByteHandler for TransmissionMod {
    type Err = TransmissionModError;
    /// Expects Header 0x0C
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        ((handler.try_into()?.copy_byte(0) >> 1) & 0x03).try_into()
    }
}



impl Display for TransmissionMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanilla => write!(f, "Vanilla"),
            Self::AllInside => write!(f, "All Inside"),
            Self::AllBikeInside => write!(f, "All Bikes Inside"),
            Self::AllOutside => write!(f, "All Outside"),
        }
    }
}
