use std::fmt::Display;

/// Whether a vehicle/combo uses inside or outside drift.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transmission {
    Inside,
    Outside,
}

impl Display for Transmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Transmission::Inside => "Inside Drift",
                Transmission::Outside => "Outside Drift",
            }
        )
    }
}
