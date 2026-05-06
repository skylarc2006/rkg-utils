use crate::input_data::{dpad_button::DPadButton, stick_input::StickInput, drift_flag::DriftFlag};

/// Represents the errors that can go wrong while parsing a `ControllerInput`.
#[derive(thiserror::Error, Debug)]
pub enum ControllerInputError {
    /// Frame duration cannot be 0.
    #[error("Invalid frame duration (cannot be 0).")]
    FrameDurationTooShort,
}

/// Represents a controller input state.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ControllerInput {
    accelerator: bool,
    brake: bool,
    brake_drift: bool,
    drift_flag: DriftFlag,
    item: bool,
    /// Unknown face input; mask 0xF0 in a button byte. If creating a new `ControllerInput`, this can be essentially ignored and set to false.
    unknown_face_button: bool,
    dpad: DPadButton,
    stick: StickInput,
    frame_duration: u32,
}

impl ControllerInput {
    /// Creates a new `ControllerInput`. `unknown_face_button` can be safely set to false.
    pub fn new(
        accelerator: bool,
        brake: bool,
        brake_drift: bool,
        drift_flag: DriftFlag,
        item: bool,
        unknown_face_button: bool,
        dpad: DPadButton,
        stick: StickInput,
        frame_duration: u32,
    ) -> Self {
        ControllerInput {
            accelerator,
            brake,
            brake_drift,
            drift_flag,
            item,
            unknown_face_button,
            dpad,
            stick,
            frame_duration,
        }
    }

    /// Returns true if both controller inputs have identical face button configurations.
    pub fn face_buttons_equal_to(&self, other: ControllerInput) -> bool {
        self.accelerator() == other.accelerator()
            && self.brake() == other.brake()
            && self.drift_flag() == other.drift_flag()
            && self.item() == other.item()
            && self.unknown_face_button() == other.unknown_face_button()
    }

    pub fn accelerator(&self) -> bool {
        self.accelerator
    }

    pub fn set_accelerator(&mut self, value: bool) {
        self.accelerator = value;
    }

    pub fn brake(&self) -> bool {
        self.brake
    }

    pub fn set_brake(&mut self, value: bool) {
        self.brake = value;
    }

    pub fn brake_drift(&self) -> bool {
        self.brake_drift
    }

    pub fn set_brake_drift(&mut self, value: bool) {
        self.brake_drift = value;
    }

    pub fn drift_flag(&self) -> DriftFlag {
        self.drift_flag
    }

    pub fn set_drift_flag(&mut self, value: DriftFlag) {
        self.drift_flag = value;
    }

    pub fn item(&self) -> bool {
        self.item
    }

    pub fn set_item(&mut self, value: bool) {
        self.item = value;
    }

    /// Unknown face input; mask 0xF0 in a button byte.
    pub fn unknown_face_button(&self) -> bool {
        self.unknown_face_button
    }

    pub fn set_unknown_face_button(&mut self, value: bool) {
        self.unknown_face_button = value;
    }

    pub fn dpad(&self) -> DPadButton {
        self.dpad
    }

    pub fn set_dpad(&mut self, value: DPadButton) {
        self.dpad = value;
    }

    pub fn stick(&self) -> StickInput {
        self.stick
    }

    pub fn set_stick(&mut self, value: StickInput) {
        self.stick = value;
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, value: u32) {
        self.frame_duration = value;
    }
}
