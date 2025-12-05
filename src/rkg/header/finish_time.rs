use std::fmt::Display;

pub struct FinishTime {
    minutes: u8,       // 7 bits, offset 0x00
    seconds: u8,       // 7 bits, offset 0x00.7
    milliseconds: u16, // 10 bits, offset 0x01.6
}

impl FinishTime {
    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }
}

impl Display for FinishTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:03}",
            self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl From<&mut bitreader::BitReader<'_>> for FinishTime {
    fn from(value: &mut bitreader::BitReader<'_>) -> Self {
        let minutes: u8 = value
            .read_u8(7)
            .expect("Failed to read minutes of finish time");

        let seconds: u8 = value
            .read_u8(7)
            .expect("Failed to read seconds of finish time");

        let milliseconds: u16 = value
            .read_u16(10)
            .expect("Failed to read milliseconds of finish time");

        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }
}
