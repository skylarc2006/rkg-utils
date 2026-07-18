pub mod ctgp_footer;
pub mod sp_footer;

#[doc(inline)]
pub use ctgp_footer::CTGPFooter;
#[doc(inline)]
pub use sp_footer::SPFooter;

/// Footer attached to an RKG file after the input data.
pub enum FooterType {
    /// A CTGP-R footer.
    CTGPFooter(CTGPFooter),
    /// An MKW-SP footer.
    SPFooter(SPFooter),
    /// Raw bytes from a footer whose format is not recognized.
    Unknown(Vec<u8>),
}

impl FooterType {
    /// Returns `true` if this is a [`FooterType::CTGPFooter`].
    pub fn is_ctgp(&self) -> bool {
        matches!(self, Self::CTGPFooter(_))
    }
    /// Returns `true` if this is a [`FooterType::SPFooter`].
    pub fn is_sp(&self) -> bool {
        matches!(self, Self::SPFooter(_))
    }
    /// Returns `true` if this is a [`FooterType::Unknown`].
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
