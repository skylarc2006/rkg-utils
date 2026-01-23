use crate::{
    write_bits,
    byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler},
    header::{
        combo::{Combo, ComboError},
        controller::{Controller, ControllerError},
        date::{Date, DateError},
        ghost_type::{GhostType, GhostTypeError},
        in_game_time::{InGameTime, InGameTimeError},
        location::Location,
        mii::{Mii, MiiError},
        slot_id::{SlotId, SlotIdError},
    },
};

use std::io::Read;

pub mod combo;
pub mod controller;
pub mod date;
pub mod ghost_type;
pub mod in_game_time;
pub mod location;
pub mod mii;
pub mod slot_id;

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("File is not RKGD")]
    NotRKGD,
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    #[error("Slot ID Error: {0}")]
    SlotIdError(#[from] SlotIdError),
    #[error("Combo Error: {0}")]
    ComboError(#[from] ComboError),
    #[error("Date Error: {0}")]
    DateError(#[from] DateError),
    #[error("Controller Error: {0}")]
    ControllerError(#[from] ControllerError),
    #[error("Ghost Type Error: {0}")]
    GhostTypeError(#[from] GhostTypeError),
    #[error("Mii Error: {0}")]
    MiiError(#[from] MiiError),
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// All the data in the Header of an RKGD
/// https://wiki.tockdom.com/wiki/RKG_(File_Format)#File_Header
pub struct Header {
    raw_data: [u8; 0x88],
    finish_time: InGameTime,
    slot_id: SlotId,
    combo: Combo,
    date_set: Date,
    controller: Controller,
    is_compressed: bool,
    ghost_type: GhostType,
    is_automatic_drift: bool,
    decompressed_input_data_length: u16,
    lap_count: u8,
    lap_split_times: [InGameTime; 10],
    location: Location,
    mii: Mii,
    mii_crc16: u16,
}

impl Header {
    /// Reads header from a file at the path
    pub fn new_from_path<P: AsRef<std::path::Path>>(p: P) -> Result<Self, HeaderError> {
        let mut rkg_data = [0u8; 0x88];
        std::fs::File::open(p)?.read_exact(&mut rkg_data)?;
        Self::new(&rkg_data)
    }

    /// Reads header from slice
    pub fn new(header_data: &[u8]) -> Result<Self, HeaderError> {
        if header_data.len() != 0x88 {
            return Err(HeaderError::NotCorrectSize);
        }
        if header_data[0..4] != [0x52, 0x4B, 0x47, 0x44] {
            return Err(HeaderError::NotRKGD);
        }

        let finish_time = InGameTime::from_byte_handler(&header_data[4..7])?;
        let slot_id = SlotId::from_byte_handler(header_data[7])?;
        let combo = Combo::from_byte_handler(&header_data[0x08..0x0A])?;
        let date_set = Date::from_byte_handler(&header_data[0x09..=0x0B])?;
        let controller = Controller::from_byte_handler(header_data[0x0B])?;
        let is_compressed = ByteHandler::from(header_data[0x0C]).read_bool(3);
        let ghost_type = GhostType::from_byte_handler(&header_data[0x0C..=0x0D])?;
        let is_automatic_drift = ByteHandler::from(header_data[0x0D]).read_bool(1);
        let decompressed_input_data_length =
            ByteHandler::try_from(&header_data[0x0E..=0x0F])?.copy_word(0);

        let lap_count = header_data[0x10];
        let mut lap_split_times: [InGameTime; 10] = [Default::default(); 10];
        for index in 0..10 {
            let start = (0x11 + index * 3) as usize;
            lap_split_times[index as usize] =
                InGameTime::from_byte_handler(&header_data[start..start + 3])?;
        }

        let codes = ByteHandler::try_from(&header_data[0x34..=0x37]).unwrap();
        let location =
            Location::find(codes.copy_byte(0), codes.copy_byte(1), None).unwrap_or_default();

        
        let mii = Mii::new(&header_data[0x3C..0x3C + 0x4A])?;

        let mii_crc16 = ByteHandler::try_from(&header_data[0x86..=0x87])?.copy_word(0);

        Ok(Self {
            raw_data: header_data.try_into().unwrap(),
            finish_time,
            slot_id,
            combo,
            date_set,
            controller,
            is_compressed,
            ghost_type,
            is_automatic_drift,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            location,
            mii,
            mii_crc16,
        })
    }

    /// Returns true if Mii CRC16 is correct (i.e. Mii data not illegally tampered with)
    pub fn verify_mii_crc16(&self) -> bool {
        // Verify CRC16 based on the actual bytes in the header buffer (0x3C to 0x86)
        crc16(&self.raw_data[0x3C..0x86]) == self.mii_crc16()
    }

    /// Recalculates and updates Mii CRC16
    pub fn fix_mii_crc16(&mut self) {
        // Calculate CRC16 based on the actual bytes in the header buffer (0x3C to 0x86)
        // This ensures the CRC matches what's actually written to the file
        self.mii_crc16 = crc16(&self.raw_data[0x3C..0x86]);
        self.raw_data[0x86..0x88].copy_from_slice(&self.mii_crc16.to_be_bytes());
    }

    pub fn raw_data(&self) -> &[u8; 0x88] {
        &self.raw_data
    }

    pub fn raw_data_mut(&mut self) -> &mut [u8; 0x88] {
        &mut self.raw_data
    }

    pub fn finish_time(&self) -> &InGameTime {
        &self.finish_time
    }

    pub fn set_finish_time(&mut self, finish_time: InGameTime) {
        self.finish_time = finish_time;
        write_bits(&mut self.raw_data, 0x04, 0, 7, finish_time.minutes() as u64);
        write_bits(&mut self.raw_data, 0x04, 7, 7, finish_time.seconds() as u64);
        write_bits(&mut self.raw_data, 0x05, 6, 10, finish_time.milliseconds() as u64);
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    pub fn set_slot_id(&mut self, slot_id: SlotId) {
        self.slot_id = slot_id;
        write_bits(&mut self.raw_data, 0x07, 0, 6, slot_id as u64);
    }

    pub fn combo(&self) -> &Combo {
        &self.combo
    }

    pub fn set_combo(&mut self, combo: Combo) {
        self.combo = combo;
        write_bits(&mut self.raw_data, 0x08, 0, 5, self.combo.vehicle() as u64);
        write_bits(&mut self.raw_data, 0x08, 5, 6, self.combo.character() as u64);
    }

    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

    pub fn set_date_set(&mut self, date_set: Date) {
        self.date_set = date_set;
        write_bits(&mut self.raw_data, 0x09, 4, 7, (self.date_set.year() - 2000) as u64);
        write_bits(&mut self.raw_data, 0x0A, 3, 4, self.date_set.month() as u64);
        write_bits(&mut self.raw_data, 0x0A, 7, 5, self.date_set.day() as u64);
    }

    pub fn controller(&self) -> Controller {
        self.controller
    }

    pub fn set_controller(&mut self, controller: Controller) {
        self.controller = controller;
        write_bits(&mut self.raw_data, 0x0B, 4, 4, self.controller as u64);
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub(crate) fn set_compressed(&mut self, is_compressed: bool) {
        self.is_compressed = is_compressed;
        write_bits(&mut self.raw_data, 0x0C, 4, 1, is_compressed as u64);
    }

    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    pub fn set_ghost_type(&mut self, ghost_type: GhostType) {
        self.ghost_type = ghost_type;
        write_bits(&mut self.raw_data, 0x0C, 7, 7, u8::from(self.ghost_type) as u64);
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[InGameTime] {
        &self.lap_split_times[0..self.lap_count as usize]
    }

    pub fn set_lap_split_time(&mut self, index: usize, lap_split_time: InGameTime) {
        if index >= self.lap_count as usize {
            return;
        }

        self.lap_split_times[index] = lap_split_time;
        write_bits(&mut self.raw_data, 0x11 + index * 3, 0, 7, lap_split_time.minutes() as u64);
        write_bits(&mut self.raw_data, 0x11 + index * 3, 7, 7, lap_split_time.seconds() as u64);
        write_bits(&mut self.raw_data, 0x12 + index * 3, 6, 10, lap_split_time.milliseconds() as u64);
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
        write_bits(&mut self.raw_data, 0x34, 0, 8, u8::from(self.location.country()) as u64);
        write_bits(&mut self.raw_data, 0x35, 0, 8, u8::from(self.location.subregion()) as u64);
    }

    pub fn mii(&self) -> &Mii {
        &self.mii
    }

    pub fn mii_mut(&mut self) -> &mut Mii {
        &mut self.mii
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}

fn crc16(value: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000; // Initial value for XModem variant
    let polynomial: u16 = 0x1021; // Standard CCITT polynomial

    for &byte in value.iter() {
        crc ^= (byte as u16) << 8; // XOR current byte with the high byte of CRC

        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ polynomial;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
