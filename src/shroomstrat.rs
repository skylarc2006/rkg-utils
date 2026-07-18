use std::fmt::Display;

/// Errors that can occur while constructing a [`Shroomstrat`].
#[derive(thiserror::Error, Debug)]
pub enum ShroomstratError {
    /// Shroom usages cannot be after the last lap.
    #[error("Shroom usage cannot be after the last lap")]
    ShroomUsageInvalid,
    /// Lap count cannot be 0.
    #[error("Lap count cannot be 0")]
    LapCountInvalid,
}

/// Records which lap (if any) each of a ghost's three mushrooms was used on.
///
/// "Shroomstrat" is Mario Kart Wii community terminology for the lap-by-lap
/// distribution of mushroom usage during a race.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shroomstrat {
    /// The lap that the first mushroom was used on.
    /// `None` if the first mushroom was never used.
    shroom_1_usage: Option<u8>,
    /// The lap that the second mushroom was used on.
    /// `None` if the second mushroom was never used.
    shroom_2_usage: Option<u8>,
    /// The lap that the third mushroom was used on.
    /// `None` if the third mushroom was never used.
    shroom_3_usage: Option<u8>,
    /// The total lap count associated with the run.
    lap_count: u8,
}

impl Shroomstrat {
    /// Creates a new [`Shroomstrat`].
    /// Values of `0` for shroom usages are treated as that shroom being unused.
    ///
    /// # Errors
    ///
    /// Returns [`ShroomstratError::LapCountInvalid`] if `lap_count` is 0, or
    /// [`ShroomstratError::ShroomUsageInvalid`] if any shroom usage is past `lap_count`.
    pub fn new(
        shroom_1_usage: u8,
        shroom_2_usage: u8,
        shroom_3_usage: u8,
        lap_count: u8,
    ) -> Result<Self, ShroomstratError> {
        if lap_count == 0 {
            Err(ShroomstratError::LapCountInvalid)
        } else if shroom_1_usage > lap_count
            || shroom_2_usage > lap_count
            || shroom_3_usage > lap_count
        {
            Err(ShroomstratError::ShroomUsageInvalid)
        } else {
            Ok(Self {
                shroom_1_usage: if shroom_1_usage == 0 {
                    None
                } else {
                    Some(shroom_1_usage)
                },
                shroom_2_usage: if shroom_2_usage == 0 {
                    None
                } else {
                    Some(shroom_2_usage)
                },
                shroom_3_usage: if shroom_3_usage == 0 {
                    None
                } else {
                    Some(shroom_3_usage)
                },
                lap_count,
            })
        }
    }

    /// Returns the lap the first mushroom was used on, or `None` if it was never used.
    pub fn shroom_1_usage(&self) -> Option<u8> {
        self.shroom_1_usage
    }

    /// Returns the lap the second mushroom was used on, or `None` if it was never used.
    pub fn shroom_2_usage(&self) -> Option<u8> {
        self.shroom_2_usage
    }

    /// Returns the lap the third mushroom was used on, or `None` if it was never used.
    pub fn shroom_3_usage(&self) -> Option<u8> {
        self.shroom_3_usage
    }

    /// Returns the three raw footer bytes `[shroom_1_usage, shroom_2_usage, shroom_3_usage]`.
    /// A value of 0 means the mushroom was never used.
    pub fn to_raw_bytes(&self) -> [u8; 3] {
        [
            self.shroom_1_usage.unwrap_or(0),
            self.shroom_2_usage.unwrap_or(0),
            self.shroom_3_usage.unwrap_or(0),
        ]
    }

    /// Returns a per-lap mushroom-usage-count vector of length `lap_count`, where
    /// index `n` holds the number of mushrooms used on lap `n + 1`.
    ///
    /// For example, `[1, 0, 2]` means one mushroom on lap 1, none on lap 2, and two
    /// on lap 3.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut shroomstrat = vec![0; self.lap_count.into()];

        for shroom in [
            self.shroom_1_usage,
            self.shroom_2_usage,
            self.shroom_3_usage,
        ]
        .iter()
        .flatten()
        {
            shroomstrat[(*shroom - 1) as usize] += 1;
        }

        shroomstrat
    }
}

/// Formats as `-`-separated per-lap mushroom usage counts, e.g. `"1-0-2"`
/// (see [`Shroomstrat::to_vec`]).
impl Display for Shroomstrat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_vec();
        let mut string = v[0].to_string();

        if v.len() > 1 {
            for lap in v[1..].iter() {
                string += format!("-{}", lap).as_str();
            }
        }
        write!(f, "{string}")
    }
}
