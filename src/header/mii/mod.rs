use chrono::{DateTime, Duration, NaiveDateTime};

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

use std::io::{Read, Write};

pub(crate) mod birthday;
pub(crate) mod build;
pub(crate) mod eyebrows;
pub(crate) mod eyes;
pub(crate) mod facial_hair;
pub(crate) mod favorite_color;
pub(crate) mod glasses;
pub(crate) mod hair;
pub(crate) mod head;
pub(crate) mod lips;
pub(crate) mod mii_type;
pub(crate) mod mole;
pub(crate) mod nose;

#[doc(inline)]
pub use birthday::{Birthday, BirthdayError};
#[doc(inline)]
pub use build::{Build, BuildError};
#[doc(inline)]
pub use eyebrows::{EyebrowType, Eyebrows, EyebrowsError};
#[doc(inline)]
pub use eyes::{EyeColor, EyeType, Eyes, EyesError};
#[doc(inline)]
pub use facial_hair::{BeardType, FacialHair, FacialHairError, MustacheType};
#[doc(inline)]
pub use favorite_color::{FavoriteColor, FavoriteColorError};
#[doc(inline)]
pub use glasses::{Glasses, GlassesColor, GlassesError, GlassesType};
#[doc(inline)]
pub use hair::{Hair, HairColor, HairError, HairType};
#[doc(inline)]
pub use head::{FaceFeatures, Head, HeadError, HeadShape, SkinTone};
#[doc(inline)]
pub use lips::{Lips, LipsColor, LipsError, LipsType};
#[doc(inline)]
pub use mii_type::{MiiType, MiiTypeError};
#[doc(inline)]
pub use mole::{Mole, MoleError};
#[doc(inline)]
pub use nose::{Nose, NoseError, NoseType};

