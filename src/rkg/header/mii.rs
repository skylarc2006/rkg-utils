// TEMPORARY!!! Both of these traits are only here until work on reading mii data actually begins
#[allow(dead_code)]
#[derive(Default)]
pub struct Mii {
    unknown1: bool,                 // 1 bit, offset 0x00
    is_girl: bool,                  // 1 bit, offset 0x00.1
    month: u8,                      // 4 bits, offset 0x00.2
    day: u8,                        // 5 bits, offset 0x00.6
    favorite_color: u8,             // 4 bits, offset 0x01.3
    is_favorite: bool,              // 1 bit, offset 0x01.7
    name: String,                   // 0x14 (10 chars), offset 0x02
    height: u8,                     // 0x01, offset 0x16
    weight: u8,                     // 0x01, offset 0x17
    mii_id1: u8,                    // 0x01, offset 0x18
    mii_id2: u8,                    // 0x01, offset 0x19
    mii_id3: u8,                    // 0x01, offset 0x1A
    mii_id4: u8,                    // 0x01, offset 0x1B
    system_id0: u8,                 // 0x01, offset 0x1C
    system_id1: u8,                 // 0x01, offset 0x1D
    system_id2: u8,                 // 0x01, offset 0x1E
    system_id3: u8,                 // 0x01, offset 0x1F
    face_shape: u8,                 // 3 bits, offset 0x20
    skin_color: u8,                 // 3 bits, offset 0x20.3
    facial_feature: u8,             // 4 bits, offset 0x20.6
    unknown2: u8,                   // 3 bits, offset 0x21.2
    mingle_off: bool,               // 1 bit, offset 0x21.5
    unknown3: bool,                 // 1 bit, offset 0x21.6
    downloaded: bool,               // 1 bit, offset 0x21.7
    hair_type: u8,                  // 7 bits, offset 0x22
    hair_color: u8,                 // 3 bits, offset 0x22.7
    hair_part_reversed: bool,       // 1 bit, offset 0x23.2
    unknown4: u8,                   // 5 bits, offset 0x23.3
    eyebrow_type: u8,               // 5 bits, offset 0x24
    unknown5: bool,                 // 1 bit, offset 0x24.5
    eyebrow_rotation: u8,           // 4 bits, offset 0x24.6
    unknown6: u8,                   // 6 bits, offset 0x25.2
    eyebrow_color: u8,              // 3 bits, offset 0x26
    eyebrow_size: u8,               // 4 bits, offset 0x26.3
    eyebrow_vertical_pos: u8,       // 5 bits, offset 0x26.7
    eyebrow_horizontal_spacing: u8, // 4 bits, offset 0x27.4
    eye_type: u8,                   // 6 bits, offset 0x28
    unknown7: u8,                   // 2 bits, offset 0x28.6
    eye_rotation: u8,               // 3 bits, offset 0x29
    eye_vertical_pos: u8,           // 5 bits, offset 0x29.3
    eye_color: u8,                  // 3 bits, offset 0x2A
    unknown8: bool,                 // 1 bit, offset 0x2A.3
    eye_size: u8,                   // 3 bits, offset 0x2A.4
    eye_horizontal_spacing: u8,     // 4 bits, offset 0x2A.7
    unknown9: u8,                   // 5 bits, offset 0x2B.3
    nose_type: u8,                  // 4 bits, offset 0x2C
    nose_size: u8,                  // 4 bits, offset 0x2C.4
    nose_vertical_pos: u8,          // 5 bits, offset 0x2D
    unknown10: u8,                  // 3 bits, offset 0x2D.5
    lip_type: u8,                   // 5 bits, offset 0x2E
    lip_color: u8,                  // 2 bits, offset 0x2E.5
    lip_size: u8,                   // 4 bits, offset 0x2E.7
    lip_vertical_pos: u8,           // 5 bits, offset 0x2F.3
    glasses_type: u8,               // 4 bits, offset 0x30
    glasses_color: u8,              // 3 bits, offset 0x30.4
    unknown11: bool,                // 1 bit, offset 0x30.7
    glasses_size: u8,               // 3 bits, offset 0x31
    glasses_vertical_pos: u8,       // 5 bits, offset 0x31.3
    mustache_type: u8,              // 2 bits, offset 0x32
    beard_type: u8,                 // 2 bits, offset 0x32.2
    facial_hair_color: u8,          // 3 bits, offset 0x32.4
    mustache_size: u8,              // 4 bits, offset 0x32.7
    mustache_vertical_pos: u8,      // 5 bits, offset 0x33.3
    has_mole: bool,                 // 1 bit, offset 0x34
    mole_size: u8,                  // 4 bits, offset 0x34.1
    mole_vertical_pos: u8,          // 5 bits, offset 0x34.5
    mole_horizontal_pos: u8,        // 5 bits, offset 0x35.2
    unknown12: bool,                // 1 bit, offset 0x35.7
    creator_name: String,           // 0x14 (10 chars), offset 0x36
}

impl Mii {
    pub fn new(mii_data: &[u8]) -> Self {
        // TODO: read mii data
        mii_data[0]; // This line is only here to avoid complaining about unused variables, remove this when implementing actual data reading
        Self::default()
    }

