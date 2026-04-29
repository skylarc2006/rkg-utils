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
    #[error("DPad Button Error: {0}")]
    DPadButtonError(#[from] DPadButtonError),
}

/// Decoded controller input sequence for a single race, stored as a list of
/// [`ControllerInput`] runs where each run represents a contiguous span of
/// identical inputs lasting `frame_duration` frames.
#[derive(Debug, Clone, PartialEq)]
pub struct InputData {
    controller_inputs: Vec<ControllerInput>,
    compressed: bool,
}

impl InputData {
    /// Constructs input data from a `Vec<ControllerInput>` and a compressed flag.
    ///
    /// Returns [`InputDataError::InputDataLengthTooShort`] if `controller_inputs` is empty.
    pub fn new(
        controller_inputs: Vec<ControllerInput>,
        compressed: bool,
    ) -> Result<Self, InputDataError> {
        if controller_inputs.is_empty() {
            return Err(InputDataError::InputDataLengthTooShort);
        }
        Ok(Self {
            controller_inputs,
            compressed,
        })
    }

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
    pub fn new_from_bytes(input_data: &[u8]) -> Result<Self, InputDataError> {
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
        let face_button_input_count = read_u16(0x00);
        let stick_input_count = read_u16(0x02);
        let dpad_button_input_count = read_u16(0x04);
        // bytes 0x06-0x07: padding

        if (input_data.len() as u16)
            < ((face_button_input_count + stick_input_count + dpad_button_input_count) * 2) + 0x08
        {
            return Err(InputDataError::InputDataMalformed);
        }

        let offset = 0x08;
        let face_button_array =
            &input_data[offset..offset + (face_button_input_count as usize * 2)];

        let offset = offset + (face_button_input_count as usize * 2);
        let stick_input_array = &input_data[offset..offset + (stick_input_count as usize * 2)];

        let offset = offset + (stick_input_count as usize * 2);
        let dpad_button_array =
            &input_data[offset..offset + (dpad_button_input_count as usize * 2)];

        let mut face_button_inputs: Vec<(FaceButtons, u32)> =
            Vec::with_capacity(face_button_input_count as usize);
        for chunk in face_button_array.chunks_exact(2).into_iter() {
            let face_buttons = FaceButtons::try_from(chunk[0])?;
            let frame_duration = chunk[1] as u32;
            face_button_inputs.push((face_buttons, frame_duration));
        }

        let mut stick_inputs: Vec<(StickInput, u32)> =
            Vec::with_capacity(stick_input_count as usize);
        for chunk in stick_input_array.chunks_exact(2).into_iter() {
            let stick_input = StickInput::try_from(chunk[0])?;
            let frame_duration = chunk[1] as u32;
            stick_inputs.push((stick_input, frame_duration));
        }

        let mut dpad_button_inputs: Vec<(DPadButton, u32)> =
            Vec::with_capacity(dpad_button_input_count as usize);
        for chunk in dpad_button_array.chunks_exact(2).into_iter() {
            let dpad_button = DPadButton::try_from(chunk[0])?;
            let frame_duration = (u16::from_be_bytes([chunk[0], chunk[1]]) & 0xFFF) as u32;
            dpad_button_inputs.push((dpad_button, frame_duration));
        }

        // Combine adjacent inputs when the same button is held across multiple bytes
        for idx in (0..face_button_inputs.len() - 1).rev() {
            if face_button_inputs[idx].0 == face_button_inputs[idx + 1].0 {
                let f1 = face_button_inputs[idx].1;
                let f2 = face_button_inputs[idx + 1].1;
                face_button_inputs[idx].1 = f1 + f2;
                face_button_inputs.remove(idx + 1);
            }
        }

        for idx in (0..stick_inputs.len() - 1).rev() {
            if stick_inputs[idx].0 == stick_inputs[idx + 1].0 {
                let f1 = stick_inputs[idx].1;
                let f2 = stick_inputs[idx + 1].1;
                stick_inputs[idx].1 = f1 + f2;
                stick_inputs.remove(idx + 1);
            }
        }

        for idx in (0..dpad_button_inputs.len() - 1).rev() {
            if dpad_button_inputs[idx].0 == dpad_button_inputs[idx + 1].0 {
                let f1 = dpad_button_inputs[idx].1;
                let f2 = dpad_button_inputs[idx + 1].1;
                dpad_button_inputs[idx].1 = f1 + f2;
                dpad_button_inputs.remove(idx + 1);
            }
        }

        let mut controller_inputs = Vec::new();

        // Track current position in each input stream
        let mut face_idx = 0;
        let mut stick_idx = 0;
        let mut dpad_idx = 0;

        // Track how many frames consumed from current input in each stream
        let mut face_offset = 0u32;
        let mut stick_offset = 0u32;
        let mut dpad_offset = 0u32;

        // Continue until all streams are exhausted
        while face_idx < face_button_inputs.len()
            || stick_idx < stick_inputs.len()
            || dpad_idx < dpad_button_inputs.len()
        {
            // Get current input from each stream (or defaults if exhausted)
            let face = face_button_inputs.get(face_idx);
            let stick = stick_inputs.get(stick_idx);
            let dpad = dpad_button_inputs.get(dpad_idx);

            // Calculate remaining frames for current input in each stream
            let face_remaining = face.map(|f| f.1 - face_offset).unwrap_or(u32::MAX);
            let stick_remaining = stick.map(|s| s.1 - stick_offset).unwrap_or(u32::MAX);
            let dpad_remaining = dpad.map(|d| d.1 - dpad_offset).unwrap_or(u32::MAX);

            // Find the minimum remaining frames (when next change occurs)
            let duration = face_remaining.min(stick_remaining).min(dpad_remaining);

            if duration == u32::MAX {
                // if all streams exhausted
                break;
            }

            let FaceButtons(face_buttons) = face.map(|f| f.0.clone()).unwrap_or_default();

            // Create combined input for this duration
            let combined = ControllerInput::new(
                face_buttons.contains(&FaceButton::Accelerator),
                face_buttons.contains(&FaceButton::Brake),
                face_buttons.contains(&FaceButton::BrakeDrift),
                face_buttons.contains(&FaceButton::Drift),
                face_buttons.contains(&FaceButton::Item),
                face_buttons.contains(&FaceButton::Unknown),
                dpad.map(|d| d.0).unwrap_or(DPadButton::None),
                stick.map(|s| s.0).unwrap_or_default(),
                duration,
            );

            controller_inputs.push(combined);

            // Update offsets and advance indices where needed
            face_offset += duration;
            stick_offset += duration;
            dpad_offset += duration;

            if let Some(face) = face
                && face_offset >= face.1
            {
                face_idx += 1;
                face_offset = 0;
            }
            if let Some(stick) = stick
                && stick_offset >= stick.1
            {
                stick_idx += 1;
                stick_offset = 0;
            }
            if let Some(dpad) = dpad
                && dpad_offset >= dpad.1
            {
                dpad_idx += 1;
                dpad_offset = 0;
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

    /// Returns `true` if any input in the sequence is illegal under normal race conditions.
    ///
    /// Two conditions are checked: a drift flag set without the brake button held,
    /// and a brake + accelerator combination where the drift flag is absent despite
    /// the previous frame having accelerator but no brake (indicating the game should
    /// have set the drift flag automatically).
    pub fn contains_illegal_brake_or_drift_inputs(&self) -> bool {
        for (idx, current_input) in self.controller_inputs().iter().enumerate() {
            if current_input.drift_flag() && !current_input.brake() {
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

    /// Serializes the input data back to the binary format used in an RKG file.
    ///
    /// Produces a 6-byte header (face, stick, and dpad entry counts as big-endian
    /// `u16`s, followed by 2 padding bytes), then the face button, stick, and dpad
    /// arrays in sequence. Each run longer than the per-stream maximum (255 frames
    /// for face/stick, 4095 for dpad) is split into multiple entries. If
    /// `compressed` is set the result is Yaz1-compressed; otherwise it is
    /// zero-padded to `0x2774` bytes.
    pub fn raw_data(&self) -> Vec<u8> {
        let mut raw_data = Vec::new();

        // Input data header
        raw_data.extend_from_slice(&self.face_button_input_count().to_be_bytes());
        raw_data.extend_from_slice(&self.stick_input_count().to_be_bytes());
        raw_data.extend_from_slice(&self.dpad_button_input_count().to_be_bytes());
        raw_data.extend_from_slice(&0u16.to_be_bytes());

        // Face button array
        // Derive vector of (FaceButtons, u32 [frames]) from controller inputs
        let controller_inputs = &self.controller_inputs;
        let mut face_button_inputs: Vec<(FaceButtons, u32)> = Vec::new();
        for (idx, input) in controller_inputs.iter().enumerate() {
            if idx > 0 && input.face_buttons_equal_to(controller_inputs[idx - 1]) {
                face_button_inputs.last_mut().unwrap().1 += input.frame_duration();
            } else {
                face_button_inputs.push((to_face_buttons(input), input.frame_duration()));
            }
        }
        for (face_buttons, frames) in &face_button_inputs {
            let button_byte = face_buttons.to_byte();
            for _ in 0..(*frames / 255) {
                raw_data.push(button_byte);
                raw_data.push(255);
            }
            raw_data.push(button_byte);
            raw_data.push((*frames % 255) as u8);
        }

        // Stick input array
        // Derive vector of (StickInput, u32 [frames]) from controller inputs
        let mut stick_inputs: Vec<(StickInput, u32)> = Vec::new();
        for (idx, input) in controller_inputs.iter().enumerate() {
            if idx > 0 && input.stick() == controller_inputs[idx - 1].stick() {
                stick_inputs.last_mut().unwrap().1 += input.frame_duration();
            } else {
                stick_inputs.push((input.stick(), input.frame_duration()));
            }
        }
        for (stick_input, frames) in &stick_inputs {
            let stick_byte = stick_input.to_byte();
            for _ in 0..(*frames / 255) {
                raw_data.push(stick_byte);
                raw_data.push(255);
            }
            raw_data.push(stick_byte);
            raw_data.push((*frames % 255) as u8);
        }

        // DPad input array
        // Derive vector of (DPadButton, u32 [frames]) from controller inputs
        let mut dpad_button_inputs: Vec<(DPadButton, u32)> = Vec::new();
        for (idx, input) in controller_inputs.iter().enumerate() {
            if idx > 0 && input.dpad() == controller_inputs[idx - 1].dpad() {
                dpad_button_inputs.last_mut().unwrap().1 += input.frame_duration();
            } else {
                dpad_button_inputs.push((input.dpad(), input.frame_duration()));
            }
        }
        for (dpad_button, frames) in &dpad_button_inputs {
            let nibble = dpad_button.to_nibble() as u16;
            for _ in 0..(*frames / 4095) {
                let word = (nibble << 12) | 0xFFF;
                raw_data.extend_from_slice(&word.to_be_bytes());
            }
            let word = (nibble << 12) | (*frames % 4095) as u16;
            raw_data.extend_from_slice(&word.to_be_bytes());
        }

        if self.compressed() {
            yaz1_compress(&raw_data)
        } else {
            raw_data.resize(0x2774, 0x00);
            raw_data
        }
    }

    /// Returns the number of face button entries that [`raw_data`](Self::raw_data) will write.
    ///
    /// Each contiguous run of identical face button state is split into
    /// ceiling(`frames` / 255) entries because the per-entry frame count is a
    /// single byte.
    pub fn face_button_input_count(&self) -> u16 {
        let mut current_face_input_frames = 0u32;
        let mut face_button_input_count = 0u16;
        for (idx, current_input) in self.controller_inputs().iter().enumerate() {
            if idx == 0 {
                current_face_input_frames += current_input.frame_duration();
            } else if current_input.face_buttons_equal_to(self.controller_inputs[idx - 1]) {
                current_face_input_frames += current_input.frame_duration();
            } else {
                face_button_input_count += (current_face_input_frames / 255 + 1) as u16;
                current_face_input_frames = current_input.frame_duration();
            }
        }
        face_button_input_count += (current_face_input_frames / 255 + 1) as u16;
        face_button_input_count
    }

    /// Returns the number of stick input entries that [`raw_data`](Self::raw_data) will write.
    ///
    /// Each contiguous run of identical stick state is split into
    /// ceiling(`frames` / 255) entries because the per-entry frame count is a
    /// single byte.
    pub fn stick_input_count(&self) -> u16 {
        let mut current_stick_input_frames = 0u32;
        let mut stick_input_count = 0u16;
        for (idx, current_input) in self.controller_inputs().iter().enumerate() {
            if idx == 0 {
                current_stick_input_frames += current_input.frame_duration();
            } else if current_input.stick() == self.controller_inputs[idx - 1].stick() {
                current_stick_input_frames += current_input.frame_duration();
            } else {
                stick_input_count += (current_stick_input_frames / 255 + 1) as u16;
                current_stick_input_frames = current_input.frame_duration();
            }
        }
        stick_input_count += (current_stick_input_frames / 255 + 1) as u16;
        stick_input_count
    }

    /// Returns the number of dpad button entries that [`raw_data`](Self::raw_data) will write.
    ///
    /// Each contiguous run of identical dpad state is split into
    /// ceiling(`frames` / 4095) entries because the per-entry frame count is a
    /// 12-bit field.
    pub fn dpad_button_input_count(&self) -> u16 {
        let mut current_dpad_input_frames = 0u32;
        let mut dpad_button_input_count = 0u16;
        for (idx, current_input) in self.controller_inputs().iter().enumerate() {
            if idx == 0 {
                current_dpad_input_frames += current_input.frame_duration();
            } else if current_input.dpad() == self.controller_inputs[idx - 1].dpad() {
                current_dpad_input_frames += current_input.frame_duration();
            } else {
                dpad_button_input_count += (current_dpad_input_frames / 4095 + 1) as u16;
                current_dpad_input_frames = current_input.frame_duration();
            }
        }
        dpad_button_input_count += (current_dpad_input_frames / 4095 + 1) as u16;
        dpad_button_input_count
    }

    /// Returns the controller input runs as a slice.
    pub fn controller_inputs(&self) -> &[ControllerInput] {
        &self.controller_inputs
    }

    /// Returns a mutable slice of the controller input runs.
    pub fn controller_inputs_mut(&mut self) -> &mut [ControllerInput] {
        &mut self.controller_inputs
    }

    /// Returns `true` if [`raw_data`](Self::raw_data) should Yaz1-compress its output.
    pub fn compressed(&self) -> bool {
        self.compressed
    }

    /// Sets whether [`raw_data`](Self::raw_data) should Yaz1-compress its output.
    pub fn set_compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
}

fn to_face_buttons(input: &ControllerInput) -> FaceButtons {
    let mut buttons = Vec::new();
    if input.accelerator() {
        buttons.push(FaceButton::Accelerator);
    }
    if input.brake() {
        buttons.push(FaceButton::Brake);
    }
    if input.drift_flag() {
        buttons.push(FaceButton::Drift);
    }
    if input.brake_drift() {
        buttons.push(FaceButton::BrakeDrift);
    }
    if input.item() {
        buttons.push(FaceButton::Item);
    }
    if input.unknown_face_button() {
        buttons.push(FaceButton::Unknown);
    }
    FaceButtons(buttons)
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
    compressed_data.extend_from_slice(&((dst.len() + 16) as u32).to_be_bytes()); // size of Yaz1 section (magic + uncomp_size + padding + compressed)
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
        Self {
            prev_flag: false,
            stored_match_pos: 0,
            stored_num_bytes: 0,
        }
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
