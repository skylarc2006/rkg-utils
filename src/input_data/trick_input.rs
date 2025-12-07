#[derive(thiserror::Error, Debug)]
pub enum TrickButtonError {
    #[error("Non Existent Trick Button")]
    NonExistentTrickButton,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrickButton {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_trick_button(value: u8) -> Result<TrickButton, TrickButtonError> {
    let trick = (value & 0x70) >> 4;

    match trick {
        0 => Ok(TrickButton::None),
        1 => Ok(TrickButton::Up),
        2 => Ok(TrickButton::Down),
        3 => Ok(TrickButton::Left),
        4 => Ok(TrickButton::Right),
        _ => Err(TrickButtonError::NonExistentTrickButton),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TrickInputError {
    #[error("Invalid Trick Input")]
    InvalidTrickInput,
    #[error("Invalid Trick Button: {0}")]
    InvalidButton(#[from] TrickButtonError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Debug)]
pub struct TrickInput {
    button: TrickButton,
    frame_duration: u32,
}

impl TrickInput {
    pub fn button(&self) -> TrickButton {
        self.button
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }
}

impl TryFrom<u16> for TrickInput {
    type Error = TrickInputError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let bytes = value.to_be_bytes();
        let button = parse_trick_button(bytes[0])?;
        // 0x0F bit mask used if this input state was held longer than 255 frames, gives amount of 256-frame intervals to add to frame duration
        let previous_full_byte_presses: u32 = (bytes[0] & 0x0F).into();
        let frame_duration = bytes[1] as u32 + (previous_full_byte_presses * 256);

        Ok(Self {
            button,
            frame_duration,
        })
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for TrickInput {
    type Error = TrickInputError;

    fn try_from(value: &mut bitreader::BitReader<'_>) -> Result<Self, Self::Error> {
        TrickInput::try_from(value.read_u16(16)?)
    }
}
