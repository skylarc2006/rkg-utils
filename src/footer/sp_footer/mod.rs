// TODO: eliminate the need for raw_data to be a struct member
use crate::{
    byte_handler::{ByteHandler, FromByteHandler},
    footer::{ctgp_footer::exact_in_game_time::ExactInGameTime, sp_footer::sp_version::SPVersion},
    header::in_game_time::{InGameTime, InGameTimeError},
    shroomstrat::Shroomstrat,
    write_bits,
};

pub mod sp_version;

/// Errors that can occur while parsing an [`SPFooter`].
#[derive(thiserror::Error, Debug)]
pub enum SPFooterError {
    /// The ghost file does not contain the expected `SPGD` magic bytes.
    #[error("Ghost is not SPGD")]
    NotSPGD,
    /// The file is not an RKG file.
    #[error("File is not an RKG")]
    NotRKGD,
    /// Data passed is impossibly too short.
    #[error("Data length is too short")]
    DataLengthTooShort,
    /// The footer version number exceeds the maximum supported value (5).
    #[error("Invalid MKW-SP footer version")]
    InvalidFooterVersion,
    /// An in-game time field could not be parsed.
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    /// Shroomstrat parsing failed.
    #[error("Shroomstrat Error: {0}")]
    ShroomstratError(#[from] crate::shroomstrat::ShroomstratError),
    /// A lap split index was out of range for the recorded lap count.
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    /// A slice-to-array conversion failed while reading footer data.
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

/// Parsed representation of the MKW-SP footer appended to Mario Kart Wii ghost files.
///
/// The footer stores metadata written by MKW Service Pack at the end of each recorded ghost,
/// including high-precision timing, version information, track hash, mushroom strategy,
/// and various gameplay flags. Some fields are only present in later footer versions.
pub struct SPFooter {
    /// The footer format version, which determines available fields and layout.
    footer_version: u32,
    /// One or more MKW-SP release versions consistent with the footer version number.
    /// `None` if the footer version is unrecognised.
    possible_sp_versions: Option<Vec<SPVersion>>,
    /// SHA-1 hash of the track file associated with this ghost.
    track_sha1: [u8; 0x14],
    /// Sub-millisecond-accurate finish time, computed as the sum of all exact lap times.
    exact_finish_time: ExactInGameTime,
    /// Whether the exact finish time is unreliable.
    exact_finish_time_unreliable: bool,
    /// Sub-millisecond-accurate lap times, one per recorded lap (up to 11).
    exact_lap_times: Vec<ExactInGameTime>,
    /// Whether the exact lap times are unreliable.
    exact_lap_times_unreliable: bool,
    /// Raw data representations of the exact lap time differences.
    lap_true_time_difference_data: Vec<[u8; 4]>,
    /// Whether the run was in 200cc.
    is_200cc: bool,
    /// Whether an ultra shortcut was performed during the run.
    has_ultra_shortcut: bool,
    /// Whether a horizontal wall glitch was performed during the run.
    has_horizontal_wall_glitch: bool,
    /// Whether a wallride was performed during the run.
    has_wallride: bool,
    /// Per-lap mushroom usage counts. `None` for footer version 0, which lacks this data.
    shroomstrat: Option<Shroomstrat>,
    /// Whether vanilla mode was enabled during the run. `None` for footer versions below 3.
    is_vanilla_mode_enabled: Option<bool>,
    /// Whether simplified controls were enabled during the run. `None` for footer versions below 4.
    has_simplified_controls: Option<bool>,
    /// Whether the run was set in mirror mode. `None` for footer versions below 5.
    set_in_mirror: Option<bool>,
    /// Length of the footer in bytes, excluding the trailing CRC32.
    len: u32,
    /// Number of laps recorded in the ghost.
    lap_count: u8,
}

impl SPFooter {
    /// Parses an [`SPFooter`] from a complete RKG ghost file byte slice.
    ///
    /// Validates the `SPGD` magic and footer version, then reads all footer fields
    /// including high-precision lap times, track SHA-1, shroomstrat, and gameplay flags.
    /// Fields introduced in later footer versions are stored as `Option`, and will be
    /// `None` when the footer version does not include them.
    ///
    /// # Arguments
    ///
    /// * `data` - The full raw bytes of the RKG ghost file, including the MKW-SP footer.
    ///
    /// # Errors
    ///
    /// Returns an [`SPFooterError`] if:
    /// - The `SPGD` magic bytes are absent ([`SPFooterError::NotSPGD`]).
    /// - The footer version exceeds 5 ([`SPFooterError::InvalidFooterVersion`]).
    /// - Any byte slice conversion, integer parse, or time parse fails.
    pub fn new(data: &[u8]) -> Result<Self, SPFooterError> {
        if data.len() < 0x0C {
            return Err(SPFooterError::DataLengthTooShort);
        }

        if data[..0x04] != *b"RKGD" {
            return Err(SPFooterError::NotRKGD);
        }

        if data[data.len() - 0x08..data.len() - 0x04] != *b"SPGD" {
            return Err(SPFooterError::NotSPGD);
        }

        let footer_len = (u32::from_be_bytes(
            data[data.len() - 0x0C..data.len() - 0x08]
                .try_into()
                .unwrap(),
        ) + 0x08) as usize;

        if data.len() < footer_len {
            return Err(SPFooterError::DataLengthTooShort);
        }

        let footer_data = &data[data.len() - footer_len - 0x04..data.len() - 0x04];

        let footer_version = u32::from_be_bytes(footer_data[..0x04].try_into().unwrap());

        if footer_version > 5 {
            return Err(SPFooterError::InvalidFooterVersion);
        }

        let possible_sp_versions = SPVersion::from(footer_version);

        let mut current_offset = 0x04;

        let track_sha1 = footer_data[current_offset..current_offset + 0x14]
            .to_owned()
            .try_into()
            .unwrap();
        current_offset += 0x14;

        // Exact lap split calculation
        let lap_count = data[0x10] as usize;

        let lap_time_data = &data[0x11..0x11 + (3 * lap_count)];
        let mut lap_times = Vec::new();

        for lap_time in lap_time_data.chunks_exact(3) {
            lap_times.push(InGameTime::from_byte_handler(lap_time)?);
        }

        let lap_true_time_differences =
            &footer_data[current_offset..current_offset + lap_count * 0x04];
        let mut lap_true_time_difference_data = Vec::new();
        lap_true_time_difference_data.resize(lap_count, [0u8; 4]);

        for (idx, difference) in lap_true_time_differences.chunks_exact(4).enumerate() {
            lap_true_time_difference_data[idx] = difference.try_into().unwrap();
        }

        let mut exact_lap_times = Vec::new();
        exact_lap_times.resize(lap_count, ExactInGameTime::default());

        let mut previous_subtractions = 0i64;
        let mut subtraction_ps = 0i64;
        let mut exact_lap_times_unreliable = false;
        let exact_finish_time_unreliable;

        for lap_true_time_difference in lap_true_time_difference_data.iter() {
            if f32::from_be_bytes(*lap_true_time_difference) < -1.0 {
                exact_lap_times_unreliable = true;
            }
        }

        for (idx, exact_lap_time) in exact_lap_times.iter_mut().enumerate() {
            let lap_true_time_difference = if exact_lap_times_unreliable {
                0f32
            } else {
                f32::from_be_bytes(lap_true_time_difference_data[idx])
            };

            let mut true_time_ps_subtraction =
                (lap_true_time_difference as f64 * 1e+9).floor() as i64;

            // subtract the sum of the previous laps' difference because the lap differences add up to
            // have its decimal portion be equal to the total time
            true_time_ps_subtraction -= previous_subtractions;

            if true_time_ps_subtraction > 1e+9 as i64 {
                true_time_ps_subtraction -= subtraction_ps;
                subtraction_ps = if subtraction_ps == 0 { 1e+9 as i64 } else { 0 };
            }
            previous_subtractions += true_time_ps_subtraction;

            let true_ps =
                lap_times[idx].to_milliseconds() as i64 * 1e+9 as i64 + true_time_ps_subtraction;
            let true_sec = (true_ps / 1e+12 as i64) as u8;
            let true_fractional_sec = true_ps % 1e+12 as i64;

            *exact_lap_time = if true_ps > 0 {
                ExactInGameTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
            } else {
                ExactInGameTime::default()
            };
        }

        let mut exact_finish_time: ExactInGameTime =
            exact_lap_times[..lap_count].iter().copied().sum();

        let finish_true_time_difference = f32::from_be_bytes(
            lap_true_time_difference_data[lap_true_time_difference_data.len() - 1],
        );
        if exact_lap_times_unreliable && finish_true_time_difference > -1.0 {
            let true_time_ps_subtraction =
                (finish_true_time_difference as f64 * 1e+9).floor() as i64;
            let total_picoseconds =
                exact_finish_time.time_to_picoseconds() as i64 + true_time_ps_subtraction;

            let true_sec = (total_picoseconds / 1e+12 as i64) as u8;
            let true_fractional_sec = total_picoseconds % 1e+12 as i64;

            exact_finish_time = if total_picoseconds > 0 {
                ExactInGameTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
            } else {
                ExactInGameTime::default()
            };

            exact_finish_time_unreliable = false;
        } else {
            exact_finish_time_unreliable = exact_lap_times_unreliable;
        }

        current_offset += 0x2C;

        let bools = ByteHandler::from(footer_data[current_offset]);
        let is_200cc = bools.read_bool(7);
        let has_ultra_shortcut = bools.read_bool(6);
        let has_horizontal_wall_glitch = bools.read_bool(5);
        let has_wallride = bools.read_bool(4);

        let shroomstrat = if footer_version >= 1 {
            let shroom_data: [u8; 3] = footer_data[current_offset..current_offset + 0x03]
                .try_into()
                .unwrap();

            let raw = u32::from_be_bytes([0, shroom_data[0], shroom_data[1], shroom_data[2]]);
            let shroom_1 = ((raw >> 15) & 0x1F) as u8;
            let shroom_2 = ((raw >> 10) & 0x1F) as u8;
            let shroom_3 = ((raw >> 5) & 0x1F) as u8;

            Some(Shroomstrat::new(
                shroom_1,
                shroom_2,
                shroom_3,
                lap_count as u8,
            )?)
        } else {
            None
        };

        current_offset += 0x02;

        let bools = ByteHandler::from(footer_data[current_offset]);

        let is_vanilla_mode_enabled = if footer_version >= 3 {
            Some(bools.read_bool(4))
        } else {
            None
        };

        let has_simplified_controls = if footer_version >= 4 {
            Some(bools.read_bool(3))
        } else {
            None
        };

        let set_in_mirror = if footer_version >= 5 {
            Some(bools.read_bool(2))
        } else {
            None
        };

        Ok(Self {
            footer_version,
            possible_sp_versions,
            track_sha1,
            exact_finish_time,
            exact_finish_time_unreliable,
            exact_lap_times,
            exact_lap_times_unreliable,
            lap_true_time_difference_data,
            is_200cc,
            has_ultra_shortcut,
            has_horizontal_wall_glitch,
            has_wallride,
            shroomstrat,
            is_vanilla_mode_enabled,
            has_simplified_controls,
            set_in_mirror,
            len: footer_len as u32,
            lap_count: lap_count as u8,
        })
    }

    /// Returns the raw bytes of the footer, excluding the trailing CRC32.
    pub fn raw_data(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();

        data.extend_from_slice(&self.footer_version.to_be_bytes());
        data.extend_from_slice(self.track_sha1());
        for lap in self.lap_true_time_difference_data().iter() {
            data.extend_from_slice(lap);
        }
        for _ in 0..11 - self.lap_count {
            data.extend_from_slice(&0u32.to_be_bytes());
        }

        let [shroom_1, shroom_2, shroom_3] = self
            .shroomstrat
            .map(|shroomstrat| shroomstrat.to_raw_bytes())
            .unwrap_or([0, 0, 0]);

        let mut flags = [0u8; 4];
        write_bits(&mut flags, 0, 0, 1, self.is_200cc() as u64);
        write_bits(&mut flags, 0, 1, 1, self.has_ultra_shortcut() as u64);
        write_bits(
            &mut flags,
            0,
            2,
            1,
            self.has_horizontal_wall_glitch() as u64,
        );
        write_bits(&mut flags, 0, 3, 1, self.has_wallride() as u64);
        write_bits(&mut flags, 0, 4, 5, shroom_1 as u64);
        write_bits(&mut flags, 1, 1, 5, shroom_2 as u64);
        write_bits(&mut flags, 1, 6, 5, shroom_3 as u64);
        write_bits(
            &mut flags,
            2,
            3,
            1,
            self.is_vanilla_mode_enabled.unwrap_or(false) as u64,
        );
        write_bits(
            &mut flags,
            2,
            4,
            1,
            self.has_simplified_controls.unwrap_or(false) as u64,
        );
        write_bits(
            &mut flags,
            2,
            5,
            1,
            self.set_in_mirror.unwrap_or(false) as u64,
        );
        data.extend_from_slice(&flags);

        data.extend_from_slice(&(self.len() as u32 - 8).to_be_bytes());
        data.extend_from_slice(b"SPGD");

        data
    }

    /// Returns the footer format version number.
    pub fn footer_version(&self) -> u32 {
        self.footer_version
    }

    /// Returns the possible MKW-SP release versions consistent with this footer's version number.
    ///
    /// Returns `None` if the footer version did not match any known release.
    pub fn possible_sp_versions(&self) -> Option<&Vec<SPVersion>> {
        self.possible_sp_versions.as_ref()
    }

    /// Returns the SHA-1 hash of the track file associated with this ghost.
    pub fn track_sha1(&self) -> &[u8; 0x14] {
        &self.track_sha1
    }

    /// Returns the sub-millisecond-accurate finish time, computed as the sum of all exact lap times.
    pub fn exact_finish_time(&self) -> ExactInGameTime {
        self.exact_finish_time
    }

    /// Returns whether the exact finish time is unreliable.
    pub fn exact_finish_time_unreliable(&self) -> bool {
        self.exact_finish_time_unreliable
    }

    /// Returns the sub-millisecond-accurate lap times for all recorded laps.
    pub fn exact_lap_times(&self) -> &[ExactInGameTime] {
        let lap_count = std::cmp::min(self.exact_lap_times.len(), self.lap_count as usize);
        &self.exact_lap_times[0..lap_count]
    }

    /// Returns whether the exact lap times are unreliable.
    pub fn exact_lap_times_unreliable(&self) -> bool {
        self.exact_lap_times_unreliable
    }

    /// Returns the lap true time differences, in their raw data form.
    pub fn lap_true_time_difference_data(&self) -> &[[u8; 4]] {
        &self.lap_true_time_difference_data
    }

    /// Returns the sub-millisecond-accurate time for a single lap by index.
    ///
    /// # Errors
    ///
    /// Returns [`SPFooterError::LapSplitIndexError`] if `idx` is greater than or equal to
    /// the number of recorded laps.
    pub fn exact_lap_time(&self, idx: usize) -> Result<ExactInGameTime, SPFooterError> {
        if idx >= self.lap_count as usize || idx >= self.exact_lap_times.len() {
            return Err(SPFooterError::LapSplitIndexError);
        }
        Ok(self.exact_lap_times[idx])
    }

    /// Returns whether a speed modifier was active during the run.
    pub fn is_200cc(&self) -> bool {
        self.is_200cc
    }

    /// Returns whether an ultra shortcut was performed during the run.
    pub fn has_ultra_shortcut(&self) -> bool {
        self.has_ultra_shortcut
    }

    /// Returns whether a horizontal wall glitch was performed during the run.
    pub fn has_horizontal_wall_glitch(&self) -> bool {
        self.has_horizontal_wall_glitch
    }

    /// Returns whether a wallride was performed during the run.
    pub fn has_wallride(&self) -> bool {
        self.has_wallride
    }

    /// Returns the per-lap mushroom usage counts (shroomstrat) for the recorded laps.
    ///
    /// Returns `None` for footer version 0, which does not include shroomstrat data.
    pub fn shroomstrat(&self) -> Option<Shroomstrat> {
        self.shroomstrat
    }

    /// Returns whether vanilla mode was enabled during the run.
    ///
    /// Returns `None` for footer versions below 3, which do not include this field.
    pub fn is_vanilla_mode_enabled(&self) -> Option<bool> {
        self.is_vanilla_mode_enabled
    }

    /// Returns whether simplified controls were enabled during the run.
    ///
    /// Returns `None` for footer versions below 4, which do not include this field.
    pub fn has_simplified_controls(&self) -> Option<bool> {
        self.has_simplified_controls
    }

    /// Returns whether the run was set in mirror mode.
    ///
    /// Returns `None` for footer versions below 5, which do not include this field.
    pub fn set_in_mirror(&self) -> Option<bool> {
        self.set_in_mirror
    }

    /// Returns the length of the footer in bytes, excluding the trailing CRC32.
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the footer has zero length.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
