use std::{
    array::TryFromSliceError,
    io::{Read, Write},
};

use crate::{
    crc::crc32,
    ctgp_metadata::CTGPMetadata,
    header::{Header, in_game_time::InGameTime, location::constants::Country},
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
 * Implement modifying all Mii data
 * Finish writing modified header data to file
 * Handle pause times that happen during the countdown (and thus have a negative pause time)
 * Handle brake drift inputs in 200cc ghosts (bit mask 0x10)
 * Implement MKW-SP footer support
 * Implement Retro Rewind footer support
 * Implement Pulsar footer support
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
    #[error("Input Data Error: {0}")]
    InputDataError(#[from] input_data::InputDataError),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] byte_handler::ByteHandlerError),
    #[error("Try From Slice Error (CRC32): {0}")]
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
}

impl Ghost {
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, GhostError> {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)?.read_to_end(&mut buf)?;
        Self::new(&buf)
    }

    pub fn new(bytes: &[u8]) -> Result<Self, GhostError> {
        let input_data_end_offset;

        let header = Header::new(&bytes[..0x88])?;

        let file_crc32 = u32::from_be_bytes(bytes[bytes.len() - 0x04..].try_into()?);
        let base_crc32;

        let ctgp_metadata = if let Ok(ctgp_metadata) = CTGPMetadata::new(bytes) {
            input_data_end_offset = bytes.len() - ctgp_metadata.len() - 0x04;
            base_crc32 = u32::from_be_bytes(bytes[input_data_end_offset..input_data_end_offset + 0x04].try_into()?);
            Some(ctgp_metadata)
        } else {
            input_data_end_offset = bytes.len() - 0x04;
            base_crc32 = file_crc32;
            None
        };

        let input_data = InputData::new(&bytes[0x88..input_data_end_offset])?;

        Ok(Self {
            header,
            input_data,
            base_crc32,
            ctgp_metadata,
            file_crc32,
        })
    }

    pub fn save_to_bytes(&mut self) -> Result<Vec<u8>, GhostError> {
        if self.header().is_compressed() != self.input_data().is_compressed() {
            let compressed = self.input_data().is_compressed();
            self.header_mut().set_compressed(compressed);
        }

        let mut buf = Vec::from(self.header().raw_data());

        if self.header().is_modified() {
            self.write_header_data(&mut buf);
        }

        buf.extend_from_slice(self.input_data().raw_data());

        if let Some(ctgp_metadata) = self.ctgp_metadata() {
            let base_crc32 = crc32(&buf);
            buf.extend_from_slice(&base_crc32.to_be_bytes());
            buf.extend_from_slice(ctgp_metadata.raw_data());
        }

        let crc32 = crc32(&buf);
        buf.extend_from_slice(&crc32.to_be_bytes());
        Ok(buf)
    }

    pub fn save_to_file<T: AsRef<std::path::Path>>(&mut self, path: T) -> Result<(), GhostError> {
        let buf = self.save_to_bytes()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(&buf)?;

        Ok(())
    }

    fn write_header_data(&mut self, buf: &mut [u8]) {
        if !self.header().is_modified() {
            return;
        }

        Ghost::write_in_game_time(buf, 0x04, 0, self.header().finish_time());
        write_bits(buf, 0x07, 0, 6, u8::from(self.header().slot_id()) as u64);
        write_bits(
            buf,
            0x08,
            0,
            6,
            u8::from(self.header().combo().vehicle()) as u64,
        );
        write_bits(
            buf,
            0x08,
            6,
            6,
            u8::from(self.header().combo().character()) as u64,
        );
        write_bits(
            buf,
            0x09,
            4,
            7,
            (self.header().date_set().year() - 2000) as u64,
        );
        write_bits(buf, 0x0A, 3, 4, self.header().date_set().month() as u64);
        write_bits(buf, 0x0A, 7, 5, self.header().date_set().day() as u64);
        write_bits(buf, 0x0B, 4, 4, u8::from(self.header().controller()) as u64);
        write_bits(buf, 0x0C, 7, 7, u8::from(self.header().ghost_type()) as u64);
        write_bits(buf, 0x0D, 6, 1, self.header().is_automatic_drift() as u64);

        for (index, lap_split) in self.header().lap_split_times().iter().enumerate() {
            write_bits(buf, 0x11 + index * 0x03, 0, 7, lap_split.minutes() as u64);
            write_bits(buf, 0x11 + index * 0x03, 7, 7, lap_split.seconds() as u64);
            write_bits(
                buf,
                0x12 + index * 0x03,
                6,
                10,
                lap_split.milliseconds() as u64,
            );
        }

        write_bits(
            buf,
            0x34,
            0,
            8,
            u8::from(self.header().location().country()) as u64,
        );

        let subregion_id = if self.header().location().country() != Country::NotSet {
            u8::from(self.header().location().subregion()) as u64
        } else {
            0xFF
        };

        write_bits(
            buf,
            0x35,
            0,
            8,
            subregion_id,
        );

        // TODO: write mii data changes

    }

    fn write_in_game_time(
        buf: &mut [u8],
        byte_offset: usize,
        bit_offset: usize,
        in_game_time: &InGameTime,
    ) {
        let mut bit_offset = bit_offset + byte_offset * 8;

        write_bits(
            buf,
            bit_offset / 8,
            bit_offset % 8,
            7,
            in_game_time.minutes() as u64,
        );

        bit_offset += 7;

        write_bits(
            buf,
            bit_offset / 8,
            bit_offset % 8,
            7,
            in_game_time.seconds() as u64,
        );

        bit_offset += 7;

        write_bits(
            buf,
            bit_offset / 8,
            bit_offset % 8,
            10,
            in_game_time.milliseconds() as u64,
        );
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

    pub fn ctgp_metadata(&self) -> &Option<CTGPMetadata> {
        &self.ctgp_metadata
    }

    pub fn base_crc32(&self) -> u32 {
        self.base_crc32
    }

    pub fn file_crc32(&self) -> u32 {
        self.file_crc32
    }
}

pub(crate) fn write_bits(
    buf: &mut [u8],
    byte_offset: usize,
    bit_offset: usize,
    bit_width: usize,
    value: u64,
) {
    let bytes_needed = (bit_offset + bit_width + 7) / 8;
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
