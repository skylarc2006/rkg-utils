use std::{array::TryFromSliceError, io::{Read, Write}};

use crate::{
    crc::crc32, ctgp_metadata::CTGPMetadata, header::Header, input_data::InputData,
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
<<<<<<< HEAD
=======
 * Implement illegal stick position check
 * Write/save to new file, recalculate crc32
 * Be able to modify variables in ghost files
>>>>>>> 870701a2f0d80967579f8ae9ef2f9917c8dd68c0
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
    ctgp_metadata: Option<CTGPMetadata>,
    crc32: u32,
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

        let ctgp_metadata = if let Ok(ctgp_metadata) = CTGPMetadata::new(bytes) {
            input_data_end_offset = bytes.len() - ctgp_metadata.len();
            Some(ctgp_metadata)
        } else {
            input_data_end_offset = bytes.len() - 0x04;
            None
        };

        let input_data = InputData::new(&bytes[0x88..input_data_end_offset])?;

        let crc32 = u32::from_be_bytes(bytes[bytes.len() - 0x04..].try_into()?);

        Ok(Self {
            header,
            input_data,
            ctgp_metadata,
            crc32,
        })
    }

    pub fn save_to_file<T: AsRef<std::path::Path>>(&mut self, path: T) -> Result<(), GhostError> {
        self.header.set_compressed(self.input_data.is_compressed());

        let mii_data: Vec<u8> = self.header.mii().raw_data().to_vec();

        self.header_mut().raw_data_mut()[0x3C..0x3C + 0x4A]
            .copy_from_slice(&mii_data);

        self.header.fix_mii_crc16();

        let mut file = std::fs::File::create(path)?;
        file.write_all(self.header.raw_data())?;
        file.write_all(self.input_data.raw_data())?;


        if let Some(ctgp_metadata) = &self.ctgp_metadata {
            file.write_all(ctgp_metadata.raw_data())?;
        }

        // Calculate CRC32 over all data (excluding the CRC32 itself)
        let mut data_for_crc = Vec::new();
        data_for_crc.extend_from_slice(self.header.raw_data());
        data_for_crc.extend_from_slice(self.input_data.raw_data());
        if let Some(ctgp_metadata) = &self.ctgp_metadata {
            data_for_crc.extend_from_slice(ctgp_metadata.raw_data());
        }
        self.crc32 = crc32(&data_for_crc);
        file.write_all(&self.crc32.to_be_bytes())?;
        Ok(())
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

    pub fn ctgp_metadata(&self) -> &Option<CTGPMetadata> {
        &self.ctgp_metadata
    }

    pub fn crc32(&self) -> u32 {
        self.crc32
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
