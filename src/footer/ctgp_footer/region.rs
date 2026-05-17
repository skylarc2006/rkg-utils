/// Enum representing possible game regions
#[derive(Clone, Copy, Debug)]
pub enum Region {
    NtscU,
    Pal,
    NtscJ,
    Unknown,
}

impl From<u8> for Region {
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
    fn from(value: Region) -> Self {
        match value {
            Region::NtscU => b'E',
            Region::Pal => b'P',
            Region::NtscJ => b'J',
            Region::Unknown => 0,
        }
    }
}
