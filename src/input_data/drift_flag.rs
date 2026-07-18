/// Represents possible options of the drift flag input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriftFlag {
    /// Drift flag is force-enabled for a [`ControllerInput`](crate::input_data::ControllerInput).
    Enabled,
    /// Drift flag is force-disabled for a [`ControllerInput`](crate::input_data::ControllerInput).
    Disabled,
    /// Drift flag is automatically set based off of surrounding [`ControllerInput`](crate::input_data::ControllerInput)s
    /// in an array when writing input data to a file.
    AutoDetect,
}
