/// Calculate CRC16 using CCITT XModem variant
///
/// # Arguments
/// * `data` - The data slice to calculate CRC16 for
///
/// # Returns
/// The calculated CRC16 value
pub fn crc16(data: &[u8]) -> u16 {
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
