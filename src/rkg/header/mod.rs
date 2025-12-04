use bitreader::BitReader;

use crate::rkg::header::{finish_time::FinishTime, mii::Mii};

pub mod finish_time;
pub mod mii;

pub struct Header {
    rkgd: String,                        // 0x04, offset 0x00
    finish_time: FinishTime,             // 0x03, offset 0x04
    track_id: u8,                        // 6 bits, offset 0x07
    unknown1: u8,                        // 2 bits, offset 0x07.6, likely padding
    vehicle_id: u8,                      // 6 bits, offset 0x08
    character_id: u8,                    // 6 bits, offset 0x08.6
    year_set: u8,                        // 7 bits, offset 0x09.4
    month_set: u8,                       // 4 bits, offset 0x0A.3
    day_set: u8,                         // 5 bits, offset 0x0A.7
    controller_id: u8,                   // 4 bits, offset 0x0B.4
    unknown2: u8,                        // 4 bits, offset 0x0C, always 0?
    is_compressed: bool,                 // 1 bit, offset 0xC.4
    unknown3: u8,                        // 2 bits, offset 0x0C.5, always 0?
    ghost_type: u8,                      // 7 bits, offset 0x0C.7
    is_automatic_drift: bool,            // 1 bit, offset 0x0D.6
    unknown4: bool,                      // 1 bit, offset 0x0D.7, likely padding
    decompressed_input_data_length: u16, // 0x02, offset 0x0E
    lap_count: u8,                       // 0x01, offset 0x10
    lap_split_times: Vec<FinishTime>,    // 0x0F, offset 0x11, first 5 laps
    // 0x14, offset 0x20, vanilla game attempts to store laps greater than 5 but fails.
    country_code: u8,   // 0x01, offset 0x34
    state_code: u8,     // 0x01, offset 0x35
    location_code: u16, // 0x02, offset 0x36
    unknown6: u32,      // 0x04, offset 0x38, typically 0
    mii_data: Mii,      // 0x4A, offset 0x3C
    mii_crc16: u16,     // 0x02, offset 0x86
}

impl Header {
    pub fn new(rkg_data: &[u8]) -> Self {
        // Get RKGD magic
        let mut rkg_reader: BitReader<'_> = BitReader::new(&rkg_data);
        let rkgd_bytes: [u8; _] = rkg_reader
            .read_u32(32)
            .expect("Failed to read rkgd")
            .to_be_bytes();
        let mut rkgd: String = String::new();

        // Convert the byte array to a String
        match String::from_utf8(rkgd_bytes.to_vec()) {
            Ok(s) => {
                rkgd = s;
            }
            Err(e) => {
                eprintln!("Failed to convert bytes to UTF-8 string: {}", e);
            }
        }

        // Get finish time
        let m: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read minutes of finish time")
            .to_be();
        let s: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read seconds of finish time")
            .to_be();
        let ms: u16 = rkg_reader
            .read_u16(10)
            .expect("Failed to read milliseconds of finish time"); // bitreader already interprets multi-byte values as big endian
        let finish_time: FinishTime = FinishTime::new(m, s, ms);

        // TODO: everything from this point on
        let track_id: u8 = 0;
        let unknown1: u8 = 0;
        let vehicle_id: u8 = 0;
        let character_id: u8 = 0;
        let year_set: u8 = 0;
        let month_set: u8 = 0;
        let day_set: u8 = 0;
        let controller_id: u8 = 0;
        let unknown2: u8 = 0;
        let is_compressed: bool = false;
        let unknown3: u8 = 0;
        let ghost_type: u8 = 0;
        let is_automatic_drift: bool = false;
        let unknown4: bool = false;
        let decompressed_input_data_length: u16 = 0;
        let lap_count: u8 = 0;
        let lap_split_times: Vec<FinishTime> = Vec::new();
        let country_code: u8 = 0;
        let state_code: u8 = 0;
        let location_code: u16 = 0;
        let unknown6: u32 = 0;
        let mii_data: Mii = Mii::new(&rkg_data[0x3C..0x86]);
        let mii_crc16: u16 = 0;

        Self {
            rkgd,
            finish_time,
            track_id,
            unknown1,
            vehicle_id,
            character_id,
            year_set,
            month_set,
            day_set,
            controller_id,
            unknown2,
            is_compressed,
            unknown3,
            ghost_type,
            is_automatic_drift,
            unknown4,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            country_code,
            state_code,
            location_code,
            unknown6,
            mii_data,
            mii_crc16,
        }
    }

    pub fn rkgd(&self) -> &str {
        &self.rkgd
    }

    pub fn finish_time(&self) -> &FinishTime {
        &self.finish_time
    }

    pub fn track_id(&self) -> u8 {
        self.track_id
    }

    pub fn unknown1(&self) -> u8 {
        self.unknown1
    }

    pub fn vehicle_id(&self) -> u8 {
        self.vehicle_id
    }

    pub fn character_id(&self) -> u8 {
        self.character_id
    }

    pub fn year_set(&self) -> u8 {
        self.year_set
    }

    pub fn month_set(&self) -> u8 {
        self.month_set
    }

    pub fn day_set(&self) -> u8 {
        self.day_set
    }

    pub fn controller_id(&self) -> u8 {
        self.controller_id
    }

    pub fn unknown2(&self) -> u8 {
        self.unknown2
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn unknown3(&self) -> u8 {
        self.unknown3
    }

    pub fn ghost_type(&self) -> u8 {
        self.ghost_type
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn unknown4(&self) -> bool {
        self.unknown4
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[FinishTime] {
        &self.lap_split_times
    }

    pub fn country_code(&self) -> u8 {
        self.country_code
    }

    pub fn state_code(&self) -> u8 {
        self.state_code
    }

    pub fn location_code(&self) -> u16 {
        self.location_code
    }

    pub fn unknown6(&self) -> u32 {
        self.unknown6
    }

    pub fn mii_data(&self) -> &Mii {
        &self.mii_data
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}
