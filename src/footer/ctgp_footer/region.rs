/// A disc region, as encoded by the game's region byte in the CTGP footer.
#[derive(Clone, Copy, Debug)]
pub enum Region {
    /// NTSC-U (North America); region byte `b'E'`.
    NtscU,
    /// PAL (Europe/Australia); region byte `b'P'`.
    Pal,
    /// NTSC-J (Japan); region byte `b'J'`.
    NtscJ,
    /// The region byte did not match any known disc region.
    Unknown,
}

impl From<u8> for Region {
    /// Converts a raw region byte into a [`Region`]. Any unrecognized byte
    /// becomes [`Region::Unknown`].
    fn from(value: u8) -> Self {
        match value {
            b'E' => Self::NtscU,
            b'P' => Self::Pal,
            b'J' => Self::NtscJ,
            _ => Self::Unknown,
        }
    }
}

impl From<Region> for u8 {
    /// Converts a [`Region`] back into its raw region byte. [`Region::Unknown`]
    /// becomes `0`.
    fn from(value: Region) -> Self {
        match value {
            Region::NtscU => b'E',
            Region::Pal => b'P',
            Region::NtscJ => b'J',
            Region::Unknown => 0,
        }
    }
}
