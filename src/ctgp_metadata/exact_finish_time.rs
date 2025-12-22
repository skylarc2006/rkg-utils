use std::fmt::Display;

#[derive(Default, Clone, Copy)]
pub struct ExactFinishTime {
    minutes: u8,
    seconds: u8,
    femtoseconds: u64,
}

impl ExactFinishTime {
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, femtoseconds: u64) -> Self {
        Self {
            minutes,
            seconds,
            femtoseconds,
        }
    }

    pub fn minutes(self) -> u8 {
        self.minutes
    }

    pub fn seconds(self) -> u8 {
        self.seconds
    }

    pub fn femtoseconds(self) -> u64 {
        self.femtoseconds
    }
}

impl Display for ExactFinishTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:015}",
            self.minutes, self.seconds, self.femtoseconds
        )
    }
}
