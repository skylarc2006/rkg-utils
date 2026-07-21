//! This crate is a library to completely and coherently access the data in RKGD files,
//! Mario Kart Wii's native Ghost Data.
//!
//! Features:
//! - Reading and Writing Vanilla Game Data (including embedded Mii data)
//! - Creating new RKGD Ghost Data files
//! - Reading and Writing Pulsar (Retro Rewind) Modified Data
//! - Reading and Writing CTGP Modified Data
//! - Reading and Writing MKW-SP Modified Data

use std::{
    array::TryFromSliceError,
    io::{Read, Write},
};

use chrono::{DateTime, Duration, NaiveDateTime, TimeDelta};
use sha1::{Digest, Sha1};

use crate::{crc::crc32, input_data::CompressionMethod};

pub mod byte_handler;
pub mod crc;
pub mod footer;
pub mod header;
pub mod input_data;
pub mod shroomstrat;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use footer::{CTGPFooter, FooterType, SPFooter};
#[doc(inline)]
pub use header::{Combo, Header, HeaderError, Mii};
#[doc(inline)]
pub use input_data::{ControllerInput, InputData, InputDataError};
#[doc(inline)]
pub use shroomstrat::Shroomstrat;

/// Errors that can occur while parsing or modifying a [`Ghost`].
#[derive(thiserror::Error, Debug)]
pub enum GhostError {
    /// Error with data length being too short.
    #[error("Data length too short")]
    DataLengthTooShort,
    /// The RKG file header could not be parsed.
    #[error("Header Error: {0}")]
    HeaderError(#[from] header::HeaderError),
    /// The embedded Mii data could not be parsed.
    #[error("Mii Error: {0}")]
    MiiError(#[from] header::mii::MiiError),
    /// The ghost input data could not be parsed.
    #[error("Input Data Error: {0}")]
    InputDataError(#[from] input_data::InputDataError),
    /// The CTGP footer could not be parsed.
    #[error("CTGP Footer Error: {0}")]
    CTGPFooterError(#[from] footer::ctgp_footer::CTGPFooterError),
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] byte_handler::ByteHandlerError),
    /// A slice-to-array conversion failed (e.g. when extracting a CRC-32 word).
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),
    /// A file I/O operation failed.
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

/// A fully parsed Mario Kart Wii RKG ghost file.
///
/// Holds the file header, decompressed or compressed input data, and an optional
/// external footer (CTGP or SP). All CRC values and the raw byte representation
/// are computed on demand via their respective methods.
pub struct Ghost {
    /// The parsed ghost header.
    header: Header,
    /// The ghost's controller input data (compressed or decompressed).
    input_data: InputData,
    /// The footer appended to the file, if present.
    footer: Option<FooterType>,
    /// When `true`, any existing external footer is preserved when saving.
    should_preserve_external_footer: bool,
}

impl Ghost {
    /// Creates a [`Ghost`] from a [`Header`] and [`InputData`].
    ///
    /// Sets `is_compressed` and `decompressed_input_data_length` on the header
    /// to match the provided input data. No footer is attached and
    /// `should_preserve_external_footer` is set to `false`.
    pub fn new(mut header: Header, input_data: InputData) -> Self {
        header.set_compressed(input_data.compressed());
        let decompressed_len = 8u16
            + input_data.face_button_input_count() * 2
            + input_data.stick_input_count() * 2
            + input_data.dpad_button_input_count() * 2;
        header.set_decompressed_input_data_length(decompressed_len);
        Self {
            header,
            input_data,
            footer: None,
            should_preserve_external_footer: false,
        }
    }

    /// Parses a [`Ghost`] from an RKG file at the given path.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::IOError`] if the file cannot be opened or read,
    /// and other [`GhostError`] variants if parsing fails.
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, GhostError> {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)?.read_to_end(&mut buf)?;
        Self::new_from_bytes(&buf)
    }

    /// Parses a [`Ghost`] from a byte slice.
    ///
    /// Detects and parses an optional CTGP or SP footer if present.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::DataLengthTooShort`] if `bytes` is shorter than
    /// `0x90` bytes OR input data is shorter than the expected input data size.
    /// Returns other [`GhostError`] variants if any field fails to parse.
    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self, GhostError> {
        if bytes.len() < 0x90 {
            return Err(GhostError::DataLengthTooShort);
        }

        let header = Header::new_from_bytes(&bytes[..0x88])?;

        let input_data_len = if bytes[0x8C..0x90] == *b"Yaz1" {
            u32::from_be_bytes(bytes[0x88..0x8C].try_into().unwrap()) as usize + 0x04
        } else {
            header.decompressed_input_data_length() as usize
        };

        if bytes.len() < 0x88 + input_data_len + 0x04 {
            return Err(GhostError::DataLengthTooShort);
        }

        let mut input_data = InputData::new_from_bytes(&bytes[0x88..0x88 + input_data_len])?;

        let footer = if let Ok(ctgp_footer) = CTGPFooter::new(bytes) {
            input_data.set_compression_method(CompressionMethod::CTGP);
            Some(FooterType::CTGPFooter(ctgp_footer))
        } else if let Ok(sp_footer) = SPFooter::new(bytes) {
            input_data.set_compression_method(CompressionMethod::SP);
            Some(FooterType::SPFooter(sp_footer))
        } else {
            let section_len = if input_data.compressed() {
                input_data_len
            } else if bytes.len() >= 0x88 + 0x2774 + 4 {
                0x2774
            } else {
                input_data_len
            };
            let footer_start = 0x88 + section_len + 4;
            let footer_end = bytes.len().saturating_sub(4);
            if footer_start < footer_end {
                Some(FooterType::Unknown(
                    bytes[footer_start..footer_end].to_vec(),
                ))
            } else {
                None
            }
        };

        Ok(Self {
            header,
            input_data,
            footer,
            should_preserve_external_footer: true,
        })
    }

    /// Updates the SHA-1 field in the CTGP footer, if one is present.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::CTGPFooterError`] if the SHA-1 field cannot be written,
    /// or [`GhostError::InputDataError`] if the input data cannot be serialized.
    pub fn update_ghost_sha1(&mut self) -> Result<(), GhostError> {
        let sha1 = compute_sha1_hex(&self.raw_data()?);
        if let Some(FooterType::CTGPFooter(ctgp_footer)) = self.footer_mut() {
            ctgp_footer.set_ghost_sha1(&sha1)?;
        }
        Ok(())
    }

    /// Flushes modifications and writes the ghost to a file at the given path.
    ///
    /// # Errors
    ///
    /// Returns any error from [`update_ghost_sha1`](Ghost::update_ghost_sha1) or
    /// from file creation/writing.
    pub fn save_to_file<T: AsRef<std::path::Path>>(&mut self, path: T) -> Result<(), GhostError> {
        self.update_ghost_sha1()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(&self.raw_data()?)?;
        Ok(())
    }

    /// Sets compression flag in `InputData` and the compression flag in the header.
    pub fn set_input_data_compressed(&mut self, compressed: bool) {
        self.input_data_mut().set_compressed(compressed);
        self.header_mut().set_compressed(compressed);
    }

    /// Returns the computed raw file bytes.
    ///
    /// The result always reflects the current state of all parsed fields.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::InputDataError`] if the input data's uncompressed
    /// raw representation would exceed `0x2774` bytes.
    pub fn raw_data(&mut self) -> Result<Vec<u8>, GhostError> {
        let mut buf = Vec::from(self.header.raw_data());

        if let Some(FooterType::CTGPFooter(_)) = &self.footer()
            && self.should_preserve_external_footer
        {
            let _ = &self
                .input_data_mut()
                .set_compression_method(CompressionMethod::CTGP);
        } else if let Some(FooterType::SPFooter(_)) = &self.footer()
            && self.should_preserve_external_footer
        {
            let _ = &self
                .input_data_mut()
                .set_compression_method(CompressionMethod::SP);
        } else {
            let _ = &self
                .input_data_mut()
                .set_compression_method(CompressionMethod::Vanilla);
        }

        buf.extend_from_slice(&self.input_data.raw_data()?);
        let base_crc32 = crc32(&buf);
        buf.extend_from_slice(&base_crc32.to_be_bytes());
        if self.should_preserve_external_footer
            && let Some(footer) = &self.footer
        {
            buf.extend_from_slice(&footer.raw_data());
        }
        let file_crc32 = crc32(&buf);
        buf.extend_from_slice(&file_crc32.to_be_bytes());
        Ok(buf)
    }

    /// Returns the parsed RKG file header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a mutable reference to the parsed RKG file header.
    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    /// Returns the ghost's controller input data.
    pub fn input_data(&self) -> &InputData {
        &self.input_data
    }

    /// Returns a mutable reference to the ghost's controller input data.
    pub fn input_data_mut(&mut self) -> &mut InputData {
        &mut self.input_data
    }

    /// Returns the footer, if present.
    pub fn footer(&self) -> Option<&FooterType> {
        self.footer.as_ref()
    }

    /// Returns a mutable reference to the footer, if present.
    pub fn footer_mut(&mut self) -> Option<&mut FooterType> {
        self.footer.as_mut()
    }

    /// Returns the shroomstrat of the ghost. If a CTGP or MKW-SP footer (footer version >= 1) is present
    /// and [`Ghost::should_preserve_external_footer`] is true, it uses the info from its footer. If not,
    /// input data is used to estimate the shroomstrat. Shroomstrat estimation may be inaccurate if the
    /// item button is pressed during a cannon, after a respawn, while being spun out by something, or
    /// otherwise losing mushrooms early.
    pub fn shroomstrat(&self) -> Shroomstrat {
        if self.should_preserve_external_footer()
            && let Some(footer) = self.footer()
        {
            match footer {
                FooterType::CTGPFooter(f) => return f.shroomstrat(),
                FooterType::SPFooter(f) => {
                    if f.footer_version() >= 1 {
                        return f.shroomstrat().unwrap();
                    }
                }
                FooterType::Unknown(_) => (),
            }
        }

        const MILLISECONDS_PER_FRAME: f64 = 1000f64 / 59.94f64;
        let mut current_shroom_index = 0;
        let mut shroom_usages = [0u8; 3];
        let mut current_frames = 0;
        let mut previous_input = &ControllerInput::default();

        let mut lap_splits_ms = Vec::new();
        for lap in self.header().lap_split_times().iter() {
            lap_splits_ms.push(lap.to_milliseconds());
        }

        for input in self.input_data().controller_inputs().iter() {
            if current_frames >= 240
                && current_shroom_index < 3
                && input.item()
                && !previous_input.item()
            {
                let current_ms =
                    (MILLISECONDS_PER_FRAME * (current_frames - 240) as f64).floor() as u32;

                let mut current_lap = 1u8;
                let mut previous_laps_ms_sum = 0u32;

                for lap_split in lap_splits_ms.iter() {
                    if current_ms < previous_laps_ms_sum + *lap_split {
                        break;
                    }
                    previous_laps_ms_sum += lap_split;
                    current_lap += 1;
                }

                if current_lap <= self.header().lap_count() {
                    shroom_usages[current_shroom_index] = current_lap;
                    current_shroom_index += 1;
                }
            }

            current_frames += input.frame_duration();
            previous_input = input;
        }

        Shroomstrat::new(
            shroom_usages[0],
            shroom_usages[1],
            shroom_usages[2],
            self.header().lap_count(),
        )
        .unwrap()
    }

    /// Returns the CRC-32 of the header and input data, excluding any footer.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::InputDataError`] if the input data's uncompressed
    /// raw representation would exceed `0x2774` bytes.
    pub fn base_crc32(&self) -> Result<u32, GhostError> {
        let mut buf = Vec::from(self.header.raw_data());
        buf.extend_from_slice(&self.input_data.raw_data()?);
        Ok(crc32(&buf))
    }

    /// Returns the CRC-32 of the entire file excluding its final 4 bytes.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::InputDataError`] if the input data's uncompressed
    /// raw representation would exceed `0x2774` bytes.
    pub fn file_crc32(&mut self) -> Result<u32, GhostError> {
        let raw = self.raw_data()?;
        Ok(crc32(&raw[..raw.len() - 4]))
    }

    /// Returns whether an existing external footer will be preserved when saving.
    pub fn should_preserve_external_footer(&self) -> bool {
        self.should_preserve_external_footer
    }

    /// Sets whether an existing external footer should be preserved when saving.
    pub fn set_should_preserve_external_footer(&mut self, b: bool) {
        self.should_preserve_external_footer = b;
    }
}

/// Writes `bit_width` bits of `value` into `buf`, starting `bit_offset` bits past
/// `byte_offset` bytes from the start of the buffer, in big-endian bit order.
///
/// # Panics
///
/// Panics if `byte_offset`, `bit_offset`, and `bit_width` together describe a range
/// that falls outside `buf`.
pub(crate) fn write_bits(
    buf: &mut [u8],
    byte_offset: usize,
    bit_offset: usize,
    bit_width: usize,
    value: u64,
) {
    let bytes_needed = (bit_offset + bit_width).div_ceil(8);
    let mut chunk: u64 = 0;

    for i in 0..bytes_needed {
        chunk = (chunk << 8) | buf[byte_offset + i] as u64;
    }

    let shift = bytes_needed * 8 - bit_offset - bit_width;
    let mask = ((1u64 << bit_width) - 1) << shift;

    chunk = (chunk & !mask) | ((value << shift) & mask);

    for i in (0..bytes_needed).rev() {
        buf[byte_offset + i] = (chunk & 0xFF) as u8;
        chunk >>= 8;
    }
}

/// Computes the SHA-1 hash of `input` and returns it as a 20-byte array.
pub(crate) fn compute_sha1_hex(input: &[u8]) -> [u8; 0x14] {
    let mut hasher = Sha1::new();
    hasher.update(input);
    hasher.finalize().into()
}

/// Converts a raw Wii tick count into a [`NaiveDateTime`].
///
/// Ticks run at 60.75 MHz relative to the Wii epoch of 2000-01-01 00:00:00.
pub(crate) fn datetime_from_timestamp(tick_count: u64) -> NaiveDateTime {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let epoch_shift = 946_684_800; // Shifts epoch from 1970-01-01 to 2000-01-01 (which is what the Wii uses)
    let total_seconds = tick_count as f64 / clock_rate;
    let total_nanoseconds = (total_seconds * 1_000_000_000.0) as i64;

    let duration = Duration::nanoseconds(total_nanoseconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

/// Converts a raw Wii tick count into a [`TimeDelta`] (duration from an arbitrary reference).
///
/// Ticks run at 60.75 MHz; the result is truncated to millisecond precision.
pub(crate) fn duration_from_ticks(tick_count: u64) -> TimeDelta {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let total_seconds = tick_count as f64 / clock_rate;
    let total_milliseconds = (total_seconds * 1_000.0) as i64;

    Duration::milliseconds(total_milliseconds)
}
