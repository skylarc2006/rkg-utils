use crate::header::location::{country::Country, subregion::Subregion};

pub mod country;
pub mod subregion;

/// Represents the country and subregion of the player. https://docs.google.com/spreadsheets/d/1mSAomO_msfNllNsPeXbgU6UbJaGV5t6NvbZi6ebPFx4/edit?usp=sharing
pub struct Location {
    country: Country,
    subregion: Subregion,
}
