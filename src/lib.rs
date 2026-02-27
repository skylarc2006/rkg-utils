use std::{
    array::TryFromSliceError,
    io::{Read, Write},
};

use chrono::{DateTime, Duration, NaiveDateTime, TimeDelta};
use sha1::{Digest, Sha1};

use crate::{
    crc::crc32,
    ctgp_metadata::CTGPMetadata,
    header::{Header, mii::Mii},
    input_data::InputData,
};

pub mod byte_handler;
pub mod crc;
pub mod ctgp_metadata;
pub mod header;
pub mod input_data;

/*
 * TODO:
 * Unfinished/unimplemented functionality
 * ----------------------------------------------
 * Implement MKW-SP footer support
 * Implement Retro Rewind data support
 * Implement Pulsar data support
 * Implement TryFrom<_> for T where T: Into<ByteHandler>, relies on https://github.com/rust-lang/rust/issues/31844 currently
 * Represent at a Type-system level which types can convert from T to TypeHandler to whichever Struct
 * Optimize Little-Endian calculations
 * Figure out whether Big-Endian works
 */

#[cfg(test)]
mod tests;

#[derive(thiserror::Error, Debug)]
pub enum GhostError {
    #[error("Header Error: {0}")]
    HeaderError(#[from] header::HeaderError),
    #[error("Mii Error: {0}")]
    MiiError(#[from] header::mii::MiiError),
    #[error("Input Data Error: {0}")]
    InputDataError(#[from] input_data::InputDataError),
    #[error("CTGP Metadata Error: {0}")]
    CTGPMetadataError(#[from] ctgp_metadata::CTGPMetadataError),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] byte_handler::ByteHandlerError),
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

pub struct Ghost {
    header: Header,
    input_data: InputData,
    base_crc32: u32,
    ctgp_metadata: Option<CTGPMetadata>,
    file_crc32: u32,
    should_preserve_external_metadata: bool,
}

impl Ghost {
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, GhostError> {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)?.read_to_end(&mut buf)?;
        Self::new(&buf)
    }

    pub fn new(bytes: &[u8]) -> Result<Self, GhostError> {
        let header = Header::new(&bytes[..0x88])?;

        let file_crc32 = u32::from_be_bytes(bytes[bytes.len() - 0x04..].try_into()?);
        let base_crc32;

        let ctgp_metadata = if let Ok(ctgp_metadata) = CTGPMetadata::new(bytes) {
            let input_data_end_offset = bytes.len() - ctgp_metadata.len() - 0x04;
            base_crc32 = u32::from_be_bytes(
                bytes[input_data_end_offset..input_data_end_offset + 0x04].try_into()?,
            );
            Some(ctgp_metadata)
        } else {
            base_crc32 = file_crc32;
            None
        };

        let input_data_len = if bytes[0x8C..0x90] == *b"Yaz1" {
            u32::from_be_bytes(bytes[0x88..0x8C].try_into().unwrap()) as usize + 0x04
        } else {
            header.decompressed_input_data_length() as usize
        };

        let input_data = InputData::new(&bytes[0x88..0x88 + input_data_len])?;

        Ok(Self {
            header,
            input_data,
            base_crc32,
            ctgp_metadata,
            file_crc32,
            should_preserve_external_metadata: true,
        })
    }

    pub fn save_to_bytes(&mut self) -> Result<Vec<u8>, GhostError> {
        let mii_bytes = self.header().mii().raw_data().to_vec();
        self.header_mut().set_mii(Mii::new(mii_bytes)?);
        self.header_mut().fix_mii_crc16();

        let mut buf = Vec::from(self.header().raw_data());

        buf.extend_from_slice(self.input_data().raw_data());

        if let Some(ctgp_metadata) = self.ctgp_metadata()
            && self.should_preserve_external_metadata()
        {
            let base_crc32 = crc32(&buf);
            buf.extend_from_slice(&base_crc32.to_be_bytes());
            buf.extend_from_slice(ctgp_metadata.raw_data());
        }

        let crc32 = crc32(&buf);
        buf.extend_from_slice(&crc32.to_be_bytes());

        if let Some(ctgp_metadata) = self.ctgp_metadata_mut() {
            ctgp_metadata.set_ghost_sha1(&compute_sha1_hex(&buf))?;
        }

        Ok(buf)
    }

    pub fn save_to_file<T: AsRef<std::path::Path>>(&mut self, path: T) -> Result<(), GhostError> {
        let buf = self.save_to_bytes()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(&buf)?;

        Ok(())
    }

    pub fn compress_input_data(&mut self) {
        if self.input_data().is_compressed() {
            return;
        }

        self.input_data_mut().compress();
        self.header_mut().set_compressed(true);
    }

    pub fn decompress_input_data(&mut self) {
        if !self.input_data().is_compressed() {
            return;
        }

        self.input_data_mut().decompress();
        self.header_mut().set_compressed(false);
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn input_data(&self) -> &InputData {
        &self.input_data
    }

    pub fn input_data_mut(&mut self) -> &mut InputData {
        &mut self.input_data
    }

    pub fn ctgp_metadata(&self) -> Option<&CTGPMetadata> {
        self.ctgp_metadata.as_ref()
    }

    pub fn ctgp_metadata_mut(&mut self) -> Option<&mut CTGPMetadata> {
        self.ctgp_metadata.as_mut()
    }

    pub fn base_crc32(&self) -> u32 {
        self.base_crc32
    }

    pub fn file_crc32(&self) -> u32 {
        self.file_crc32
    }

    pub fn should_preserve_external_metadata(&self) -> bool {
        self.should_preserve_external_metadata
    }

    pub fn set_should_preserve_external_metadata(&mut self, b: bool) {
        self.should_preserve_external_metadata = b;
    }
}

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

pub(crate) fn compute_sha1_hex(input: &[u8]) -> [u8; 0x14] {
    let mut hasher = Sha1::new();
    hasher.update(input);
    hasher.finalize().into()
}

pub(crate) fn datetime_from_timestamp(tick_count: u64) -> NaiveDateTime {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let epoch_shift = 946_684_800; // Shifts epoch from 1970-01-01 to 2000-01-01 (which is what the Wii uses)
    let total_seconds = tick_count as f64 / clock_rate;
    let total_nanoseconds = (total_seconds * 1_000_000_000.0) as i64;

    let duration = Duration::nanoseconds(total_nanoseconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

pub(crate) fn duration_from_ticks(tick_count: u64) -> TimeDelta {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let total_seconds = tick_count as f64 / clock_rate;
    let total_milliseconds = (total_seconds * 1_000.0) as i64;

    Duration::milliseconds(total_milliseconds)
}
