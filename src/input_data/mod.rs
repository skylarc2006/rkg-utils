use crate::input_data::{
    controller_input::ControllerInput,
    dpad_button::{DPadButton, DPadButtonError},
    face_button::{FaceButton, FaceButtonError, FaceButtons},
    stick_input::{StickInput, StickInputError},
};

pub mod controller_input;
pub mod dpad_button;
pub mod face_button;
pub mod stick_input;

/// Errors that can occur while parsing [`InputData`].
#[derive(thiserror::Error, Debug)]
pub enum InputDataError {
    /// Input data is impossibly short.
    #[error("Input data length is too short")]
    InputDataLengthTooShort,
    /// Input data is malformed.
    #[error("Input data is malformed")]
    InputDataMalformed,
    /// Face Button Error.
    #[error("Face Button Error: {0}")]
    FaceButtonError(#[from] FaceButtonError),
    /// Stick Input Error.
    #[error("Stick Input Error: {0}")]
    StickInputError(#[from] StickInputError),
    /// DPad Button Error.
    #[error("Stick Input Error: {0}")]
    DPadButtonError(#[from] DPadButtonError),
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputData {
    controller_inputs: Vec<ControllerInput>,
    compressed: bool,
}

impl InputData {
    /// Parses controller input data from raw RKG bytes starting at offset `0x88`.
    ///
    /// If the bytes beginning at offset 4 carry a `Yaz1` magic header, the
    /// data is decompressed before parsing. Otherwise the slice is zero-padded
    /// to `0x2774` bytes. After parsing, adjacent identical face and stick
    /// entries are merged to represent each continuous hold as a single entry.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDataError`] variant if any individual input entry
    /// fails to parse.
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        if input_data.len() < 0x08 {
            return Err(InputDataError::InputDataLengthTooShort);
        }

        let compressed;
        let input_data = if input_data[4..8] == *b"Yaz1" {
            // YAZ1 header, decompress
            compressed = true;
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            compressed = false;
            Vec::from(input_data)
        };

        let read_u16 = |i: usize| u16::from_be_bytes(input_data[i..i + 2].try_into().unwrap());
        let face_button_count = read_u16(0x00);
        let stick_input_count = read_u16(0x02);
        let dpad_button_count = read_u16(0x04);
        // bytes 0x06-0x07: padding

        if (input_data.len() as u16)
            < ((face_button_count + stick_input_count + dpad_button_count) * 2) + 0x08
        {
            return Err(InputDataError::InputDataMalformed);
        }

        let offset = 0x08;
        let face_button_array = &input_data[offset..offset + (face_button_count as usize * 2)];

        let offset = offset + (face_button_count as usize * 2);
        let stick_input_array = &input_data[offset..offset + (stick_input_count as usize * 2)];

        let offset = offset + (stick_input_count as usize * 2);
        let dpad_button_array = &input_data[offset..offset + (dpad_button_count as usize * 2)];

        // Parse face entries, merging adjacent identical button bytes inline.
        // Each pair is (button_byte, frame_count); entries held longer than 255 frames
        // are split across consecutive pairs with the same button byte.
        let mut face_entries: Vec<(u8, u32)> = Vec::new();
        for chunk in face_button_array.chunks_exact(2) {
            let buttons_byte = chunk[0];
            let frames = chunk[1] as u32;
            if face_entries.last().map(|e| e.0) == Some(buttons_byte) {
                face_entries.last_mut().unwrap().1 += frames;
            } else {
                face_entries.push((buttons_byte, frames));
            }
        }

        // Parse stick entries with the same merge-adjacent strategy.
        let mut stick_entries: Vec<(StickInput, u32)> = Vec::new();
        for chunk in stick_input_array.chunks_exact(2) {
            let stick = StickInput::try_from(chunk[0])?;
            let frames = chunk[1] as u32;
            if stick_entries.last().map(|e| e.0) == Some(stick) {
                stick_entries.last_mut().unwrap().1 += frames;
            } else {
                stick_entries.push((stick, frames));
            }
        }

        let mut dpad_entries: Vec<(DPadButton, u16)> = Vec::new();
        for chunk in dpad_button_array.chunks_exact(2) {
            let dpad = DPadButton::try_from(chunk[0])?;
            let additional_frames: u16 = (chunk[0] & 0x0F).into();
            // 0x0F bit mask used if this input state was held longer than 255 frames, gives amount of 255-frame intervals to add to frame duration
            let frames = chunk[1] as u16 + additional_frames;
            if dpad_entries.last().map(|e| e.0) == Some(dpad) {
                dpad_entries.last_mut().unwrap().1 += frames;
            } else {
                dpad_entries.push((dpad, frames));
            }
        }

        // Walk all three timelines simultaneously. At each step take the shortest of the
        // three remaining spans, emit one ControllerInput, then advance whichever cursor(s)
        // are exhausted. This splits entries only where the timelines diverge.
        let mut controller_inputs: Vec<ControllerInput> = Vec::new();
        let mut face_idx = 0;
        let mut stick_idx = 0;
        let mut dpad_idx = 0;
        let mut face_rem = face_entries.first().map(|e| e.1).unwrap_or(0);
        let mut stick_rem = stick_entries.first().map(|e| e.1).unwrap_or(0);
        let mut dpad_rem = dpad_entries.first().map(|e| e.1 as u32).unwrap_or(0);

        while face_idx < face_entries.len()
            && stick_idx < stick_entries.len()
            && dpad_idx < dpad_entries.len()
        {
            let frames = face_rem.min(stick_rem).min(dpad_rem);
            let face_buttons = FaceButtons::try_from(face_entries[face_idx].0)?;
            let stick = stick_entries[stick_idx].0;
            let dpad = dpad_entries[dpad_idx].0;
            controller_inputs.push(
                ControllerInput::new(
                    face_buttons.0.contains(&FaceButton::Accelerator),
                    face_buttons.0.contains(&FaceButton::Brake),
                    face_buttons.0.contains(&FaceButton::BrakeDrift),
                    face_buttons.0.contains(&FaceButton::Drift),
                    face_buttons.0.contains(&FaceButton::Item),
                    face_buttons.0.contains(&FaceButton::Unknown),
                    dpad,
                    stick,
                    frames
                ));

            face_rem -= frames;
            stick_rem -= frames;
            dpad_rem -= frames;

            if face_rem == 0 {
                face_idx += 1;
                if face_idx < face_entries.len() {
                    face_rem = face_entries[face_idx].1;
                }
            }
            if stick_rem == 0 {
                stick_idx += 1;
                if stick_idx < stick_entries.len() {
                    stick_rem = stick_entries[stick_idx].1;
                }
            }
            if dpad_rem == 0 {
                dpad_idx += 1;
                if dpad_idx < dpad_entries.len() {
                    dpad_rem = dpad_entries[dpad_idx].1 as u32;
                }
            }
        }

        Ok(Self {
            controller_inputs,
            compressed,
        })
    }

    /// Returns an input state at a specified frame in the race.
    ///
    /// Returns `None` if the frame is 0 or otherwise out-of-range.
    pub fn get_input_at_frame(&self, frame: u32) -> Option<ControllerInput> {
        if frame == 0 {
            return None;
        }
        let mut elapsed = 0u32;
        for input in &self.controller_inputs {
            elapsed += input.frame_duration();
            if frame <= elapsed {
                return Some(*input);
            }
        }
        None
    }

    pub fn contains_illegal_brake_or_drift_inputs(&self) -> bool {
        for (idx, current_input) in self.controller_inputs().iter().enumerate() {
            if current_input.drift_flag()
                && !current_input.brake()
            {
                // Illegal drift input
                return true;
            } else if idx > 0 {
                let previous_input = self.controller_inputs()[idx - 1];
                if current_input.brake()
                    && current_input.accelerator()
                    && !current_input.drift_flag()
                    && previous_input.accelerator()
                    && !previous_input.brake()
                {
                    // Illegal brake input (drift flag isn't 1 when it should be)
                    return true;
                }
            }
        }
        false
    }

    pub fn raw_data(&self) -> Vec<u8> {
        // TODO: calculate this from controller_inputs
        Vec::from([0u8])
    }

    pub fn face_button_input_count(&self) -> u16 {
        // TODO: calculate this from controller inputs
        0u16
    }

    pub fn stick_input_count(&self) -> u16 {
        // TODO: calculate this from controller inputs
        0u16
    }

    pub fn dpad_button_input_count(&self) -> u16 {
        // TODO: calculate this from controller inputs
        0u16
    }

    pub fn controller_inputs(&self) -> &[ControllerInput] {
        &self.controller_inputs
    }

    pub fn controller_inputs_mut(&mut self) -> &mut [ControllerInput] {
        &mut self.controller_inputs
    }

    pub fn compressed(&self) -> bool {
        self.compressed
    }

    pub fn set_compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
}

/// Decompresses a Yaz1-encoded byte slice into raw input data.
///
/// The slice must begin with the `Yaz1` magic followed by the uncompressed
/// size as a big-endian `u32` and 8 bytes of padding. The result is
/// zero-padded to `0x2774` bytes.
///
/// Returns `None` if the magic is missing, the data is truncated, or the
/// decompressed output does not match the expected size.
///
/// Adapted from <https://github.com/AtishaRibeiro/InputDisplay/blob/master/InputDisplay/Core/Yaz1dec.cs>.
pub(crate) fn yaz1_decompress(data: &[u8]) -> Option<Vec<u8>> {
    // YAZ1 files start with "Yaz1" magic header
    if data.len() < 16 || &data[0..4] != b"Yaz1" {
        return None;
    }

    let uncompressed_size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;

    let mut result = Vec::with_capacity(uncompressed_size);

    let decompressed = decompress_block(
        data,
        16, // Start after 16-byte header
        uncompressed_size,
    );

    if let Some(mut dec) = decompressed {
        result.append(&mut dec);
    }

    if result.len() == uncompressed_size {
        result.resize(0x2774, 0);
        Some(result)
    } else {
        None
    }
}

/// Compresses a byte slice using the Yaz1 encoding scheme.
///
/// Writes a 16-byte Yaz1 header (magic, uncompressed size, 8 bytes of padding),
/// followed by the compressed payload. The result is padded so its length is a
/// multiple of 4 (matching the original JS behavior: adds `len % 4` zero bytes).
pub(crate) fn yaz1_compress(src: &[u8]) -> Vec<u8> {
    // first remove padded 0s (decompressed input data is padded with 0s to 0x2774 bytes)
    let mut trailing_bytes_to_remove = 0usize;
    for idx in (0..src.len()).rev() {
        if src[idx] == 0 {
            trailing_bytes_to_remove += 1;
        } else {
            break;
        }
    }

    let src = &src[0..src.len() - trailing_bytes_to_remove];


    let mut dst = encode_yaz1(src);

    let remainder = dst.len() % 4;
    dst.resize(dst.len() + remainder, 0);

    let mut compressed_data = Vec::new();
    compressed_data.extend_from_slice(&((dst.len() + 8) as u32).to_be_bytes()); // size of compressed data
    compressed_data.extend_from_slice(b"Yaz1");
    compressed_data.extend_from_slice(&(src.len() as u32).to_be_bytes());
    compressed_data.extend_from_slice(&[0u8; 8]); // padding
    compressed_data.extend_from_slice(&dst);
    compressed_data
}

fn encode_yaz1(src: &[u8]) -> Vec<u8> {
    let size = src.len();
    let mut state = NintendoEncState::new();
    let mut dst_buf = [0u8; 24]; // 8 codes × 3 bytes maximum
    let mut dst_pos = 0usize;
    let mut src_pos = 0usize;
    let mut valid_bit_count = 0u32;
    let mut curr_code_byte = 0u8;
    let mut output: Vec<u8> = Vec::new();

    while src_pos < size {
        let (num_bytes, match_pos) = state.encode(src, size, src_pos);

        if num_bytes < 3 {
            dst_buf[dst_pos] = src[src_pos];
            dst_pos += 1;
            src_pos += 1;
            curr_code_byte |= 0x80u8 >> valid_bit_count;
        } else {
            let dist = src_pos - match_pos - 1;
            let mut num_bytes = num_bytes;

            if num_bytes >= 0x12 {
                dst_buf[dst_pos] = (dist >> 8) as u8;
                dst_pos += 1;
                dst_buf[dst_pos] = (dist & 0xff) as u8;
                dst_pos += 1;
                if num_bytes > 0xff + 0x12 {
                    num_bytes = 0xff + 0x12;
                }
                dst_buf[dst_pos] = (num_bytes - 0x12) as u8;
                dst_pos += 1;
            } else {
                dst_buf[dst_pos] = (((num_bytes - 2) << 4) | (dist >> 8)) as u8;
                dst_pos += 1;
                dst_buf[dst_pos] = (dist & 0xff) as u8;
                dst_pos += 1;
            }
            src_pos += num_bytes;
        }

        valid_bit_count += 1;
        if valid_bit_count == 8 {
            output.push(curr_code_byte);
            output.extend_from_slice(&dst_buf[..dst_pos]);
            curr_code_byte = 0;
            valid_bit_count = 0;
            dst_pos = 0;
        }
    }

    if valid_bit_count > 0 {
        output.push(curr_code_byte);
        output.extend_from_slice(&dst_buf[..dst_pos]);
    }

    output
}

struct NintendoEncState {
    prev_flag: bool,
    stored_match_pos: usize,
    stored_num_bytes: usize,
}

impl NintendoEncState {
    fn new() -> Self {
        Self { prev_flag: false, stored_match_pos: 0, stored_num_bytes: 0 }
    }

    fn encode(&mut self, src: &[u8], size: usize, pos: usize) -> (usize, usize) {
        if self.prev_flag {
            self.prev_flag = false;
            return (self.stored_num_bytes, self.stored_match_pos);
        }

        let (mut num_bytes, match_pos) = simple_enc(src, size, pos);

        if num_bytes >= 3 {
            let (num_bytes1, match_pos1) = simple_enc(src, size, pos + 1);
            self.stored_num_bytes = num_bytes1;
            self.stored_match_pos = match_pos1;
            if num_bytes1 >= num_bytes + 2 {
                num_bytes = 1;
                self.prev_flag = true;
            }
        }

        (num_bytes, match_pos)
    }
}

fn simple_enc(src: &[u8], size: usize, pos: usize) -> (usize, usize) {
    let start_pos = pos.saturating_sub(0x1000);
    let max_match = size - pos;
    let mut num_bytes = 1usize;
    let mut match_pos = 0usize;

    for i in start_pos..pos {
        let mut j = 0usize;
        while j < max_match && src[i + j] == src[j + pos] {
            j += 1;
        }
        if j > num_bytes {
            num_bytes = j;
            match_pos = i;
        }
    }

    (num_bytes, match_pos)
}

/// Decompresses a single Yaz1 block starting at `offset` within `src`.
///
/// Returns `None` if the source data is truncated mid-block. The output is
/// exactly `uncompressed_size` bytes when successful.
fn decompress_block(src: &[u8], offset: usize, uncompressed_size: usize) -> Option<Vec<u8>> {
    let mut dst = Vec::with_capacity(uncompressed_size);
    let mut src_pos = offset;

    let mut valid_bit_count = 0; // number of valid bits left in "code" byte
    let mut curr_code_byte = 0u8;

    while dst.len() < uncompressed_size {
        // Read new "code" byte if the current one is used up
        if valid_bit_count == 0 {
            if src_pos >= src.len() {
                return None;
            }
            curr_code_byte = src[src_pos];
            src_pos += 1;
            valid_bit_count = 8;
        }

        if (curr_code_byte & 0x80) != 0 {
            // Straight copy
            if src_pos >= src.len() {
                return None;
            }
            dst.push(src[src_pos]);
            src_pos += 1;
        } else {
            // RLE part
            if src_pos + 1 >= src.len() {
                return None;
            }

            let byte1 = src[src_pos];
            src_pos += 1;
            let byte2 = src[src_pos];
            src_pos += 1;

            let dist = (((byte1 & 0xF) as usize) << 8) | (byte2 as usize);
            let copy_source = dst.len().wrapping_sub(dist + 1);

            let mut num_bytes = (byte1 >> 4) as usize;
            if num_bytes == 0 {
                if src_pos >= src.len() {
                    return None;
                }
                num_bytes = src[src_pos] as usize + 0x12;
                src_pos += 1;
            } else {
                num_bytes += 2;
            }

            // Copy run - must handle overlapping copies
            for i in 0..num_bytes {
                if copy_source + i >= dst.len() {
                    return None;
                }
                let byte = dst[copy_source + i];
                dst.push(byte);
            }
        }

        // Use next bit from "code" byte
        curr_code_byte <<= 1;
        valid_bit_count -= 1;
    }

    Some(dst)
}