    pub fn unknown1(&self) -> bool {
        self.unknown1
    }

    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn favorite_color(&self) -> u8 {
        self.favorite_color
    }

    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn weight(&self) -> u8 {
        self.weight
    }

    pub fn mii_id1(&self) -> u8 {
        self.mii_id1
    }

    pub fn mii_id2(&self) -> u8 {
        self.mii_id2
    }

    pub fn mii_id3(&self) -> u8 {
        self.mii_id3
    }

    pub fn mii_id4(&self) -> u8 {
        self.mii_id4
    }

    pub fn system_id0(&self) -> u8 {
        self.system_id0
    }

    pub fn system_id1(&self) -> u8 {
        self.system_id1
    }

    pub fn system_id2(&self) -> u8 {
        self.system_id2
    }

    pub fn system_id3(&self) -> u8 {
        self.system_id3
    }

    pub fn face_shape(&self) -> u8 {
        self.face_shape
    }

    pub fn skin_color(&self) -> u8 {
        self.skin_color
    }

    pub fn facial_feature(&self) -> u8 {
        self.facial_feature
    }

    pub fn unknown2(&self) -> u8 {
        self.unknown2
    }

    pub fn mingle_off(&self) -> bool {
        self.mingle_off
    }

    pub fn unknown3(&self) -> bool {
        self.unknown3
    }

    pub fn downloaded(&self) -> bool {
        self.downloaded
    }

    pub fn hair_type(&self) -> u8 {
        self.hair_type
    }

    pub fn hair_color(&self) -> u8 {
        self.hair_color
    }

    pub fn hair_part_reversed(&self) -> bool {
        self.hair_part_reversed
    }

    pub fn unknown4(&self) -> u8 {
        self.unknown4
    }

    pub fn eyebrow_type(&self) -> u8 {
        self.eyebrow_type
    }

    pub fn unknown5(&self) -> bool {
        self.unknown5
    }

    pub fn eyebrow_rotation(&self) -> u8 {
        self.eyebrow_rotation
    }

    pub fn unknown6(&self) -> u8 {
        self.unknown6
    }

    pub fn eyebrow_color(&self) -> u8 {
        self.eyebrow_color
    }

    pub fn eyebrow_size(&self) -> u8 {
        self.eyebrow_size
    }

    pub fn eyebrow_vertical_pos(&self) -> u8 {
        self.eyebrow_vertical_pos
    }

    pub fn eyebrow_horizontal_spacing(&self) -> u8 {
        self.eyebrow_horizontal_spacing
    }

    pub fn eye_type(&self) -> u8 {
        self.eye_type
    }

    pub fn unknown7(&self) -> u8 {
        self.unknown7
    }

    pub fn eye_rotation(&self) -> u8 {
        self.eye_rotation
    }

    pub fn eye_vertical_pos(&self) -> u8 {
        self.eye_vertical_pos
    }

    pub fn eye_color(&self) -> u8 {
        self.eye_color
    }

    pub fn unknown8(&self) -> bool {
        self.unknown8
    }

    pub fn eye_size(&self) -> u8 {
        self.eye_size
    }

    pub fn eye_horizontal_spacing(&self) -> u8 {
        self.eye_horizontal_spacing
    }

    pub fn unknown9(&self) -> u8 {
        self.unknown9
    }

    pub fn nose_type(&self) -> u8 {
        self.nose_type
    }

    pub fn nose_size(&self) -> u8 {
        self.nose_size
    }

    pub fn nose_vertical_pos(&self) -> u8 {
        self.nose_vertical_pos
    }

    pub fn unknown10(&self) -> u8 {
        self.unknown10
    }

    pub fn lip_type(&self) -> u8 {
        self.lip_type
    }

    pub fn lip_color(&self) -> u8 {
        self.lip_color
    }

    pub fn lip_size(&self) -> u8 {
        self.lip_size
    }

    pub fn lip_vertical_pos(&self) -> u8 {
        self.lip_vertical_pos
    }

    pub fn glasses_type(&self) -> u8 {
        self.glasses_type
    }

    pub fn glasses_color(&self) -> u8 {
        self.glasses_color
    }

    pub fn unknown11(&self) -> bool {
        self.unknown11
    }

    pub fn glasses_size(&self) -> u8 {
        self.glasses_size
    }

    pub fn glasses_vertical_pos(&self) -> u8 {
        self.glasses_vertical_pos
    }

    pub fn mustache_type(&self) -> u8 {
        self.mustache_type
    }

    pub fn beard_type(&self) -> u8 {
        self.beard_type
    }

    pub fn facial_hair_color(&self) -> u8 {
        self.facial_hair_color
    }

    pub fn mustache_size(&self) -> u8 {
        self.mustache_size
    }

    pub fn mustache_vertical_pos(&self) -> u8 {
        self.mustache_vertical_pos
    }

    pub fn has_mole(&self) -> bool {
        self.has_mole
    }

    pub fn mole_size(&self) -> u8 {
        self.mole_size
    }

    pub fn mole_vertical_pos(&self) -> u8 {
        self.mole_vertical_pos
    }

    pub fn mole_horizontal_pos(&self) -> u8 {
        self.mole_horizontal_pos
    }

    pub fn unknown12(&self) -> bool {
        self.unknown12
    }

    pub fn creator_name(&self) -> &str {
        &self.creator_name
    }
}
