use crate::header::controller::Controller;

/// Errors that can occur while parsing a [`StickInput`].
#[derive(thiserror::Error, Debug)]
pub enum StickInputError {
    /// One or both raw axis values exceeded the maximum encoded value of 14.
    #[error("Invalid Stick Input (must be range 0-14).")]
    InvalidStickInput,
}

/// A single encoded analog stick input entry from a Mario Kart Wii ghost file.
///
/// Both axes are encoded in a single byte as 4-bit unsigned values (0–14).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StickInput {
    x: u8,
    y: u8,
}

impl StickInput {
    pub fn new(x: u8, y: u8) -> Result<Self, StickInputError> {
        if x > 14 || y > 14 {
            Err(StickInputError::InvalidStickInput)
        } else {
            Ok(Self { x, y })
        }
    }

    /// Whether the stick input is impossible based on the `Controller` used.
    pub fn is_impossible(&self, controller: Controller) -> bool {
        const ILLEGAL_STICK_INPUTS: [[u8; 2]; 44] = [
            // These inputs are illegal for GCN, CCP, and Nunchuk (24 total)
            [0, 14],
            [0, 13],
            [0, 12],
            [0, 0],
            [0, 1],
            [0, 2],
            [1, 14],
            [1, 13],
            [1, 0],
            [1, 1],
            [2, 14],
            [2, 0],
            [14, 14],
            [14, 13],
            [14, 12],
            [14, 0],
            [14, 1],
            [14, 2],
            [13, 14],
            [13, 13],
            [13, 0],
            [13, 1],
            [12, 14],
            [12, 0],
            // Illegal stick inputs for specifically GCN/CCP (additional 20)
            [0, 11],
            [1, 12],
            [2, 13],
            [3, 14],
            [4, 14],
            [10, 14],
            [11, 14],
            [11, 13],
            [11, 0],
            [12, 13],
            [12, 12],
            [12, 1],
            [13, 12],
            [13, 11],
            [13, 2],
            [14, 11],
            [14, 10],
            [14, 9],
            [14, 4],
            [14, 3],
        ];

        let illegal_stick_inputs = match controller {
            Controller::Nunchuk => &ILLEGAL_STICK_INPUTS[..24],
            Controller::Classic | Controller::Gamecube => &ILLEGAL_STICK_INPUTS,
            Controller::WiiWheel => {
                return false;
            }
        };

        for illegal_stick_input in illegal_stick_inputs.iter() {
            if &[self.x, self.y] == illegal_stick_input {
                return true;
            }
        }

        false



    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn set_x(&mut self, x: u8) -> Result<(), StickInputError> {
        if x > 14 {
            Err(StickInputError::InvalidStickInput)
        } else {
            self.x = x;
            Ok(())
        }
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), StickInputError> {
        if y > 14 {
            Err(StickInputError::InvalidStickInput)
        } else {
            self.y = y;
            Ok(())
        }
    }
}

impl TryFrom<u8> for StickInput {
    type Error = StickInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let x = (value & 0xF0) >> 4;
        let y = value & 0x0F;

        Self::new(x, y)
    }
}
