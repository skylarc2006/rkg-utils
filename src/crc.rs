use std::io::{Read, Seek, SeekFrom, Write};

/// Calculate CRC16 using CCITT XModem variant
/// 
/// # Arguments
/// * `data` - The data slice to calculate CRC16 for
/// 
/// # Returns
/// The calculated CRC16 value
pub fn crc16_ccitt_xmodem(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000; // Initial value for XModem variant
    let polynomial: u16 = 0x1021; // Standard CCITT polynomial

    for &byte in data {
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

/// Update CRC16 in a file
/// 
/// Reads bytes from offset 0x3C to 0x86 (exclusive), calculates CRC16,
/// and writes it at offset 0x86 in big-endian format.
/// 
/// # Arguments
/// * `file` - A mutable reference to a file that implements Read, Write, and Seek
/// 
/// # Errors
/// Returns an IO error if the file operations fail or if the file is too small
pub fn update_crc16<T: Read + Write + Seek>(file: &mut T) -> std::io::Result<()> {
    const CRC16_START: u64 = 0x3C;
    const CRC16_END: u64 = 0x86; // not inclusive
    const CRC16_WRITE: u64 = 0x86;

    // Determine file size
    let file_size = file.seek(SeekFrom::End(0))?;

    if file_size < CRC16_WRITE + 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File too small for CRC16 update",
        ));
    }

    // Read bytes [0x3C, 0x86)
    let length = (CRC16_END - CRC16_START) as usize;
    let mut buffer = vec![0u8; length];

    file.seek(SeekFrom::Start(CRC16_START))?;
    file.read_exact(&mut buffer)?;

    // Compute CRC16
    let crc16 = crc16_ccitt_xmodem(&buffer);

    // Write CRC16 at offset 0x86 (big-endian)
    file.seek(SeekFrom::Start(CRC16_WRITE))?;
    let crc_bytes = crc16.to_be_bytes();
    file.write_all(&crc_bytes)?;
    file.flush()?;

    Ok(())
}

/// Generate CRC32 lookup table
fn make_crc_table() -> [u32; 256] {
    let mut crc_table = [0u32; 256];
    for n in 0..256 {
        let mut c = n;
        for _ in 0..8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        crc_table[n as usize] = c;
    }
    crc_table
}

/// Calculate CRC32 for a byte slice
/// 
/// This is the standard CRC32 algorithm used in many file formats.
/// 
/// # Arguments
/// * `data` - The data slice to calculate CRC32 for
/// 
/// # Returns
/// The calculated CRC32 value
pub fn crc32(data: &[u8]) -> u32 {
    static CRC_TABLE: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    let crc_table = CRC_TABLE.get_or_init(make_crc_table);
    
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in data {
        crc = (crc >> 8) ^ crc_table[((crc ^ byte as u32) & 0xFF) as usize];
    }

    crc ^ 0xFFFFFFFF
}

/// Calculate CRC32 from a file, excluding the last 4 bytes
/// 
/// This function reads the entire file except for the last 4 bytes
/// and calculates the CRC32 checksum.
/// 
/// # Arguments
/// * `file` - A mutable reference to a file that implements Read and Seek
/// 
/// # Errors
/// Returns an IO error if the file operations fail or if the file is too small
pub fn crc32_from_file<T: Read + Seek>(file: &mut T) -> std::io::Result<u32> {
    static CRC_TABLE: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    let crc_table = CRC_TABLE.get_or_init(make_crc_table);
    
    let mut crc: u32 = 0xFFFFFFFF;

    // Remember current position
    let file_size = file.seek(SeekFrom::End(0))?;

    if file_size < 4 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File too small to contain a CRC32!",
        ));
    }

    // Go back to start
    file.seek(SeekFrom::Start(0))?;

    const BUFFER_SIZE: usize = 4096;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    let mut bytes_to_read = file_size - 4; // exclude last 4 bytes
    
    while bytes_to_read > 0 {
        let chunk_size = bytes_to_read.min(BUFFER_SIZE as u64) as usize;
        let read_count = file.read(&mut buffer[..chunk_size])?;

        for i in 0..read_count {
            crc = (crc >> 8) ^ crc_table[((crc ^ buffer[i] as u32) & 0xFF) as usize];
        }

        bytes_to_read -= read_count as u64;
    }

    Ok(crc ^ 0xFFFFFFFF)
}
