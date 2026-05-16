use crate::footer::{ctgp_footer::CTGPFooter, sp_footer::SPFooter};

pub mod ctgp_footer;
pub mod sp_footer;

/// Footer attached to an RKG file after the input data.
pub enum FooterType {
    CTGPFooter(CTGPFooter),
    SPFooter(SPFooter),
    /// Raw bytes from a footer whose format is not recognized.
    Unknown(Vec<u8>),
}

impl FooterType {
    pub fn is_ctgp(&self) -> bool {
        matches!(self, Self::CTGPFooter(_))
    }
    pub fn is_sp(&self) -> bool {
        matches!(self, Self::SPFooter(_))
    }
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Returns the raw bytes of the footer.
    pub fn raw_data(&self) -> Vec<u8> {
        match self {
            Self::CTGPFooter(footer) => footer.raw_data(),
            Self::SPFooter(footer) => footer.raw_data(),
            Self::Unknown(bytes) => bytes.to_owned(),
        }
    }
}