/// Errors that can occur while parsing or modifying a [`Mii`].
#[derive(thiserror::Error, Debug)]
pub enum MiiError {
    /// A UTF-16 byte sequence could not be decoded as a valid string.
    #[error("FromUtf16Error: {0}")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),
    /// The input data was not exactly `0x4A` bytes long.
    #[error("Invalid data length")]
    InvalidLength,
    /// A birthday field could not be parsed.
    #[error("Birthday Error: {0}")]
    BirthdayError(#[from] BirthdayError),
    /// A favorite color field could not be parsed.
    #[error("FavColor Error: {0}")]
    FavColorError(#[from] FavoriteColorError),
    /// A build (height/weight) field could not be parsed.
    #[error("Build Error: {0}")]
    BuildError(#[from] BuildError),
    /// A head (shape/skin tone/face features) field could not be parsed.
    #[error("Head Error: {0}")]
    HeadError(#[from] HeadError),
    /// A hair field could not be parsed.
    #[error("Hair Error: {0}")]
    HairError(#[from] HairError),
    /// An eyebrows field could not be parsed.
    #[error("Eyebrows Error: {0}")]
    EyebrowsError(#[from] EyebrowsError),
    /// An eyes field could not be parsed.
    #[error("Eyes Error: {0}")]
    EyesError(#[from] EyesError),
    /// A nose field could not be parsed.
    #[error("Nose Error: {0}")]
    NoseError(#[from] NoseError),
    /// A lips field could not be parsed.
    #[error("Lips Error: {0}")]
    LipsError(#[from] LipsError),
    /// A glasses field could not be parsed.
    #[error("Glasses Error: {0}")]
    GlassesError(#[from] GlassesError),
    /// A facial hair field could not be parsed.
    #[error("Facial Hair Error: {0}")]
    FacialHairError(#[from] FacialHairError),
    /// A mole field could not be parsed.
    #[error("Mole Error: {0}")]
    MoleError(#[from] MoleError),
    /// A Mii type field could not be parsed.
    #[error("Mii Type Error: {0}")]
    MiiTypeError(#[from] MiiTypeError),
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// A file I/O operation failed.
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

/// A fully parsed Mii character, as stored in the Wii's Mii data format.
///
/// Holds all customization fields decoded from the 74-byte (`0x4A`) Mii binary
/// block.
///
/// The binary layout is documented at <http://wiibrew.org/wiki/Mii_Data#Mii_format>.
#[derive(Debug, Clone)]
pub struct Mii {
    /// Bytes/bits from the original data not derived from any field above,
    /// OR'd back into the corresponding position when rebuilding [`raw_data`](Mii::raw_data).
    /// All zero for a [`Mii`] built with [`new`](Mii::new).
    reserved: [u8; 0x4A],
    /// Whether the Mii is female (`true`) or male (`false`).
    is_girl: bool,
    /// The Mii's birthday.
    birthday: Birthday,
    /// The Mii's favorite color, used to set outfit color.
    favorite_color: FavoriteColor,
    /// Whether this Mii has been marked as a favorite.
    is_favorite: bool,
    /// The Mii's display name (up to 10 characters).
    name: String,
    /// The Mii's body proportions (height and weight).
    build: Build,
    /// Whether this is a normal, foreign, or special Mii.
    mii_type: MiiType,
    /// The date and time the Mii was created, derived from the embedded timestamp.
    creation_date: NaiveDateTime,
    /// The full 32-bit Mii ID, including the type bits and creation timestamp.
    mii_id: u32,
    /// The ID of the Wii console that created this Mii.
    system_id: u32,
    /// The Mii's head shape, skin tone, and facial feature overlay.
    head: Head,
    /// Whether Mingle is disabled for this Mii.
    mingle_off: bool,
    /// Whether this Mii was downloaded from the Mii Channel.
    downloaded: bool,
    /// The Mii's hair style, color, and flip setting.
    hair: Hair,
    /// The Mii's eyebrow style, color, size, rotation, and position.
    eyebrows: Eyebrows,
    /// The Mii's eye style, color, size, rotation, and position.
    eyes: Eyes,
    /// The Mii's nose style, size, and vertical position.
    nose: Nose,
    /// The Mii's lip style, color, size, and vertical position.
    lips: Lips,
    /// The Mii's glasses style, color, size, and vertical position.
    glasses: Glasses,
    /// The Mii's beard and mustache style, color, size, and position.
    facial_hair: FacialHair,
    /// The Mii's mole visibility, position, and size.
    mole: Mole,
    /// The name of the player who created this Mii (up to 10 characters).
    creator_name: String,
}

impl Mii {
    /// Creates a [`Mii`] from its components as arguments.
    pub fn new(
        is_girl: bool,
        birthday: Birthday,
        favorite_color: FavoriteColor,
        is_favorite: bool,
        name: String,
        build: Build,
        mii_type: MiiType,
        creation_date: NaiveDateTime,
        system_id: u32,
        head: Head,
        mingle_enabled: bool,
        downloaded: bool,
        hair: Hair,
        eyebrows: Eyebrows,
        eyes: Eyes,
        nose: Nose,
        lips: Lips,
        glasses: Glasses,
        facial_hair: FacialHair,
        mole: Mole,
        creator_name: String,
    ) -> Self {
        let mii_id_prefix = (u8::from(mii_type) as u32) << 29;
        let creation_date_timestamp = timestamp_from_creation_date(creation_date) & 0x1FFFFFFF;
        let mii_id = mii_id_prefix | creation_date_timestamp;

        Self {
            reserved: [0u8; 0x4A],
            is_girl,
            birthday,
            favorite_color,
            is_favorite,
            name,
            build,
            mii_type,
            creation_date,
            mii_id,
            system_id,
            head,
            mingle_off: !mingle_enabled,
            downloaded,
            hair,
            eyebrows,
            eyes,
            nose,
            lips,
            glasses,
            facial_hair,
            mole,
            creator_name,
        }
    }

    /// Parses a [`Mii`] from a 74-byte (`0x4A`) data block.
    ///
    /// Accepts any type that can be converted into `[u8; 0x4A]`, such as a
    /// `&[u8]` slice or a byte array.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::InvalidLength`] if the input cannot be converted to
    /// exactly `0x4A` bytes. Returns other [`MiiError`] variants if any
    /// individual field fails to parse.
    pub fn new_from_bytes(mii_data: impl TryInto<[u8; 0x4A]>) -> Result<Self, MiiError> {
        let raw_data = mii_data.try_into().map_err(|_| MiiError::InvalidLength)?;

        let bytes = ByteHandler::try_from(&raw_data[0..=1])?;
        let is_girl = bytes.read_bool(6);
        let birthday = Birthday::from_byte_handler(bytes)?;

        let favorite_color = FavoriteColor::from_byte_handler(raw_data[1])?;
        let is_favorite = raw_data[1].is_multiple_of(2);

        let name = utf16be_to_string(&raw_data[0x02..=0x15])?;

        let build = Build::from_byte_handler(&raw_data[0x16..=0x17])?;

        let mii_type = MiiType::try_from(&raw_data[0x18] >> 5)?;

        let mut mii_id = raw_data[0x18..0x1C].to_owned();
        mii_id[0] &= 0x1F;
        let creation_date =
            creation_date_from_timestamp(u32::from_be_bytes(mii_id.try_into().unwrap()));

        let mii_id = ByteHandler::try_from(&raw_data[0x18..=0x1B])?.copy_dword();
        let system_id = ByteHandler::try_from(&raw_data[0x1C..=0x1F])?.copy_dword();

        let bytes = ByteHandler::try_from(&raw_data[0x20..=0x21])?;
        let mingle_off = bytes.read_bool(10);
        let downloaded = bytes.read_bool(8);
        let head = Head::from_byte_handler(bytes)?;
        let hair = Hair::from_byte_handler(&raw_data[0x22..=0x23])?;
        let eyebrows = Eyebrows::from_byte_handler(&raw_data[0x24..=0x27])?;
        let eyes = Eyes::from_byte_handler(&raw_data[0x28..=0x2B])?;
        let nose = Nose::from_byte_handler(&raw_data[0x2C..=0x2D])?;
        let lips = Lips::from_byte_handler(&raw_data[0x2E..=0x2F])?;
        let glasses = Glasses::from_byte_handler(&raw_data[0x30..=0x31])?;
        let facial_hair = FacialHair::from_byte_handler(&raw_data[0x32..=0x33])?;
        let mole = Mole::from_byte_handler(&raw_data[0x34..=0x35])?;

        let creator_name = utf16be_to_string(&raw_data[0x36..=0x49])?;

        let mut mii = Self {
            reserved: [0u8; 0x4A],
            is_girl,
            birthday,
            favorite_color,
            is_favorite,
            name,
            build,
            mii_type,
            creation_date,
            mii_id,
            system_id,
            head,
            mingle_off,
            downloaded,
            hair,
            eyebrows,
            eyes,
            nose,
            lips,
            glasses,
            facial_hair,
            mole,
            creator_name,
        };

        // Any bit not derived from a parsed field above (unused padding bits within
        // packed fields, and leftover bytes after the name fields' null terminators)
        // is captured here, so raw_data() has exactly the same output for an
        // unmodified Mii.
        let clean = mii.field_bytes();
        for i in 0..0x4A {
            mii.reserved[i] = raw_data[i] ^ clean[i];
        }

        Ok(mii)
    }

    /// Parses a [`Mii`] from a raw Mii file or an RKG ghost file.
    ///
    /// If the file begins with the `RKGD` magic bytes, Mii data is read
    /// starting at offset `0x3C` within the file. Otherwise the file is
    /// read from the beginning. In both cases exactly `0x4A` bytes are used.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::InvalidLength`] if the file contains fewer than
    /// `0x4A` usable bytes. Returns [`MiiError::IOError`] for file I/O
    /// failures, and other [`MiiError`] variants if any field fails to parse.
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, MiiError> {
        let mut data = Vec::new();
        std::fs::File::open(&path)?.read_to_end(&mut data)?;

        if data.len() >= 0x3C && data[..4] == *b"RKGD" {
            // Mii data starts at offset 0x3C in an RKG file
            data = Vec::from(&data[0x3C..]);
        }

        if data.len() < 0x4A {
            return Err(MiiError::InvalidLength);
        }

        Self::new_from_bytes(&data[..0x4A])
    }

    /// Writes the Mii's raw data to a file, creating or truncating it as needed.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::IOError`] if the file cannot be created or written.
    pub fn save_to_file<T: AsRef<std::path::Path>>(&self, path: T) -> Result<(), MiiError> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(&self.raw_data())?;

        Ok(())
    }

    /// Returns the raw 74-byte Mii data block, computed from the current field values.
    ///
    /// Bits not derived from any field (unused padding bits within packed fields,
    /// and leftover bytes after the name fields' null terminators) are filled in from
    /// whatever was captured when this [`Mii`] was parsed (all zero otherwise), so a
    /// freshly parsed, unmodified [`Mii`] round-trips back to its original bytes exactly.
    pub fn raw_data(&self) -> [u8; 0x4A] {
        let mut raw_data = self.field_bytes();
        for (byte, reserved) in raw_data.iter_mut().zip(self.reserved.iter()) {
            *byte |= *reserved;
        }
        raw_data
    }

    /// Builds the raw 74-byte Mii data block from the parsed fields alone, mirroring
    /// the field layout read by [`new_from_bytes`](Mii::new_from_bytes) in reverse.
    /// Bits not owned by any field are left as `0`.
    fn field_bytes(&self) -> [u8; 0x4A] {
        let mut raw_data = Vec::with_capacity(0x4A);

        let mut mii_info_bytes = <[u8; 2]>::from(self.birthday);
        mii_info_bytes[0] |= (self.is_girl as u8) << 6;
        mii_info_bytes[1] |= u8::from(self.favorite_color) << 1;
        mii_info_bytes[1] |= !self.is_favorite as u8;

        raw_data.extend_from_slice(&mii_info_bytes);
        let mut name_bytes = [0u8; 0x14];
        let name_utf16 = string_to_utf16be(&self.name);
        name_bytes[..name_utf16.len()].copy_from_slice(&name_utf16);
        raw_data.extend_from_slice(&name_bytes);
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.build));
        raw_data.extend_from_slice(&self.mii_id.to_be_bytes());
        raw_data.extend_from_slice(&self.system_id.to_be_bytes());

        let mut head_bytes = <[u8; 2]>::from(self.head);
        head_bytes[1] |= (self.mingle_off as u8) << 2;
        head_bytes[1] |= self.downloaded as u8;
        raw_data.extend_from_slice(&head_bytes);

        raw_data.extend_from_slice(&<[u8; 2]>::from(self.hair));
        raw_data.extend_from_slice(&<[u8; 4]>::from(self.eyebrows));
        raw_data.extend_from_slice(&<[u8; 4]>::from(self.eyes));
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.nose));
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.lips));
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.glasses));
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.facial_hair));
        raw_data.extend_from_slice(&<[u8; 2]>::from(self.mole));
        let mut creator_name_bytes = [0u8; 0x14];
        let creator_utf16 = string_to_utf16be(&self.creator_name);
        creator_name_bytes[..creator_utf16.len()].copy_from_slice(&creator_utf16);
        raw_data.extend_from_slice(&creator_name_bytes);

        raw_data.try_into().unwrap()
    }

    /// Returns whether the Mii is female.
    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    /// Sets whether the Mii is female.
    pub fn set_is_girl(&mut self, is_girl: bool) {
        self.is_girl = is_girl;
    }

    /// Returns the Mii's birthday.
    pub fn birthday(&self) -> Birthday {
        self.birthday
    }

    /// Sets the Mii's birthday.
    pub fn set_birthday(&mut self, birthday: Birthday) {
        self.birthday = birthday;
    }

    /// Returns the Mii's favorite color.
    pub fn favorite_color(&self) -> FavoriteColor {
        self.favorite_color
    }

    /// Sets the Mii's favorite color.
    pub fn set_favorite_color(&mut self, favorite_color: FavoriteColor) {
        self.favorite_color = favorite_color;
    }

    /// Returns whether this Mii has been marked as a favorite.
    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }

    /// Sets whether this Mii is marked as a favorite.
    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = is_favorite;
    }

    /// Returns the Mii's display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the Mii's display name.
    ///
    /// Names longer than 10 characters are silently ignored.
    pub fn set_name(&mut self, name: &str) {
        if name.len() > 10 {
            return;
        }
        self.name = name.to_string();
        self.reserved[0x02..0x16].fill(0);
    }

    /// Returns the Mii's body proportions (height and weight).
    pub fn build(&self) -> Build {
        self.build
    }

    /// Sets the Mii's body proportions.
    pub fn set_build(&mut self, build: Build) {
        self.build = build;
    }

    /// Returns whether this is a normal, foreign, or special Mii.
    pub fn mii_type(&self) -> MiiType {
        self.mii_type
    }

    /// Sets the Mii type and updates the type bits in the Mii ID.
    ///
    /// Does nothing if the type is already set to the given value.
    pub fn set_mii_type(&mut self, mii_type: MiiType) {
        if self.mii_type() == mii_type {
            return;
        }
        self.mii_id = (self.mii_id & 0x1FFF_FFFF) | ((u8::from(mii_type) as u32) << 29);
        self.mii_type = mii_type;
    }

    /// Returns the date and time the Mii was created.
    pub fn creation_date(&self) -> NaiveDateTime {
        self.creation_date
    }

    /// Sets the Mii's creation date and updates the timestamp bits in the Mii ID.
    ///
    /// Note: the valid range of the timestamp has not been fully determined.
    pub fn set_creation_date(&mut self, creation_date: NaiveDateTime) {
        let creation_date_timestamp = timestamp_from_creation_date(creation_date) & 0x1FFFFFFF;
        self.mii_id = (self.mii_id & 0xE000_0000) | creation_date_timestamp;
        self.creation_date = creation_date;
    }

    /// Returns the full 32-bit Mii ID, which encodes both the type and creation timestamp.
    pub fn mii_id(&self) -> u32 {
        self.mii_id
    }

    /// Returns the ID of the Wii console that created this Mii.
    pub fn system_id(&self) -> u32 {
        self.system_id
    }

    /// Sets the system ID.
    pub fn set_system_id(&mut self, system_id: u32) {
        self.system_id = system_id;
    }

    /// Returns the Mii's head shape, skin tone, and facial feature overlay.
    pub fn head(&self) -> Head {
        self.head
    }

    /// Sets the Mii's head options.
    pub fn set_head(&mut self, head: Head) {
        self.head = head;
    }

    /// Returns whether Mingle is enabled for this Mii.
    pub fn is_mingle_enabled(&self) -> bool {
        !self.mingle_off
    }

    /// Sets whether Mingle is enabled for this Mii.
    pub fn set_mingle_enabled(&mut self, enabled: bool) {
        self.mingle_off = !enabled;
    }

    /// Returns whether this Mii was downloaded from the Mii Channel.
    pub fn downloaded(&self) -> bool {
        self.downloaded
    }

    /// Sets whether this Mii was downloaded from the Mii Channel.
    pub fn set_downloaded(&mut self, downloaded: bool) {
        self.downloaded = downloaded;
    }

    /// Returns the Mii's hair style, color, and flip setting.
    pub fn hair(&self) -> Hair {
        self.hair
    }

    /// Sets the Mii's hair options.
    pub fn set_hair(&mut self, hair: Hair) {
        self.hair = hair;
    }

    /// Returns the Mii's eyebrow options.
    pub fn eyebrows(&self) -> Eyebrows {
        self.eyebrows
    }

    /// Sets the Mii's eyebrow options.
    pub fn set_eyebrows(&mut self, eyebrows: Eyebrows) {
        self.eyebrows = eyebrows;
    }

    /// Returns the Mii's eye options.
    pub fn eyes(&self) -> Eyes {
        self.eyes
    }

    /// Sets the Mii's eye options.
    pub fn set_eyes(&mut self, eyes: Eyes) {
        self.eyes = eyes;
    }

    /// Returns the Mii's nose options.
    pub fn nose(&self) -> Nose {
        self.nose
    }

    /// Sets the Mii's nose options.
    pub fn set_nose(&mut self, nose: Nose) {
        self.nose = nose;
    }

    /// Returns the Mii's lip options.
    pub fn lips(&self) -> Lips {
        self.lips
    }

    /// Sets the Mii's lip options.
    pub fn set_lips(&mut self, lips: Lips) {
        self.lips = lips;
    }

    /// Returns the Mii's glasses options.
    pub fn glasses(&self) -> Glasses {
        self.glasses
    }

    /// Sets the Mii's glasses options.
    pub fn set_glasses(&mut self, glasses: Glasses) {
        self.glasses = glasses;
    }

    /// Returns the Mii's facial hair options.
    pub fn facial_hair(&self) -> FacialHair {
        self.facial_hair
    }

    /// Sets the Mii's facial hair options.
    pub fn set_facial_hair(&mut self, facial_hair: FacialHair) {
        self.facial_hair = facial_hair;
    }

    /// Returns the Mii's mole options.
    pub fn mole(&self) -> Mole {
        self.mole
    }

    /// Sets the Mii's mole options.
    pub fn set_mole(&mut self, mole: Mole) {
        self.mole = mole;
    }

    /// Returns the name of the player who created this Mii.
    pub fn creator_name(&self) -> &str {
        &self.creator_name
    }

    /// Sets the creator name.
    ///
    /// Names longer than 10 characters are silently ignored.
    pub fn set_creator_name(&mut self, creator_name: &str) {
        if creator_name.len() > 10 {
            return;
        }
        self.creator_name = creator_name.to_string();
        self.reserved[0x36..0x4A].fill(0);
    }
}

impl Default for Mii {
    fn default() -> Self {
        Self::new(
            false,
            Birthday::new(0, 0).unwrap(),
            FavoriteColor::Red,
            true,
            String::from("no name"),
            Build::new(64, 64).unwrap(),
            MiiType::Normal,
            NaiveDateTime::parse_from_str("2006-01-01 00:00:08", "%Y-%m-%d %H:%M:%S").unwrap(),
            3976168146,
            Head::new(HeadShape::Sharp, SkinTone::Natural, FaceFeatures::None),
            false,
            false,
            Hair::new(HairType::NormalLong, HairColor::Chocolate, false),
            Eyebrows::new(
                6,
                4,
                2,
                10,
                HairColor::Chocolate,
                EyebrowType::FlatAngledLarge,
            )
            .unwrap(),
            Eyes::new(4, 4, 2, 12, EyeColor::Black, EyeType::Normal).unwrap(),
            Nose::new(9, 4, NoseType::Normal).unwrap(),
            Lips::new(13, 4, LipsType::Neutral, LipsColor::Orange).unwrap(),
            Glasses::new(10, 4, GlassesType::None, GlassesColor::Black).unwrap(),
            FacialHair::new(BeardType::None, MustacheType::None, HairColor::Black, 4, 10).unwrap(),
            Mole::new(false, 2, 20, 4).unwrap(),
            String::new(),
        )
    }
}

/// Compares Miis by their decoded fields only; `reserved` (leftover, meaningless
/// padding bits from the original data) is deliberately excluded.
impl PartialEq for Mii {
    fn eq(&self, other: &Self) -> bool {
        self.is_girl == other.is_girl
            && self.birthday == other.birthday
            && self.favorite_color == other.favorite_color
            && self.is_favorite == other.is_favorite
            && self.name == other.name
            && self.build == other.build
            && self.mii_type == other.mii_type
            && self.creation_date == other.creation_date
            && self.mii_id == other.mii_id
            && self.system_id == other.system_id
            && self.head == other.head
            && self.mingle_off == other.mingle_off
            && self.downloaded == other.downloaded
            && self.hair == other.hair
            && self.eyebrows == other.eyebrows
            && self.eyes == other.eyes
            && self.nose == other.nose
            && self.lips == other.lips
            && self.glasses == other.glasses
            && self.facial_hair == other.facial_hair
            && self.mole == other.mole
            && self.creator_name == other.creator_name
    }
}

/// Decodes a big-endian UTF-16 byte slice into a [`String`], stopping at the
/// first null character (`U+0000`).
///
/// # Errors
///
/// Returns a [`std::string::FromUtf16Error`] if the byte sequence contains
/// invalid UTF-16 code units.
fn utf16be_to_string(bytes: &[u8]) -> Result<String, std::string::FromUtf16Error> {
    let utf16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .take_while(|&u| u != 0)
        .collect();

    String::from_utf16(&utf16)
}

/// Encodes a UTF-8 string as a big-endian UTF-16 byte sequence.
///
/// The result is not null-terminated; callers are responsible for any padding.
fn string_to_utf16be(string: &str) -> Vec<u8> {
    string
        .encode_utf16()
        .collect::<Vec<u16>>()
        .iter()
        .flat_map(|&u| u.to_be_bytes())
        .collect()
}

/// Converts a raw 29-bit Mii creation timestamp into a [`NaiveDateTime`].
///
/// The timestamp counts ticks at a rate of 4 ticks per second (i.e. each tick
/// is 0.25 seconds), relative to the Mii epoch of 2006-01-01 00:00:00 UTC.
fn creation_date_from_timestamp(value: u32) -> NaiveDateTime {
    let clock_rate = 0.25; // 3 second ticks
    let epoch_shift = 1_136_073_600; // Shifts epoch from 1970-01-01 to 2006-01-01 (which is what Miis use)
    let total_seconds = (value as f64 / clock_rate).floor() as i64;

    let duration = Duration::seconds(total_seconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

/// Converts a [`NaiveDateTime`] into a raw 29-bit Mii creation timestamp.
///
/// The timestamp counts ticks at a rate of 4 ticks per second (i.e. each tick
/// is 0.25 seconds), relative to the Mii epoch of 2006-01-01 00:00:00 UTC.
fn timestamp_from_creation_date(date: NaiveDateTime) -> u32 {
    let clock_rate = 0.25; // 3 second ticks
    let epoch_shift = 1_136_073_600; // Shifts epoch from 1970-01-01 to 2006-01-01 (which is what Miis use)
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    let duration = date.signed_duration_since(epoch.naive_utc());
    let total_seconds = duration.num_seconds();

    (total_seconds as f64 * clock_rate).floor() as u32
}
