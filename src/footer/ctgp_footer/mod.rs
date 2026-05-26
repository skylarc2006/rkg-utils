use crate::byte_handler::{ByteHandlerError, FromByteHandler};
use crate::footer::ctgp_footer::region::Region;
use crate::footer::ctgp_footer::{
    category::Category, ctgp_version::CTGPVersion, exact_in_game_time::ExactInGameTime,
};
use crate::header::in_game_time::InGameTime;
use crate::shroomstrat::Shroomstrat;
use crate::{byte_handler::ByteHandler, input_data::yaz1_decompress};
use crate::{compute_sha1_hex, datetime_from_timestamp, duration_from_ticks};
use chrono::{TimeDelta, prelude::*};

pub mod category;
pub mod ctgp_version;
pub mod exact_in_game_time;
pub mod region;

/// Errors that can occur while parsing a [`CTGPFooter`].
#[derive(thiserror::Error, Debug)]
pub enum CTGPFooterError {
    /// The ghost file does not contain the expected `CKGD` magic bytes.
    #[error("Ghost is not CKGD")]
    NotCKGD,
    /// Data passed is impossibly too short.
    #[error("Data length is too short")]
    DataLengthTooShort,
    /// Full RKG data was not passed.
    #[error("File is not an RKG file")]
    NotRKGD,
    /// The footer version byte is not one of the supported values (1, 2, 3, 5, 6, 7).
    #[error("Invalid CTGP footer version")]
    InvalidFooterVersion,
    /// A slice-to-array conversion failed while reading footer data.
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    /// A lap split index was out of range for the recorded lap count.
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    /// The category bytes could not be mapped to a known [`Category`].
    #[error("Category Error: {0}")]
    CategoryError(#[from] category::CategoryError),
    /// An in-game time field could not be parsed.
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] crate::header::in_game_time::InGameTimeError),
    /// Shroomstrat parsing failed.
    #[error("Shroomstrat Error: {0}")]
    ShroomstratError(#[from] crate::shroomstrat::ShroomstratError),
    /// A numeric string could not be parsed as an integer.
    #[error("Parse Int Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// Parsed representation of the CTGP-specific footer appended to Mario Kart Wii ghost files.
///
/// The footer stores metadata written by CTGP-R at the end of each recorded ghost, including
/// high-precision timing, version information, RTC timestamps, pause data, and various
/// integrity/cheat-detection flags.
///
/// Full documentation of CTGP footer data can be found at https://wiki.tockdom.com/wiki/CRKG_(File_Format).
pub struct CTGPFooter {
    /// Length of the footer in bytes, excluding the trailing CRC32.
    len: usize,
    /// The footer format version, which determines the layout of the footer.
    footer_version: u8,
    /// The run category as determined by CTGP's metadata.
    category: Category,
    /// Raw category byte, stored to preserve any anomalous values that map to the same [`Category`].
    category_byte: u8,
    /// Raw shortcut byte, stored to preserve any anomalous values that map to the same [`Category`].
    shortcut_byte: u8,
    /// Whether the player respawned at any point during the run.
    respawns: bool,
    /// Whether the Mii name in the ghost file has been replaced.
    has_mii_name_replaced: bool,
    /// Whether the Mii data in the ghost file has been replaced.
    has_mii_data_replaced: bool,
    /// Whether CTGP flagged this ghost as potentially cheated.
    potentially_cheated_ghost: bool,
    /// Whether CTGP flagged potential rapid-fire input during the run.
    potential_rapidfire: bool,
    /// Whether CTGP flagged a potential slowdown event during the run.
    potential_slowdown: bool,
    /// Whether the player went out of bounds during the run.
    went_oob: bool,
    /// Whether the player was launched by a cannon during the run.
    cannoned: bool,
    /// The shroomstrat used by the ghost.
    shroomstrat: Shroomstrat,
    /// Whether CTGP detected a dubious split-line intersection on the final lap.
    /// `None` for footer versions below 2.
    final_lap_dubious_intersection: Option<bool>,
    /// Whether a USB GameCube adapter was enabled during the run.
    /// `None` for footer versions below 2.
    usb_gamecube_enabled: Option<bool>,
    /// Whether any My Stuff content was actually used during the run.
    /// `None` for footer versions below 3.
    my_stuff_used: Option<bool>,
    /// Whether the player had My Stuff enabled during the run.
    /// `None` for footer versions below 3.
    my_stuff_enabled: Option<bool>,
    /// Whether the player had anti-TAS deliberately disabled during the run.
    /// `None` for footer versions below 7.
    anti_tas_deliberately_disabled: Option<bool>,
    /// Total RTC time the game was paused during the run.
    rtc_time_paused: TimeDelta,
    /// Raw Wii tick count for the paused duration, stored to enforce identical raw data to the file.
    rtc_time_paused_ticks: u64,
    /// RTC timestamp recorded when the race began.
    rtc_race_start: NaiveDateTime,
    /// Raw Wii tick count for the race-start timestamp, stored to enforce identical raw data to the file.
    rtc_race_start_ticks: u64,
    /// RTC timestamp recorded when the race ended.
    rtc_race_end: NaiveDateTime,
    /// Raw Wii tick count for the race-end timestamp, stored to enforce identical raw data to the file.
    rtc_race_end_ticks: u64,
    /// Sub-millisecond-accurate lap times, one per recorded lap.
    exact_lap_times: Vec<ExactInGameTime>,
    /// Whether the exact lap times are unreliable.
    exact_lap_times_unreliable: bool,
    /// The lap true time differences as their raw data representation (4-byte float, BE).
    lap_true_time_difference_data: Vec<[u8; 4]>,
    /// The reserved security data block introduced in footer version 5.
    /// `None` for footer versions below 5.
    security_data_v5: Option<[u8; 0x0C]>,
    /// The reserved security data block introduced in footer version 3.
    /// `None` for footer versions below 3.
    security_data_v3: Option<[u8; 0x04]>,
    /// The region of the disc used when the ghost was created.
    /// `None` for footer versions below 3.
    disc_region: Option<Region>,
    /// Per-lap flags indicating whether CTGP detected a dubious split-line intersection.
    /// `None` for footer versions below 2.
    lap_split_dubious_intersections: Option<Vec<bool>>,
    /// The CTGP CORE version the ghost was driven on.
    core_version: CTGPVersion,
    /// One or more CTGP release versions consistent with the footer's version bytes.
    /// `None` if the version bytes are unrecognized.
    possible_release_versions: Option<Vec<CTGPVersion>>,
    /// Sub-millisecond-accurate finish time derived from the in-game time and CTGP's correction factor.
    exact_finish_time: ExactInGameTime,
    /// Whether the exact finish time is unreliable.
    exact_finish_time_unreliable: bool,
    /// The finish true time difference as its raw data representation.
    finish_true_time_difference_data: [u8; 4],
    /// The player's unique CTGP player ID.
    player_id: u64,
    /// SHA-1 hash of the track file associated with this ghost.
    track_sha1: [u8; 0x14],
    /// The CTGP security data block preceding the track SHA1 that has been present since footer version 1.
    security_data: [u8; 0x48],
    /// The CTGP security data block related to its anti-TAS feature, introduced in footer version 7.
    /// `None` for footer versions below 7.
    anti_tas_security_data: Option<[u8; 0x0A]>,
    /// SHA-1 hash of the full ghost file.
    ghost_sha1: [u8; 0x14],
    /// In-game timestamps (relative to race start) at which each pause occurred.
    pause_times: Vec<InGameTime>,
    /// Number of laps recorded in the ghost.
    lap_count: u8,
}

impl CTGPFooter {
    /// Parses a [`CTGPFooter`] from a complete RKG ghost file byte slice.
    ///
    /// Validates the `CKGD` magic and footer version, then reads all footer fields
    /// including timing data, version info, RTC timestamps, pause events, and
    /// cheat-detection flags.
    ///
    /// # Arguments
    ///
    /// * `data` - The full raw bytes of the RKG ghost file, including the CTGP footer.
    ///
    /// # Errors
    ///
    /// Returns a [`CTGPFooterError`] if:
    /// - The `CKGD` magic bytes are absent ([`CTGPFooterError::NotCKGD`]).
    /// - The footer version is not supported ([`CTGPFooterError::InvalidFooterVersion`]).
    /// - Any byte slice conversion, integer parse, category parse, or time parse fails.
    pub fn new(data: &[u8]) -> Result<Self, CTGPFooterError> {
        let ghost_sha1 = compute_sha1_hex(data);

        let data = &data[..data.len() - 0x04];
        let end = data.len();

        // Minimum ~0x96 size for base RKG file
        // + minimum 0xD0 size for CTGP footer
        if data.len() < 0x166 {
            return Err(CTGPFooterError::DataLengthTooShort);
        }

        if data[..0x04] != *b"RKGD" {
            return Err(CTGPFooterError::NotRKGD);
        }

        if data[end - 0x04..] != *b"CKGD" {
            return Err(CTGPFooterError::NotCKGD);
        }

        let len = ByteHandler::try_from(&data[end - 0x08..][..0x04])?.copy_dword() as usize;

        let footer_version = data[end - 0x09];

        match footer_version {
            1 | 2 | 3 | 5 | 6 | 7 => {}
            _ => {
                return Err(CTGPFooterError::InvalidFooterVersion);
            }
        }

        let category_byte = data[end - 0x0A];
        let shortcut_byte = data[end - 0x0C];
        let category = Category::try_from(category_byte, shortcut_byte)?;

        let flags = ByteHandler::from(data[end - 0x0B]);
        let respawns = flags.read_bool(0);
        let has_mii_name_replaced = flags.read_bool(1);
        let has_mii_data_replaced = flags.read_bool(2);
        let potentially_cheated_ghost = flags.read_bool(3);
        let potential_rapidfire = flags.read_bool(4);
        let potential_slowdown = flags.read_bool(5);
        let went_oob = flags.read_bool(6);
        let cannoned = flags.read_bool(7);

        let shroomstrat = Shroomstrat::new(
            data[end - 0x0D],
            data[end - 0x0E],
            data[end - 0x0F],
            data[0x10],
        )?;

        let flags = ByteHandler::from(data[end - 0x10]);
        let final_lap_dubious_intersection = if footer_version >= 2 {
            Some(flags.read_bool(0))
        } else {
            None
        };

        let usb_gamecube_enabled = if footer_version >= 2 {
            Some(flags.read_bool(1))
        } else {
            None
        };

        let my_stuff_used = if footer_version >= 3 {
            Some(flags.read_bool(2))
        } else {
            None
        };

        let my_stuff_enabled = if footer_version >= 3 {
            Some(flags.read_bool(3))
        } else {
            None
        };

        let anti_tas_deliberately_disabled = if footer_version >= 7 {
            Some(flags.read_bool(4))
        } else {
            None
        };

        let rtc_time_paused_ticks = u64::from_be_bytes(data[end - 0x18..][..0x08].try_into()?);
        let rtc_time_paused = duration_from_ticks(rtc_time_paused_ticks);

        let rtc_race_start_ticks = u64::from_be_bytes(data[end - 0x20..][..0x08].try_into()?);
        let rtc_race_start = datetime_from_timestamp(rtc_race_start_ticks);

        let rtc_race_end_ticks = u64::from_be_bytes(data[end - 0x28..][..0x08].try_into()?);
        let rtc_race_end = datetime_from_timestamp(rtc_race_end_ticks);

        // Exact lap split calculation
        let lap_count = data[0x10] as usize;

        let lap_time_data = &data[0x11..0x11 + (3 * lap_count)];
        let mut lap_times = Vec::new();

        for lap_time in lap_time_data.chunks_exact(3) {
            lap_times.push(InGameTime::from_byte_handler(lap_time)?);
        }

        let lap_true_time_differences_data =
            &data[end - 0x28 - (lap_count * 0x04)..][..lap_count * 0x04];
        let mut lap_true_time_differences = Vec::new();
        lap_true_time_differences.resize(lap_count, [0u8; 4]);

        // The lap differences are stored in reverse because of the nature of how ctgp data is written
        // Maybe instead potentially reverse the entire lap true time differences data slice and parse
        // it as little endian f32?
        for (idx, difference) in lap_true_time_differences_data.chunks_exact(4).enumerate() {
            lap_true_time_differences[lap_count - 1 - idx] = difference.try_into().unwrap();
        }

        let mut exact_lap_times = Vec::new();
        exact_lap_times.resize(lap_count, ExactInGameTime::default());

        let mut previous_subtractions = 0i64;
        let mut subtraction_ps = 0i64;
        let mut exact_lap_times_unreliable = false;

        for lap_true_time_difference in lap_true_time_differences.iter() {
            if f32::from_be_bytes(*lap_true_time_difference) < -1.0 {
                exact_lap_times_unreliable = true;
            }
        }

        for (idx, exact_lap_time) in exact_lap_times.iter_mut().enumerate() {
            let lap_true_time_difference = if exact_lap_times_unreliable {
                0f32
            } else {
                f32::from_be_bytes(lap_true_time_differences[idx])
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
                lap_times[idx].igt_to_millis() as i64 * 1e+9 as i64 + true_time_ps_subtraction;
            let true_sec = (true_ps / 1e+12 as i64) as u8;
            let true_fractional_sec = true_ps % 1e+12 as i64;

            *exact_lap_time = if true_ps > 0 {
                ExactInGameTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
            } else {
                ExactInGameTime::default()
            };
        }

        let security_data_v5: Option<[u8; 12]> = if footer_version >= 5 {
            Some(data[end - 0x5C..][..0x0C].try_into().unwrap())
        } else {
            None
        };

        let security_data_v3: Option<[u8; 4]> = if footer_version >= 3 {
            Some(data[end - 0x60..][..0x04].try_into().unwrap())
        } else {
            None
        };

        let disc_region = if footer_version >= 3 {
            Some(Region::from(data[end - 0x61]))
        } else {
            None
        };

        let lap_split_dubious_intersections = if footer_version >= 2 {
            let mut intersections = Vec::new();
            let handler = ByteHandler::try_from(&data[end - 0x64..][..0x02])?;
            for lap in 0..lap_count {
                intersections.push(handler.read_bool(9 - lap as u8));
            }
            Some(intersections)
        } else {
            None
        };

        let core_version = if footer_version >= 2 {
            CTGPVersion::core_from(&data[end - 0x68..][..0x04])?
        } else {
            // Infer that the core version is 1.03.0134, since the next batch of updates after TT release was CORE 1.03.0136
            CTGPVersion::new(1, 3, 134, None)
        };

        let possible_release_versions = if footer_version >= 2 {
            CTGPVersion::from(&data[end - 0x68..][..0x04])
        } else {
            // Metadata version 2 was introduced in between the 1.03.1044 and 1046 update, so it must be 1.03.1044
            Some(Vec::from([CTGPVersion::new(1, 3, 1044, None)]))
        };

        // Exact finish time calculation
        let finish_time = InGameTime::from_byte_handler(&data[0x04..0x07])?;
        let finish_true_time_difference_data: [u8; 4] =
            data[end - 0x6C..][..0x04].try_into().unwrap();

        let mut finish_true_time_difference = f32::from_be_bytes(finish_true_time_difference_data);
        let exact_finish_time_unreliable = finish_true_time_difference < -1.0;

        if exact_finish_time_unreliable {
            finish_true_time_difference = 0.0;
        }

        let true_time_ps_subtraction = (finish_true_time_difference as f64 * 1e+9).floor() as i64;

        let true_ps = finish_time.igt_to_millis() as i64 * 1e+9 as i64 + true_time_ps_subtraction;
        let true_sec = (true_ps / 1e+12 as i64) as u8;
        let true_fractional_sec = true_ps % 1e+12 as i64;

        let exact_finish_time = if true_ps > 0 {
            ExactInGameTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
        } else {
            ExactInGameTime::default()
        };

        let player_id = u64::from_be_bytes(data[end - 0x74..][..0x08].try_into().unwrap());
        let track_sha1: [u8; 0x14] = data[end - 0x88..][..0x14].try_into().unwrap();
        let security_data: [u8; 0x48] = data[end - 0xD0..][..0x48].try_into().unwrap();

        let anti_tas_security_data: Option<[u8; 0x0A]> = if footer_version >= 7 {
            Some(data[end - 0xDA..][..0x0A].try_into().unwrap())
        } else {
            None
        };

        // Pause frame times
        let input_data = &data[0x88..data.len() - len];
        let mut pause_times = Vec::new();
        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);

        let mut current_input_byte = 8;
        let mut elapsed_frames = 1u32;
        while current_input_byte < 8 + face_input_count * 2 {
            let idx = current_input_byte as usize;
            let input = &input_data[idx..idx + 2];

            if contains_ctgp_pause(input[0]) {
                // Convert frame count to InGameTime
                // Subtract 240 frames for countdown, another 2 frames because CTGP logs the pause 2 frames after it actually happens
                let mut pause_timestamp_seconds = (elapsed_frames - 242) as f64 / 59.94;
                let mut minutes = 0;
                let mut seconds = 0;

                while pause_timestamp_seconds >= 60.0 {
                    minutes += 1;
                    pause_timestamp_seconds -= 60.0;
                }

                while pause_timestamp_seconds >= 1.0 {
                    seconds += 1;
                    pause_timestamp_seconds -= 1.0;
                }

                let milliseconds = (pause_timestamp_seconds * 1000.0) as u16;

                pause_times.push(InGameTime::new(minutes, seconds, milliseconds)?);
            }

            elapsed_frames += input[1] as u32;
            current_input_byte += 2;
        }

        Ok(Self {
            len,
            footer_version,
            category,
            category_byte,
            shortcut_byte,
            respawns,
            has_mii_name_replaced,
            has_mii_data_replaced,
            potentially_cheated_ghost,
            potential_rapidfire,
            potential_slowdown,
            went_oob,
            cannoned,
            shroomstrat,
            final_lap_dubious_intersection,
            usb_gamecube_enabled,
            my_stuff_used,
            my_stuff_enabled,
            anti_tas_deliberately_disabled,
            rtc_time_paused,
            rtc_time_paused_ticks,
            rtc_race_start,
            rtc_race_start_ticks,
            rtc_race_end,
            rtc_race_end_ticks,
            exact_lap_times,
            exact_lap_times_unreliable,
            lap_true_time_difference_data: lap_true_time_differences,
            security_data_v5,
            security_data_v3,
            disc_region,
            lap_split_dubious_intersections,
            core_version,
            possible_release_versions,
            exact_finish_time,
            exact_finish_time_unreliable,
            finish_true_time_difference_data,
            player_id,
            track_sha1,
            security_data,
            anti_tas_security_data,
            ghost_sha1,
            pause_times,
            lap_count: lap_count as u8,
        })
    }

    /// Returns the raw bytes of the footer, excluding the trailing CRC32.
    /// Size is 0xE0 for footer version 7+, 0xD0 for version 6 and below.
    pub fn raw_data(&self) -> Vec<u8> {
        let footer_size: usize = if self.footer_version >= 7 { 0xE0 } else { 0xD0 };
        let mut data = vec![0u8; footer_size];
        let end = footer_size;
        let lap_count = self.lap_count as usize;

        // "CKGD" magic
        data[end - 0x04..end].copy_from_slice(b"CKGD");

        // len (big-endian u32)
        data[end - 0x08..end - 0x04].copy_from_slice(&(footer_size as u32).to_be_bytes());

        // footer_version
        data[end - 0x09] = self.footer_version;

        // category_byte (end-0x0A) and shortcut_byte (end-0x0C)
        data[end - 0x0A] = self.category_byte;
        data[end - 0x0C] = self.shortcut_byte;

        // flags: respawns, has_mii_name_replaced, ..., cannoned
        data[end - 0x0B] = (self.respawns as u8)
            | ((self.has_mii_name_replaced as u8) << 1)
            | ((self.has_mii_data_replaced as u8) << 2)
            | ((self.potentially_cheated_ghost as u8) << 3)
            | ((self.potential_rapidfire as u8) << 4)
            | ((self.potential_slowdown as u8) << 5)
            | ((self.went_oob as u8) << 6)
            | ((self.cannoned as u8) << 7);

        // shroomstrat: shroom_1_usage, shroom_2_usage, shroom_3_usage
        let shroom_bytes = self.shroomstrat.to_raw_bytes();
        data[end - 0x0D] = shroom_bytes[0];
        data[end - 0x0E] = shroom_bytes[1];
        data[end - 0x0F] = shroom_bytes[2];

        // flags2: final_lap_dubious_intersection, usb_gamecube_enabled, my_stuff_used, my_stuff_enabled
        data[end - 0x10] = (self.final_lap_dubious_intersection.unwrap_or(false) as u8)
            | ((self.usb_gamecube_enabled.unwrap_or(false) as u8) << 1)
            | ((self.my_stuff_used.unwrap_or(false) as u8) << 2)
            | ((self.my_stuff_enabled.unwrap_or(false) as u8) << 3)
            | ((self.anti_tas_deliberately_disabled.unwrap_or(false) as u8) << 4);

        // rtc_time_paused (8 bytes, big-endian u64 ticks)
        data[end - 0x18..end - 0x10].copy_from_slice(&self.rtc_time_paused_ticks.to_be_bytes());

        // rtc_race_start (8 bytes)
        data[end - 0x20..end - 0x18].copy_from_slice(&self.rtc_race_start_ticks.to_be_bytes());

        // rtc_race_end (8 bytes)
        data[end - 0x28..end - 0x20].copy_from_slice(&self.rtc_race_end_ticks.to_be_bytes());

        // lap_true_time_differences, stored in reverse order in the file
        let base = end - 0x28 - lap_count * 4;
        for (i, diff) in self.lap_true_time_difference_data.iter().rev().enumerate() {
            data[base + i * 4..base + i * 4 + 4].copy_from_slice(diff);
        }

        // security_data_v5 (0x0C bytes, footer version >= 5)
        if let Some(sd) = &self.security_data_v5 {
            data[end - 0x5C..end - 0x50].copy_from_slice(sd);
        }

        // security_data_v3 (0x04 bytes, footer version >= 3)
        if let Some(sd) = &self.security_data_v3 {
            data[end - 0x60..end - 0x5C].copy_from_slice(sd);
        }

        // disc_region (1 byte, footer version >= 3)
        if let Some(region) = self.disc_region {
            data[end - 0x61] = u8::from(region);
        }

        // lap_split_dubious_intersections (2 bytes, footer version >= 2)
        // bit (9 - lap) of the big-endian u16 stores each lap's flag
        if let Some(intersections) = &self.lap_split_dubious_intersections {
            let mut word: u16 = 0;
            for (lap, &intersection) in intersections.iter().take(lap_count).enumerate() {
                if intersection {
                    word |= 1u16 << (9 - lap);
                }
            }
            data[end - 0x64..end - 0x62].copy_from_slice(&word.to_be_bytes());
        }

        // core_version (4 bytes, footer version >= 2)
        if self.footer_version >= 2 {
            data[end - 0x68..end - 0x64].copy_from_slice(&self.core_version.to_core_bytes());
        }

        // finish_true_time_difference_data (4 bytes)
        data[end - 0x6C..end - 0x68].copy_from_slice(&self.finish_true_time_difference_data);

        // player_id (8 bytes)
        data[end - 0x74..end - 0x6C].copy_from_slice(&self.player_id.to_be_bytes());

        // track_sha1 (0x14 bytes)
        data[end - 0x88..end - 0x74].copy_from_slice(&self.track_sha1);

        // security_data (0x48 bytes)
        data[end - 0xD0..end - 0x88].copy_from_slice(&self.security_data);

        // anti_tas_security_data (0x0A bytes, footer version >= 7)
        if let Some(atsd) = &self.anti_tas_security_data {
            data[end - 0xDA..end - 0xD0].copy_from_slice(atsd);
        }

        data
    }

    /// Returns the security/signature portion of the footer.
    pub fn security_data(&self) -> &[u8] {
        &self.security_data
    }

    /// Returns the reserved security data block introduced in footer version 5.
    ///
    /// Returns `None` for footer versions below 5.
    pub fn security_data_v5(&self) -> Option<&[u8]> {
        self.security_data_v5.as_ref().map(|d| d.as_ref())
    }

    /// Returns the reserved security data block introduced in footer version 3.
    ///
    /// Returns `None` for footer versions below 3.
    pub fn security_data_v3(&self) -> Option<&[u8]> {
        self.security_data_v3.as_ref().map(|d| d.as_ref())
    }

    /// Returns the anti-TAS security data block introduced in footer version 7.
    ///
    /// Returns `None` for footer versions below 7.
    pub fn anti_tas_security_data(&self) -> Option<&[u8]> {
        self.anti_tas_security_data.as_ref().map(|d| d.as_ref())
    }

    /// Returns the SHA-1 hash of the track file associated with this ghost.
    pub fn track_sha1(&self) -> &[u8] {
        &self.track_sha1
    }

    /// Returns the SHA-1 hash of the full ghost file.
    pub fn ghost_sha1(&self) -> &[u8] {
        &self.ghost_sha1
    }

    /// Overwrites the stored ghost SHA-1 hash.
    ///
    /// # Errors
    ///
    /// Returns [`CTGPFooterError::TryFromSliceError`] if `ghost_sha1` is not exactly 20 bytes.
    pub(crate) fn set_ghost_sha1(&mut self, ghost_sha1: &[u8]) -> Result<(), CTGPFooterError> {
        self.ghost_sha1 = ghost_sha1.try_into()?;
        Ok(())
    }

    /// Returns the player's unique CTGP player ID.
    pub fn player_id(&self) -> u64 {
        self.player_id
    }

    /// Returns the sub-millisecond-accurate finish time for the run.
    pub fn exact_finish_time(&self) -> ExactInGameTime {
        self.exact_finish_time
    }

    /// Returns whether the exact finish time is unreliable.
    pub fn exact_finish_time_unreliable(&self) -> bool {
        self.exact_finish_time_unreliable
    }

    /// Returns the finish true time difference as its raw 4-byte big-endian float representation.
    pub fn finish_true_time_difference_data(&self) -> &[u8] {
        &self.finish_true_time_difference_data
    }

    /// Returns the CORE (base game) version the ghost was driven on.
    pub fn core_version(&self) -> CTGPVersion {
        self.core_version
    }

    /// Returns the possible CTGP release versions consistent with this ghost's footer bytes.
    ///
    /// Returns `None` if the version bytes did not match any known release.
    pub fn possible_release_versions(&self) -> Option<&[CTGPVersion]> {
        self.possible_release_versions.as_deref()
    }

    /// Returns per-lap dubious split-line intersection flags for the recorded laps.
    ///
    /// Returns `None` for ghosts recorded on footer version 1, which does not include this data.
    pub fn lap_split_dubious_intersections(&self) -> Option<&[bool]> {
        if let Some(intersections) = &self.lap_split_dubious_intersections {
            let lap_count = std::cmp::min(intersections.len(), self.lap_count as usize);
            return Some(&intersections[0..lap_count]);
        }
        None
    }

    /// Returns the region of the disc used when the ghost was set.
    ///
    /// Returns `None` for ghosts recorded on footer version 1 or 2, which do not include this data.
    pub fn disc_region(&self) -> Option<Region> {
        self.disc_region
    }

    /// Returns the sub-millisecond-accurate lap times for all recorded laps.
    pub fn exact_lap_times(&self) -> &[ExactInGameTime] {
        let lap_count = std::cmp::min(self.exact_lap_times.len(), self.lap_count as usize);
        &self.exact_lap_times[0..lap_count]
    }

    /// Returns the sub-millisecond-accurate time for a single lap by index.
    ///
    /// # Errors
    ///
    /// Returns [`CTGPFooterError::LapSplitIndexError`] if `idx` is greater than or equal to
    /// the number of recorded laps.
    pub fn exact_lap_time(&self, idx: usize) -> Result<ExactInGameTime, CTGPFooterError> {
        if idx >= self.lap_count as usize || idx >= self.exact_lap_times.len() {
            return Err(CTGPFooterError::LapSplitIndexError);
        }
        Ok(self.exact_lap_times[idx])
    }

    /// Returns whether the exact lap times are unreliable.
    pub fn exact_lap_times_unreliable(&self) -> bool {
        self.exact_lap_times_unreliable
    }

    /// Returns the raw lap true time difference data, one 4-byte big-endian float per lap.
    pub fn lap_true_time_difference_data(&self) -> &[[u8; 4]] {
        &self.lap_true_time_difference_data
    }

    /// Returns the real-time clock timestamp recorded when the race ended.
    pub fn rtc_race_end(&self) -> NaiveDateTime {
        self.rtc_race_end
    }

    /// Returns the real-time clock timestamp recorded when the race began.
    pub fn rtc_race_begins(&self) -> NaiveDateTime {
        self.rtc_race_start
    }

    /// Returns the total wall-clock duration the game was paused during the run.
    pub fn rtc_time_paused(&self) -> TimeDelta {
        self.rtc_time_paused
    }

    /// Returns the in-game timestamps at which each pause occurred during the run.
    pub fn pause_times(&self) -> &Vec<InGameTime> {
        &self.pause_times
    }

    /// Returns whether the player had anti-TAS deliberately disabled during the run.
    pub fn anti_tas_deliberately_disabled(&self) -> Option<bool> {
        self.anti_tas_deliberately_disabled
    }

    /// Returns whether the player had My Stuff enabled during the run.
    pub fn my_stuff_enabled(&self) -> Option<bool> {
        self.my_stuff_enabled
    }

    /// Returns whether any My Stuff content was actually used during the run.
    pub fn my_stuff_used(&self) -> Option<bool> {
        self.my_stuff_used
    }

    /// Returns whether a USB GameCube adapter was enabled during the run.
    pub fn usb_gamecube_enabled(&self) -> Option<bool> {
        self.usb_gamecube_enabled
    }

    /// Returns whether CTGP detected a suspicious split-line intersection on the final lap.
    pub fn final_lap_dubious_intersection(&self) -> Option<bool> {
        self.final_lap_dubious_intersection
    }

    /// Returns the shroomstrat of the ghost.
    pub fn shroomstrat(&self) -> Shroomstrat {
        self.shroomstrat
    }

    /// Returns whether the player was launched by a cannon during the run.
    pub fn cannoned(&self) -> bool {
        self.cannoned
    }

    /// Returns whether the player went out of bounds during the run.
    pub fn went_oob(&self) -> bool {
        self.went_oob
    }

    /// Returns whether CTGP flagged a potential slowdown event during the run.
    pub fn potential_slowdown(&self) -> bool {
        self.potential_slowdown
    }

    /// Returns whether CTGP flagged potential rapid-fire input during the run.
    pub fn potential_rapidfire(&self) -> bool {
        self.potential_rapidfire
    }

    /// Returns whether CTGP's heuristics flagged this ghost as potentially cheated.
    pub fn potentially_cheated_ghost(&self) -> bool {
        self.potentially_cheated_ghost
    }

    /// Returns whether the Mii data in the ghost file has been replaced.
    pub fn has_mii_data_replaced(&self) -> bool {
        self.has_mii_data_replaced
    }

    /// Returns whether the Mii name in the ghost file has been replaced.
    pub fn has_name_replaced(&self) -> bool {
        self.has_mii_name_replaced
    }

    /// Returns whether the player respawned at any point during the run.
    pub fn respawns(&self) -> bool {
        self.respawns
    }

    /// Returns the run category as determined by CTGP's metadata.
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the footer format version number.
    pub fn footer_version(&self) -> u8 {
        self.footer_version
    }

    /// Returns the length of the CTGP footer in bytes, excluding the trailing CRC32.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the footer has zero length.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the raw category byte as stored in the footer.
    ///
    /// Use this instead of deriving from [`CTGPFooter::category`] when exact byte value matters,
    /// as multiple byte values can map to the same [`Category`] variant.
    pub fn category_byte(&self) -> u8 {
        self.category_byte
    }

    /// Returns the raw shortcut byte as stored in the footer.
    ///
    /// The parsed [`Category`] only distinguishes zero vs. non-zero; this preserves the original value.
    pub fn shortcut_byte(&self) -> u8 {
        self.shortcut_byte
    }

    /// Returns the raw Wii tick count for the paused duration.
    pub fn rtc_time_paused_ticks(&self) -> u64 {
        self.rtc_time_paused_ticks
    }

    /// Returns the raw Wii tick count for the race-start timestamp.
    pub fn rtc_race_start_ticks(&self) -> u64 {
        self.rtc_race_start_ticks
    }

    /// Returns the raw Wii tick count for the race-end timestamp.
    pub fn rtc_race_end_ticks(&self) -> u64 {
        self.rtc_race_end_ticks
    }
}

/// Returns `true` if the given face-button byte indicates a CTGP pause event.
///
/// CTGP encodes pauses in bit 6 (`0x40`) of the face-button input byte.
fn contains_ctgp_pause(buttons: u8) -> bool {
    buttons & 0x40 != 0
}
