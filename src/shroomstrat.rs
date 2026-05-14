use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum ShroomstratError {
    /// Shroom usages cannot be after the last lap.
    #[error("Shroom usage cannot be after the last lap")]
    ShroomUsageInvalid,
    /// Lap count cannot be 0.
    #[error("Lap count cannot be 0")]
    LapCountInvalid,
}

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
    /// Returns [`ShroomstratError`] if the lap count is 0 or any shroom usages happen past the
    /// specified lap count.
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

    pub fn to_vec(&self) -> Vec<u8> {
        let mut shroomstrat = Vec::new();
        shroomstrat.resize(self.lap_count.into(), 0u8);

        for shroom in [
            self.shroom_1_usage,
            self.shroom_2_usage,
            self.shroom_3_usage,
        ]
        .iter()
        {
            if let Some(shroom) = shroom {
                shroomstrat[(*shroom - 1) as usize] += 1;
            }
        }

        shroomstrat
    }
}

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
