use crate::input_data::{dpad_button::DPadButton, drift_flag::DriftFlag, stick_input::StickInput};

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
    /// Whether the accelerator button is held.
    accelerator: bool,
    /// Whether the brake/reverse button is held.
    brake: bool,
    /// Whether brake-drift (hopping while braking) is active.
    brake_drift: bool,
    /// The drift flag state; see [`DriftFlag`].
    drift_flag: DriftFlag,
    /// Whether the item button is held.
    item: bool,
    /// Whether the pause button is held.
    pause: bool,
    /// Unknown face input; mask 0xF0 in a button byte. If creating a new `ControllerInput`, this can be essentially ignored and set to false.
    unknown_face_button: bool,
    /// The current D-Pad input.
    dpad: DPadButton,
    /// The current analog stick input.
    stick: StickInput,
    /// The number of frames this input state persists for before the next one.
    frame_duration: u32,
}

impl ControllerInput {
    /// Creates a new `ControllerInput`. `unknown_face_button` can be safely set to false.
    /// `frame_duration` is the number of frames this input state persists for.
    pub fn new(
        accelerator: bool,
        brake: bool,
        brake_drift: bool,
        drift_flag: DriftFlag,
        item: bool,
        pause: bool,
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
            pause,
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
            && self.brake_drift() == other.brake_drift()
            && self.drift_flag() == other.drift_flag()
            && self.item() == other.item()
            && self.pause() == other.pause()
            && self.unknown_face_button() == other.unknown_face_button()
    }

    /// Returns whether the accelerator button is held.
    pub fn accelerator(&self) -> bool {
        self.accelerator
    }

    /// Sets whether the accelerator button is held.
    pub fn set_accelerator(&mut self, value: bool) {
        self.accelerator = value;
    }

    /// Returns whether the brake/reverse button is held.
    pub fn brake(&self) -> bool {
        self.brake
    }

    /// Sets whether the brake/reverse button is held.
    pub fn set_brake(&mut self, value: bool) {
        self.brake = value;
    }

    /// Returns whether brake-drift (hopping while braking) is active.
    pub fn brake_drift(&self) -> bool {
        self.brake_drift
    }

    /// Sets whether brake-drift (hopping while braking) is active.
    pub fn set_brake_drift(&mut self, value: bool) {
        self.brake_drift = value;
    }

    /// Returns the drift flag state; see [`DriftFlag`].
    pub fn drift_flag(&self) -> DriftFlag {
        self.drift_flag
    }

    /// Sets the drift flag state; see [`DriftFlag`].
    pub fn set_drift_flag(&mut self, value: DriftFlag) {
        self.drift_flag = value;
    }

    /// Returns whether the item button is held.
    pub fn item(&self) -> bool {
        self.item
    }

    /// Sets whether the item button is held.
    pub fn set_item(&mut self, value: bool) {
        self.item = value;
    }

    /// Returns whether the pause button is held.
    pub fn pause(&self) -> bool {
        self.pause
    }

    /// Sets whether the pause button is held.
    pub fn set_pause(&mut self, value: bool) {
        self.pause = value;
    }

    /// Unknown face input; mask 0xF0 in a button byte.
    pub fn unknown_face_button(&self) -> bool {
        self.unknown_face_button
    }

    /// Sets the unknown face input; mask 0xF0 in a button byte.
    pub fn set_unknown_face_button(&mut self, value: bool) {
        self.unknown_face_button = value;
    }

    /// Returns the current D-Pad input.
    pub fn dpad(&self) -> DPadButton {
        self.dpad
    }

    /// Sets the current D-Pad input.
    pub fn set_dpad(&mut self, value: DPadButton) {
        self.dpad = value;
    }

    /// Returns the current analog stick input.
    pub fn stick(&self) -> StickInput {
        self.stick
    }

    /// Sets the current analog stick input.
    pub fn set_stick(&mut self, value: StickInput) {
        self.stick = value;
    }

    /// Returns the number of frames this input state persists for before the next one.
    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    /// Sets the number of frames this input state persists for before the next one.
    pub fn set_frame_duration(&mut self, value: u32) {
        self.frame_duration = value;
    }
}

impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            accelerator: Default::default(),
            brake: Default::default(),
            brake_drift: Default::default(),
            drift_flag: DriftFlag::AutoDetect,
            item: Default::default(),
            pause: Default::default(),
            unknown_face_button: Default::default(),
            dpad: Default::default(),
            stick: Default::default(),
            frame_duration: 1,
        }
    }
}
