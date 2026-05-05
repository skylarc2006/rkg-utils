use chrono::NaiveDateTime;

use crate::{
    Ghost, GhostError,
    crc::{crc16, crc32},
    footer::{
        FooterType,
        ctgp_footer::{CTGPFooter, region::Region},
    },
    header::{
        Header, HeaderError,
        combo::{Combo, ComboError, character::Character, vehicle::Vehicle},
        controller::Controller,
        date::{Date, DateError},
        ghost_type::{GhostType, GhostTypeError},
        in_game_time::{InGameTime, InGameTimeError},
        location::{Location, constants::*},
        mii::{
            Mii,
            birthday::Birthday,
            build::Build,
            eyebrows::{EyebrowType, Eyebrows},
            eyes::{EyeColor, EyeType, Eyes},
            facial_hair::{BeardType, FacialHair, MustacheType},
            favorite_color::FavoriteColor,
            glasses::{Glasses, GlassesColor, GlassesType},
            hair::{Hair, HairColor, HairType},
            head::{FaceFeatures, Head, HeadShape, SkinTone},
            lips::{Lips, LipsColor, LipsType},
            mii_type::MiiType,
            mole::Mole,
            nose::{Nose, NoseType},
        },
        slot_id::{SlotId, SlotIdError},
        transmission_mod::TransmissionMod,
    },
    input_data::{
        InputData, InputDataError,
        controller_input::ControllerInput,
        dpad_button::DPadButton,
        stick_input::{StickInput, StickInputError},
        yaz1_compress, yaz1_decompress,
    },
    write_bits,
};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let header =
        Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").expect("Couldn't read header");

    // General ghost info
    assert_eq!(header.finish_time().minutes(), 1);
    assert_eq!(header.finish_time().seconds(), 3);
    assert_eq!(header.finish_time().milliseconds(), 904);
    assert_eq!(header.finish_time().to_string(), "01:03.904");
    assert_eq!(header.slot_id(), SlotId::LuigiCircuit);
    assert_eq!(header.combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(header.combo().character(), Character::KingBoo);
    assert_eq!(header.date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(header.controller(), Controller::Classic);
    assert!(header.is_compressed());
    assert_eq!(header.ghost_type(), GhostType::ExpertStaff);
    assert!(header.is_automatic_drift());
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.lap_count(), 3);
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.location().country(), Country::NotSet);
    assert_eq!(header.location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!header.mii().is_girl());
    assert_eq!(header.mii().birthday().month(), Some(1));
    assert_eq!(header.mii().birthday().day(), Some(1));
    assert_eq!(header.mii().favorite_color(), FavoriteColor::ForestGreen);
    assert_eq!(header.mii().name(), "JC");
    assert_eq!(header.mii().build().height(), 127);
    assert_eq!(header.mii().build().weight(), 127);

    assert_eq!(header.mii().mii_id(), 0x893EF2FB);
    assert_eq!(header.mii().system_id(), 0x689EC992);

    assert_eq!(header.mii().head().shape(), HeadShape::Large);
    assert_eq!(header.mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(header.mii().head().face_features(), FaceFeatures::None);

    assert!(!header.mii().is_mingle_enabled());
    assert!(!header.mii().downloaded());

    assert_eq!(header.mii().hair().hair_type(), HairType::NormalLong);
    assert_eq!(header.mii().hair().hair_color(), HairColor::PhilippineBrown);
    assert!(!header.mii().hair().is_flipped());

    assert_eq!(header.mii().eyebrows().eyebrow_type(), EyebrowType::None);
    assert_eq!(header.mii().eyebrows().rotation(), 5);
    assert_eq!(
        header.mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(header.mii().eyebrows().size(), 4);
    assert_eq!(header.mii().eyebrows().y(), 10);
    assert_eq!(header.mii().eyebrows().x(), 2);

    assert_eq!(header.mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(header.mii().eyes().rotation(), 4);
    assert_eq!(header.mii().eyes().y(), 9);
    assert_eq!(header.mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(header.mii().eyes().size(), 6);
    assert_eq!(header.mii().eyes().x(), 1);

    assert_eq!(header.mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(header.mii().nose().size(), 0);
    assert_eq!(header.mii().nose().y(), 8);

    assert_eq!(header.mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(header.mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(header.mii().lips().size(), 7);
    assert_eq!(header.mii().lips().y(), 6);

    assert_eq!(header.mii().glasses().glasses_type(), GlassesType::None);
    assert_eq!(header.mii().glasses().glasses_color(), GlassesColor::Black);
    assert_eq!(header.mii().glasses().size(), 4);
    assert_eq!(header.mii().glasses().y(), 10);

    assert_eq!(
        header.mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(header.mii().facial_hair().beard_type(), BeardType::None);
    assert_eq!(header.mii().facial_hair().color(), HairColor::Black);
    assert_eq!(header.mii().facial_hair().mustache_size(), 4);
    assert_eq!(header.mii().facial_hair().mustache_y(), 10);

    assert!(!header.mii().mole().has_mole());
    assert_eq!(header.mii().mole().size(), 4);
    assert_eq!(header.mii().mole().y(), 20);
    assert_eq!(header.mii().mole().x(), 2);

    assert_eq!(header.mii().creator_name(), "JC");

    assert_eq!(header.mii_crc16(), 0x06F4);
}

#[test]
fn print_rkg_header() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed_Copy.rkg")
        .expect("Couldn't read header");

    println!("Track: {}", header.slot_id());
    println!("Time: {}", header.finish_time());
    println!("Date set: {:?}", header.date_set());
    println!("Player: {}", header.mii().name());
    println!("Country: {}", header.location().country());
    println!("Subregion: {}", header.location().subregion());
    println!("Controller: {}", header.controller());
    println!(
        "Combo: {} ({} Drift)",
        header.combo(),
        if header.is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        }
    );
    println!("Ghost type: {}", header.ghost_type());
    println!("Input data compressed? {}", header.is_compressed());
    println!(
        "Decompressed input data length: {}",
        header.decompressed_input_data_length()
    );
    println!("Total lap count: {}\n", header.lap_count());

    for (index, lap_time) in header.lap_split_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, lap_time);
    }
}

#[test]
fn test_rkg_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data = InputData::new_from_bytes(&rkg_data[0x88..rkg_data.len() - 0xE0])
        .expect("Couldn't read input data");

    assert_eq!(input_data.face_button_input_count(), 0x18);
    assert_eq!(input_data.stick_input_count(), 0x037B);
    assert_eq!(input_data.dpad_button_input_count(), 0x09);
    assert_eq!(input_data.controller_inputs().len(), 907);

    assert!(!input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn print_input_data() {
    let ghost = Ghost::new_from_file("./test_ghosts/illegal_brake_input.rkg").unwrap();

    for input in ghost.input_data().controller_inputs().iter() {
        println!("{:#?}", input)
    }
}

#[test]
fn test_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Some asserts
    assert_eq!(
        ctgp_metadata.track_sha1(),
        [
            0x1A, 0xE1, 0xA7, 0xD8, 0x94, 0x96, 0x0B, 0x38, 0xE0, 0x9E, 0x74, 0x94, 0x37, 0x33,
            0x78, 0xD8, 0x73, 0x05, 0xA1, 0x63
        ]
    );
    assert_eq!(
        ctgp_metadata.player_id().to_be_bytes(),
        [0xFD, 0x31, 0x97, 0xB0, 0x7D, 0x9D, 0x2B, 0x84]
    );
    let shroomstrat: [u8; 3] = [3, 0, 0];
    assert_eq!(ctgp_metadata.shroomstrat(), &shroomstrat);
}

#[test]
fn print_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/00m58s6479888 David .rkg")
        .expect("Couldn't find `./test_ghosts/00m58s6479888 David .rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Print info
    print!("Track SHA1: ");
    for byte in ctgp_metadata.track_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    print!("Ghost SHA1: ");
    for byte in ctgp_metadata.ghost_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    print!("Player ID: ");
    for byte in ctgp_metadata.player_id().to_be_bytes().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    println!("Exact finish time: {}", ctgp_metadata.exact_finish_time());
    println!("CORE version: {}", ctgp_metadata.core_version());
    print!("Possible CTGP release versions: ");
    if let Some(ctgp_versions) = ctgp_metadata.possible_ctgp_versions() {
        for version in ctgp_versions {
            print!("{}, ", version)
        }
        println!();
    } else {
        println!("Unknown")
    }
    println!();

    for (index, time) in ctgp_metadata.exact_lap_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, time);
    }
    println!();

    println!("RTC Race Begin: {}", ctgp_metadata.rtc_race_begins());
    println!("RTC Race End: {}", ctgp_metadata.rtc_race_end());
    println!(
        "RTC Time Paused: {}ms",
        ctgp_metadata.rtc_time_paused().num_milliseconds()
    );
    println!("List of pause times: [");
    for time in ctgp_metadata.pause_times() {
        println!("  {},", time)
    }
    println!("]");

    println!("\nMy Stuff enabled? {}", ctgp_metadata.my_stuff_enabled());
    println!("My Stuff used? {}", ctgp_metadata.my_stuff_used());
    println!(
        "USB Gamecube enabled? {}",
        ctgp_metadata.usb_gamecube_enabled()
    );
    println!(
        "Disc region: {:#?}",
        ctgp_metadata.disc_region().unwrap_or(Region::Unknown)
    );
    println!(
        "Final lap dubious intersection? {}",
        ctgp_metadata.final_lap_suspicious_intersection()
    );

    println!(
        "\nAll lap dubious intersection bools: {:?}",
        ctgp_metadata.lap_split_suspicious_intersections().unwrap()
    );

    println!("\nShroomstrat: {:?}", ctgp_metadata.shroomstrat());
    println!("Category: {:?}", ctgp_metadata.category());
    println!("Cannoned? {}", ctgp_metadata.cannoned());
    println!("Went OOB? {}", ctgp_metadata.went_oob());
    println!("Slowdown suspected? {}", ctgp_metadata.potential_slowdown());
    println!(
        "Rapidfire suspected? {}",
        ctgp_metadata.potential_rapidfire()
    );
    println!(
        "Suspicious ghost? {}",
        ctgp_metadata.potentially_cheated_ghost()
    );
    println!(
        "Has Mii data replaced? {}",
        ctgp_metadata.has_mii_data_replaced()
    );
    println!(
        "Has Mii name replaced? {}",
        ctgp_metadata.has_name_replaced()
    );
    println!("Respawns? {}", ctgp_metadata.respawns());
    println!("CTGP metadata version: {}", ctgp_metadata.footer_version());
}

/// CTGP adds a pause mask to frames where a pause is pressed. Actual race inputs should stay the same.
#[test]
fn test_ctgp_pause_vs_vanilla_input_timing() {
    let mut pause_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_compressed.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_compressed.rkg`")
        .read_to_end(&mut pause_rkg_data)
        .expect("Couldn't read bytes in file");

    let mut vanilla_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_vanilla.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_vanilla.rkg`")
        .read_to_end(&mut vanilla_rkg_data)
        .expect("Couldn't read bytes in file");

    let pause_inputs =
        InputData::new_from_bytes(&pause_rkg_data[0x88..pause_rkg_data.len() - 0xE0])
            .expect("Failed to read inputs from pause ghost");
    let vanilla_inputs =
        InputData::new_from_bytes(&vanilla_rkg_data[0x88..vanilla_rkg_data.len() - 0x04])
            .expect("Failed to read inputs from vanilla ghost");

    assert_eq!(
        pause_inputs.controller_inputs(),
        vanilla_inputs.controller_inputs()
    );
}

#[test]
fn illegal_drift_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_drift_input.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_drift_input.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data = InputData::new_from_bytes(&rkg_data[0x88..rkg_data.len() - 0x04])
        .expect("Failed to read input data");
    assert!(input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn illegal_brake_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_brake_input.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_brake_input.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data = InputData::new_from_bytes(&rkg_data[0x88..rkg_data.len() - 0x04])
        .expect("Failed to read input data");
    assert!(input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn test_nine_laps() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/9laps_test.rkg")
        .expect("Couldn't find `./test_ghosts/9laps_test.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let header = Header::new_from_bytes(&rkg_data[..0x88]).expect("Couldn't read header");

    for (index, lap) in header.lap_split_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, lap.to_string());
    }

    println!("\nTotal time: {}", header.finish_time().to_string());
}

#[test]
fn test_exact_finish_time() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/00m58s6479888 David .rkg")
        .expect("Couldn't find `./test_ghosts/00m58s6479888 David .rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    assert_eq!(
        ctgp_metadata.exact_finish_time().to_string(),
        "00:58.647988872949"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[0].to_string(),
        "00:19.607006953895"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[1].to_string(),
        "00:19.623577742219"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[2].to_string(),
        "00:19.417404176835"
    );
}

#[test]
fn test_recompressed_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let original_data = &rkg_data[0x88..rkg_data.len() - 0xE0];
    let decompressed_data = yaz1_decompress(&original_data[0x04..]).unwrap();
    let recompressed_data = yaz1_compress(&decompressed_data);

    let original_input_data = InputData::new_from_bytes(&original_data).unwrap();
    let recompressed_input_data = InputData::new_from_bytes(&recompressed_data).unwrap();

    assert_eq!(
        (&original_data.len() - 12) as u32,
        u32::from_be_bytes([
            original_data[0],
            original_data[1],
            original_data[2],
            original_data[3]
        ])
    );
    assert_eq!(
        (recompressed_data.len() - 4) as u32,
        u32::from_be_bytes([
            recompressed_data[0],
            recompressed_data[1],
            recompressed_data[2],
            recompressed_data[3]
        ])
    );
    assert_eq!(
        original_input_data.controller_inputs(),
        recompressed_input_data.controller_inputs()
    );
    assert!(original_input_data.controller_inputs().len() > 100);
}

#[test]
fn test_full_ghost() {
    let ghost =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 3);
    assert_eq!(ghost.header().finish_time().milliseconds(), 904);
    assert_eq!(ghost.header().finish_time().to_string(), "01:03.904");
    assert_eq!(ghost.header().slot_id(), SlotId::LuigiCircuit);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(ghost.header().combo().character(), Character::KingBoo);
    assert_eq!(ghost.header().date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Classic);
    assert!(ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::ExpertStaff);
    assert!(ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(ghost.header().location().country(), Country::NotSet);
    assert_eq!(ghost.header().location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(1));
    assert_eq!(ghost.header().mii().birthday().day(), Some(1));
    assert_eq!(
        ghost.header().mii().favorite_color(),
        FavoriteColor::ForestGreen
    );
    assert_eq!(ghost.header().mii().name(), "JC");
    assert_eq!(ghost.header().mii().build().height(), 127);
    assert_eq!(ghost.header().mii().build().weight(), 127);

    assert_eq!(ghost.header().mii().mii_id(), 0x893EF2FB);
    assert_eq!(ghost.header().mii().system_id(), 0x689EC992);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Large);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::None
    );

    assert!(!ghost.header().mii().is_mingle_enabled());
    assert!(!ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalLong
    );
    assert_eq!(
        ghost.header().mii().hair().hair_color(),
        HairColor::PhilippineBrown
    );
    assert!(!ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::None
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 5);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 4);
    assert_eq!(ghost.header().mii().eyebrows().y(), 10);
    assert_eq!(ghost.header().mii().eyebrows().x(), 2);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(ghost.header().mii().eyes().rotation(), 4);
    assert_eq!(ghost.header().mii().eyes().y(), 9);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(ghost.header().mii().eyes().size(), 6);
    assert_eq!(ghost.header().mii().eyes().x(), 1);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(ghost.header().mii().nose().size(), 0);
    assert_eq!(ghost.header().mii().nose().y(), 8);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(ghost.header().mii().lips().size(), 7);
    assert_eq!(ghost.header().mii().lips().y(), 6);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::None
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Black
    );
    assert_eq!(ghost.header().mii().glasses().size(), 4);
    assert_eq!(ghost.header().mii().glasses().y(), 10);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::None
    );
    assert_eq!(ghost.header().mii().facial_hair().color(), HairColor::Black);
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 4);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 10);

    assert!(!ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 4);
    assert_eq!(ghost.header().mii().mole().y(), 20);
    assert_eq!(ghost.header().mii().mole().x(), 2);

    assert_eq!(ghost.header().mii().creator_name(), "JC");

    assert_eq!(ghost.header().mii_crc16(), 0x06F4);

    // Input data
    assert_eq!(ghost.input_data().face_button_input_count(), 0x18);
    assert_eq!(ghost.input_data().stick_input_count(), 0x037B);
    assert_eq!(ghost.input_data().dpad_button_input_count(), 0x09);
    assert_eq!(ghost.input_data().controller_inputs().len(), 907);

    assert!(!ghost.input_data().contains_illegal_brake_or_drift_inputs());

    for controller_input in ghost.input_data().controller_inputs() {
        assert!(
            !controller_input
                .stick()
                .is_impossible(ghost.header().controller())
        );
    }

    // CTGP Metadata
    assert!(ghost.footer().is_some() && ghost.footer().unwrap().is_ctgp());
}

#[test]
fn write_to_ghost() {
    let mut ghost =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 3);
    assert_eq!(ghost.header().finish_time().milliseconds(), 904);
    assert_eq!(ghost.header().finish_time().to_string(), "01:03.904");
    assert_eq!(ghost.header().slot_id(), SlotId::LuigiCircuit);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(ghost.header().combo().character(), Character::KingBoo);
    assert_eq!(ghost.header().date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Classic);
    assert!(ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::ExpertStaff);
    assert!(ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(ghost.header().location().country(), Country::NotSet);
    assert_eq!(ghost.header().location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(1));
    assert_eq!(ghost.header().mii().birthday().day(), Some(1));
    assert_eq!(
        ghost.header().mii().favorite_color(),
        FavoriteColor::ForestGreen
    );
    assert_eq!(ghost.header().mii().name(), "JC");
    assert_eq!(ghost.header().mii().build().height(), 127);
    assert_eq!(ghost.header().mii().build().weight(), 127);

    assert_eq!(ghost.header().mii().mii_id(), 0x893EF2FB);
    assert_eq!(ghost.header().mii().system_id(), 0x689EC992);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Large);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::None
    );

    assert!(!ghost.header().mii().is_mingle_enabled());
    assert!(!ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalLong
    );
    assert_eq!(
        ghost.header().mii().hair().hair_color(),
        HairColor::PhilippineBrown
    );
    assert!(!ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::None
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 5);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 4);
    assert_eq!(ghost.header().mii().eyebrows().y(), 10);
    assert_eq!(ghost.header().mii().eyebrows().x(), 2);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(ghost.header().mii().eyes().rotation(), 4);
    assert_eq!(ghost.header().mii().eyes().y(), 9);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(ghost.header().mii().eyes().size(), 6);
    assert_eq!(ghost.header().mii().eyes().x(), 1);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(ghost.header().mii().nose().size(), 0);
    assert_eq!(ghost.header().mii().nose().y(), 8);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(ghost.header().mii().lips().size(), 7);
    assert_eq!(ghost.header().mii().lips().y(), 6);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::None
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Black
    );
    assert_eq!(ghost.header().mii().glasses().size(), 4);
    assert_eq!(ghost.header().mii().glasses().y(), 10);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::None
    );
    assert_eq!(ghost.header().mii().facial_hair().color(), HairColor::Black);
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 4);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 10);

    assert!(!ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 4);
    assert_eq!(ghost.header().mii().mole().y(), 20);
    assert_eq!(ghost.header().mii().mole().x(), 2);

    assert_eq!(ghost.header().mii().creator_name(), "JC");

    assert_eq!(ghost.header().mii_crc16(), 0x06F4);

    // modify fields
    ghost
        .header_mut()
        .set_finish_time(InGameTime::new(1, 37, 999).unwrap());
    ghost.header_mut().set_slot_id(SlotId::DryDryRuins);
    ghost
        .header_mut()
        .set_combo(Combo::new(Vehicle::BlueFalcon, Character::Toad).unwrap());
    ghost
        .header_mut()
        .set_date_set(Date::new(2018, 12, 25).unwrap());
    ghost.header_mut().set_controller(Controller::Gamecube);
    ghost.set_input_data_compressed(false);
    ghost.header_mut().set_ghost_type(GhostType::Friend23);
    ghost.header_mut().set_automatic_drift(false);
    ghost
        .header_mut()
        .set_lap_split_time(0, InGameTime::new(0, 6, 741).unwrap());
    ghost
        .header_mut()
        .set_lap_split_time(1, InGameTime::new(0, 42, 069).unwrap());
    ghost
        .header_mut()
        .set_lap_split_time(2, InGameTime::new(0, 21, 910).unwrap());
    ghost.header_mut().set_location(
        Location::find(
            u8::from(Country::UnitedStates),
            u8::from(Subregion::UnitedStates(UnitedStatesSubregion::California)),
            None,
        )
        .unwrap(),
    );

    let mii = ghost.header_mut().mii_mut();

    mii.set_is_girl(true);
    mii.set_birthday(Birthday::new(4, 20).unwrap());
    mii.set_favorite_color(FavoriteColor::Red);
    mii.set_name("DUMBASS");
    mii.set_build(Build::new(100, 50).unwrap());
    mii.set_mii_type(MiiType::Foreign);
    mii.set_creation_date(
        NaiveDateTime::parse_from_str("2023-10-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    mii.set_system_id(0x453AE846);
    mii.set_head(Head::new(
        HeadShape::Rounded,
        SkinTone::Honey,
        FaceFeatures::Freckles,
    ));
    mii.set_mingle_enabled(true);
    mii.set_downloaded(true);
    mii.set_hair(Hair::new(HairType::NormalMedium, HairColor::Black, true));
    mii.set_eyebrows(
        Eyebrows::new(4, 3, 1, 9, HairColor::Black, EyebrowType::RoundedMedium).unwrap(),
    );
    mii.set_eyes(Eyes::new(3, 5, 0, 8, EyeColor::Brown, EyeType::Normal).unwrap());
    mii.set_nose(Nose::new(7, 1, NoseType::Normal).unwrap());
    mii.set_lips(Lips::new(5, 6, LipsType::Neutral, LipsColor::Red).unwrap());
    mii.set_glasses(Glasses::new(9, 3, GlassesType::Square, GlassesColor::Brown).unwrap());
    mii.set_facial_hair(
        FacialHair::new(
            BeardType::GoateeLong,
            MustacheType::Pencil,
            HairColor::Walnut,
            3,
            9,
        )
        .unwrap(),
    );
    mii.set_mole(Mole::new(true, 3, 19, 3).unwrap());

    mii.set_creator_name("IDIOT");

    // new ghost asserts
    ghost.update_ghost_sha1().unwrap();
    let mut ghost = Ghost::new_from_bytes(&ghost.raw_data()).unwrap();

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 37);
    assert_eq!(ghost.header().finish_time().milliseconds(), 999);
    assert_eq!(ghost.header().finish_time().to_string(), "01:37.999");
    assert_eq!(ghost.header().slot_id(), SlotId::DryDryRuins);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::BlueFalcon);
    assert_eq!(ghost.header().combo().character(), Character::Toad);
    assert_eq!(ghost.header().date_set(), &Date::new(2018, 12, 25).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Gamecube);
    assert!(!ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::Friend23);
    assert!(!ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:06.741");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:42.069");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:21.910");
    assert_eq!(ghost.header().location().country(), Country::UnitedStates);
    assert_eq!(
        ghost.header().location().subregion(),
        Subregion::UnitedStates(UnitedStatesSubregion::California)
    );

    // Mii Data
    assert!(ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(4));
    assert_eq!(ghost.header().mii().birthday().day(), Some(20));
    assert_eq!(ghost.header().mii().favorite_color(), FavoriteColor::Red);
    assert_eq!(ghost.header().mii().name(), "DUMBASS");
    assert_eq!(ghost.header().mii().build().height(), 100);
    assert_eq!(ghost.header().mii().build().weight(), 50);

    // assert_eq!(ghost.header().mii().mii_id(), 0xB00B1355);
    assert_eq!(ghost.header().mii().mii_type(), MiiType::Foreign);
    assert_eq!(
        ghost.header().mii().creation_date(),
        NaiveDateTime::parse_from_str("2023-10-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    );
    assert_eq!(ghost.header().mii().system_id(), 0x453AE846);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Rounded);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Honey);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::Freckles
    );

    assert!(ghost.header().mii().is_mingle_enabled());
    assert!(ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalMedium
    );
    assert_eq!(ghost.header().mii().hair().hair_color(), HairColor::Black);
    assert!(ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::RoundedMedium
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 4);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Black
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 3);
    assert_eq!(ghost.header().mii().eyebrows().y(), 9);
    assert_eq!(ghost.header().mii().eyebrows().x(), 1);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::Normal);
    assert_eq!(ghost.header().mii().eyes().rotation(), 3);
    assert_eq!(ghost.header().mii().eyes().y(), 8);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Brown);
    assert_eq!(ghost.header().mii().eyes().size(), 5);
    assert_eq!(ghost.header().mii().eyes().x(), 0);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Normal);
    assert_eq!(ghost.header().mii().nose().size(), 1);
    assert_eq!(ghost.header().mii().nose().y(), 7);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::Neutral);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Red);
    assert_eq!(ghost.header().mii().lips().size(), 6);
    assert_eq!(ghost.header().mii().lips().y(), 5);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::Square
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Brown
    );
    assert_eq!(ghost.header().mii().glasses().size(), 3);
    assert_eq!(ghost.header().mii().glasses().y(), 9);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::Pencil
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::GoateeLong
    );
    assert_eq!(
        ghost.header().mii().facial_hair().color(),
        HairColor::Walnut
    );
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 3);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 9);

    assert!(ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 3);
    assert_eq!(ghost.header().mii().mole().y(), 19);
    assert_eq!(ghost.header().mii().mole().x(), 3);

    assert_eq!(ghost.header().mii().creator_name(), "IDIOT");

    let _ = ghost
        .save_to_file("./test_ghosts/JC_transformed_ghost.rkg")
        .unwrap();
}

// This test requires a "ctgp_ghost_collection" folder not included in the rkg-utils repository, as it's 6.5k ghost files.
// Downloadable here: https://drive.google.com/file/d/1g-aY0mcBcMq9Zse0dkQEmZqHxV_UhmXM/view?usp=sharing
/*
#[test]
fn bulk_ghost_collection() {
    for entry in std::fs::read_dir("./test_ghosts/ctgp_ghost_collection").unwrap() {
        let ghost = Ghost::new_from_file(entry.as_ref().unwrap().path())
            .expect(format!("Failed on ghost {:?}", entry.as_ref().unwrap().file_name()).as_str());
        assert!(ghost.verify_base_crc32());
        assert!(ghost.verify_file_crc32());
    }
}
*/

#[test]
fn test_compare_saved_ghost() {
    let mut ghost1 =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    ghost1
        .save_to_file("./test_ghosts/JC_LC_Compressed_Resave.rkg")
        .expect("Failed to save ghost");

    let ghost2 = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed_Resave.rkg")
        .expect("Failed to read ghost");

    assert_eq!(ghost1.header().raw_data(), ghost2.header().raw_data());
    assert_eq!(
        ghost1.input_data().raw_data(),
        ghost2.input_data().raw_data()
    );
    assert_eq!(ghost1.file_crc32(), ghost2.file_crc32());
    assert_eq!(
        ghost1.footer().as_ref().unwrap().raw_data(),
        ghost2.footer().as_ref().unwrap().raw_data()
    );
}

#[test]
fn test_sp_footer() {
    let ghost = Ghost::new_from_file("./test_ghosts/spv5.rkg").unwrap();

    let sp_footer = if let Some(FooterType::SPFooter(sp_footer)) = ghost.footer() {
        sp_footer
    } else {
        panic!("SP Footer not found")
    };

    println!("SP footer version: {}", sp_footer.footer_version());
    print!("Possible MKW-SP release versions: ");
    if let Some(sp_versions) = sp_footer.possible_sp_versions() {
        for version in sp_versions {
            print!("{}, ", version)
        }
        println!();
    } else {
        println!("Unknown")
    }

    print!("Track SHA1: ");
    for byte in sp_footer.track_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    for (index, time) in sp_footer.exact_lap_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, time);
    }
    println!("Total: {}", sp_footer.exact_finish_time());
    println!("Has speed mod? {}", sp_footer.has_speed_mod());
    println!("Has ultra shortcut? {}", sp_footer.has_ultra_shortcut());
    println!(
        "Has horizontal wall glitch? {}",
        sp_footer.has_horizontal_wall_glitch()
    );
    println!("Has wallride? {}", sp_footer.has_wallride());

    if let Some(shroomstrat_string) = sp_footer.shroomstrat_string() {
        println!("Shroomstrat: {}", shroomstrat_string);
    }

    if let Some(is_vanilla) = sp_footer.is_vanilla_mode_enabled() {
        println!("Vanilla mode enabled? {}", is_vanilla);
    }

    if let Some(simplified_controls) = sp_footer.has_simplified_controls() {
        println!("Has simplified controls? {}", simplified_controls);
    }

    if let Some(set_in_mirror) = sp_footer.set_in_mirror() {
        println!("Set in mirror mode? {}", set_in_mirror);
    }
}

#[test]
fn south_korea_ghost_test() {
    let ghost = Ghost::new_from_file("./test_ghosts/ghost2_comp_00.rkg").unwrap();
    assert_eq!(ghost.header().location().country(), Country::SouthKorea);
}

#[test]
fn mii_file_test() {
    let _ = Mii::new_from_file("./test_ghosts/4TBMrBean.miigx").unwrap();
}

#[test]
fn input_at_frame_test() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();

    let frame = 256;
    println!("Input at frame {}:", frame);
    println!("{:#?}", ghost.input_data().get_input_at_frame(frame))
}

/*
#[test]
fn ninrankings_ghost_collection() {
    for entry in std::fs::read_dir("./test_ghosts/ninrankings_ghost_collection").unwrap() {
        let ghost = Ghost::new_from_file(entry.as_ref().unwrap().path());

        match ghost {
            Err(e) => {
                println!(
                    "\nFailed on file {}",
                    entry.as_ref().unwrap().file_name().to_str().unwrap()
                );
                println!("Error: {e}");
            }
            _ => (),
        }
    }
}
*/

// In depth tests
// ===== CRC Tests =====

#[test]
fn crc16_known_vector() {
    // CRC-16 CCITT XModem: "123456789" = 0x31C3
    assert_eq!(crc16(b"123456789"), 0x31C3);
}

#[test]
fn crc16_empty() {
    assert_eq!(crc16(b""), 0x0000);
}

#[test]
fn crc16_single_byte() {
    // deterministic, same input should always be same output
    let a = crc16(b"A");
    let b = crc16(b"A");
    assert_eq!(a, b);
    assert_ne!(a, crc16(b"B"));
}

#[test]
fn crc32_known_vector() {
    // Standard CRC-32: "123456789" = 0xCBF43926
    assert_eq!(crc32(b"123456789"), 0xCBF43926);
}

#[test]
fn crc32_empty() {
    assert_eq!(crc32(b""), 0x00000000);
}

#[test]
fn crc32_deterministic() {
    let a = crc32(b"MarioKartWii");
    let b = crc32(b"MarioKartWii");
    assert_eq!(a, b);
    assert_ne!(a, crc32(b"MarioKartWi"));
}

// ===== write_bits Tests =====

#[test]
fn write_bits_full_byte() {
    let mut buf = [0u8; 1];
    write_bits(&mut buf, 0, 0, 8, 0xAB);
    assert_eq!(buf[0], 0xAB);
}

#[test]
fn write_bits_upper_nibble() {
    // bit_offset=0, width=4: writes into the upper nibble
    let mut buf = [0u8; 1];
    write_bits(&mut buf, 0, 0, 4, 0xF);
    assert_eq!(buf[0], 0xF0);
}

#[test]
fn write_bits_lower_nibble() {
    // bit_offset=4, width=4: writes into the lower nibble
    let mut buf = [0u8; 1];
    write_bits(&mut buf, 0, 4, 4, 0xF);
    assert_eq!(buf[0], 0x0F);
}

#[test]
fn write_bits_cross_byte() {
    // 8 bits starting at bit 4 of byte 0 spans into byte 1
    let mut buf = [0u8; 2];
    write_bits(&mut buf, 0, 4, 8, 0xAB);
    assert_eq!(buf[0], 0x0A);
    assert_eq!(buf[1], 0xB0);
}

#[test]
fn write_bits_preserves_surrounding() {
    // Upper nibble should not be disturbed when writing to lower nibble
    let mut buf = [0xF0u8; 1];
    write_bits(&mut buf, 0, 4, 4, 0x0);
    assert_eq!(buf[0], 0xF0);
}

#[test]
fn write_bits_overwrites_with_zero() {
    let mut buf = [0xFFu8; 1];
    write_bits(&mut buf, 0, 0, 4, 0x0);
    assert_eq!(buf[0], 0x0F); // upper nibble cleared, lower preserved
}

// ===== InGameTime Tests =====

#[test]
fn in_game_time_new_valid() {
    let t = InGameTime::new(1, 3, 904).unwrap();
    assert_eq!(t.minutes(), 1);
    assert_eq!(t.seconds(), 3);
    assert_eq!(t.milliseconds(), 904);
}

#[test]
fn in_game_time_new_zero() {
    let t = InGameTime::new(0, 0, 0).unwrap();
    assert_eq!(t.igt_to_millis(), 0);
}

#[test]
fn in_game_time_new_max_storable() {
    assert!(InGameTime::new(127, 127, 1023).is_ok());
}

#[test]
fn in_game_time_minutes_overflow() {
    assert!(matches!(
        InGameTime::new(128, 0, 0),
        Err(InGameTimeError::InGameTimeElementTooLarge)
    ));
}

#[test]
fn in_game_time_seconds_overflow() {
    assert!(matches!(
        InGameTime::new(0, 128, 0),
        Err(InGameTimeError::InGameTimeElementTooLarge)
    ));
}

#[test]
fn in_game_time_milliseconds_overflow() {
    assert!(matches!(
        InGameTime::new(0, 0, 1024),
        Err(InGameTimeError::InGameTimeElementTooLarge)
    ));
}

#[test]
fn in_game_time_is_valid_in_bounds() {
    assert!(InGameTime::new(99, 59, 999).unwrap().is_valid());
    assert!(InGameTime::new(0, 0, 0).unwrap().is_valid());
}

#[test]
fn in_game_time_is_valid_out_of_bounds() {
    // Storable but not semantically valid in-game
    assert!(!InGameTime::new(100, 0, 0).unwrap().is_valid());
    assert!(!InGameTime::new(0, 60, 0).unwrap().is_valid());
    assert!(!InGameTime::new(0, 0, 1000).unwrap().is_valid());
}

#[test]
fn in_game_time_default_is_valid() {
    assert!(InGameTime::default().is_valid());
}

#[test]
fn in_game_time_display() {
    assert_eq!(InGameTime::new(1, 3, 904).unwrap().to_string(), "01:03.904");
    assert_eq!(InGameTime::new(0, 0, 0).unwrap().to_string(), "00:00.000");
    assert_eq!(
        InGameTime::new(99, 59, 999).unwrap().to_string(),
        "99:59.999"
    );
}

#[test]
fn in_game_time_add_no_carry() {
    let a = InGameTime::new(0, 25, 540).unwrap();
    let b = InGameTime::new(0, 19, 127).unwrap();
    let sum = a + b;
    assert_eq!(sum.minutes(), 0);
    assert_eq!(sum.seconds(), 44);
    assert_eq!(sum.milliseconds(), 667);
}

#[test]
fn in_game_time_add_with_carry() {
    let a = InGameTime::new(0, 59, 600).unwrap();
    let b = InGameTime::new(0, 0, 500).unwrap();
    let sum = a + b;
    assert_eq!(sum.minutes(), 1);
    assert_eq!(sum.seconds(), 0);
    assert_eq!(sum.milliseconds(), 100);
}

#[test]
fn in_game_time_sum_lap_splits() {
    let laps = [
        InGameTime::new(0, 25, 540).unwrap(),
        InGameTime::new(0, 19, 127).unwrap(),
        InGameTime::new(0, 19, 237).unwrap(),
    ];
    let total: InGameTime = laps.iter().copied().sum();
    // 25540 + 19127 + 19237 = 63904 ms = 1:03.904
    assert_eq!(total.to_string(), "01:03.904");
}

#[test]
fn in_game_time_igt_to_millis() {
    assert_eq!(InGameTime::new(1, 3, 904).unwrap().igt_to_millis(), 63904);
    assert_eq!(InGameTime::new(0, 0, 0).unwrap().igt_to_millis(), 0);
}

#[test]
fn in_game_time_from_milliseconds_persists() {
    let original = InGameTime::new(1, 23, 456).unwrap();
    let millis = original.igt_to_millis();
    let reconstructed = InGameTime::from_milliseconds(millis).unwrap();
    assert_eq!(reconstructed.minutes(), 1);
    assert_eq!(reconstructed.seconds(), 23);
    assert_eq!(reconstructed.milliseconds(), 456);
}

#[test]
fn in_game_time_raw_bytes_persists() {
    let original = InGameTime::new(1, 23, 456).unwrap();
    let bytes: [u8; 3] = original.into();
    // Re-encode via write_bits approach and check the 3-byte layout
    // minutes(7) + seconds(7) + ms(10) = 24 bits = 3 bytes
    let minutes = (bytes[0] >> 1) as u8;
    let seconds_high = bytes[0] & 0x01;
    let seconds = ((seconds_high << 6) | (bytes[1] >> 2)) as u8;
    let ms = (((bytes[1] & 0x03) as u16) << 8) | bytes[2] as u16;
    assert_eq!(minutes, 1);
    assert_eq!(seconds, 23);
    assert_eq!(ms, 456);
}

// ===== Date Tests =====

#[test]
fn date_new_valid() {
    let d = Date::new(2025, 11, 12).unwrap();
    assert_eq!(d.year(), 2025);
    assert_eq!(d.month(), 11);
    assert_eq!(d.day(), 12);
}

#[test]
fn date_year_boundary_min() {
    assert!(Date::new(2000, 1, 1).is_ok());
}

#[test]
fn date_year_boundary_max() {
    assert!(Date::new(2035, 12, 31).is_ok());
}

#[test]
fn date_year_too_low() {
    assert!(matches!(Date::new(1999, 1, 1), Err(DateError::YearInvalid)));
}

#[test]
fn date_year_too_high() {
    assert!(matches!(Date::new(2036, 1, 1), Err(DateError::YearInvalid)));
}

#[test]
fn date_month_zero() {
    assert!(matches!(
        Date::new(2025, 0, 1),
        Err(DateError::MonthInvalid)
    ));
}

#[test]
fn date_month_thirteen() {
    assert!(matches!(
        Date::new(2025, 13, 1),
        Err(DateError::MonthInvalid)
    ));
}

#[test]
fn date_day_31_valid_month() {
    assert!(Date::new(2025, 1, 31).is_ok());
    assert!(Date::new(2025, 3, 31).is_ok());
}

#[test]
fn date_day_31_invalid_month() {
    assert!(matches!(Date::new(2025, 4, 31), Err(DateError::DayInvalid)));
    assert!(matches!(Date::new(2025, 6, 31), Err(DateError::DayInvalid)));
    assert!(matches!(
        Date::new(2025, 11, 31),
        Err(DateError::DayInvalid)
    ));
}

#[test]
fn date_february_leap_year() {
    // 2024 - 2000 = 24, 24 % 4 == 0: leap year
    assert!(Date::new(2024, 2, 29).is_ok());
    assert!(matches!(Date::new(2024, 2, 30), Err(DateError::DayInvalid)));
}

#[test]
fn date_february_non_leap_year() {
    // 2025 - 2000 = 25, 25 % 4 != 0: not a leap year
    assert!(Date::new(2025, 2, 28).is_ok());
    assert!(matches!(Date::new(2025, 2, 29), Err(DateError::DayInvalid)));
}

#[test]
fn date_year_2000_leap() {
    // 2000 - 2000 = 0, 0 % 4 == 0 && 2000 % 400 == 0: leap year
    assert!(Date::new(2000, 2, 29).is_ok());
}

#[test]
fn date_display() {
    assert_eq!(Date::new(2025, 11, 12).unwrap().to_string(), "2025-11-12");
    assert_eq!(Date::new(2000, 1, 1).unwrap().to_string(), "2000-01-01");
}

#[test]
fn date_equality() {
    assert_eq!(
        Date::new(2025, 11, 12).unwrap(),
        Date::new(2025, 11, 12).unwrap()
    );
    assert_ne!(
        Date::new(2025, 11, 12).unwrap(),
        Date::new(2025, 11, 13).unwrap()
    );
}

// ===== SlotId Tests =====

#[test]
fn slot_id_luigi_circuit() {
    let id = SlotId::LuigiCircuit;
    assert_eq!(SlotId::try_from(u8::from(id)).unwrap(), id);
}

#[test]
fn slot_id_mario_circuit() {
    assert_eq!(u8::from(SlotId::MarioCircuit), 0x00);
    assert_eq!(SlotId::try_from(0x00u8).unwrap(), SlotId::MarioCircuit);
}

#[test]
fn slot_id_galaxy_colosseum() {
    assert_eq!(u8::from(SlotId::GalaxyColosseum), 0xC9);
    assert_eq!(SlotId::try_from(0xC9u8).unwrap(), SlotId::GalaxyColosseum);
}

#[test]
fn slot_id_all_battle_arenas() {
    let arenas = [
        SlotId::BlockPlaza,
        SlotId::DelfinoPier,
        SlotId::FunkyStadium,
        SlotId::ChainChompWheel,
        SlotId::ThwompDesert,
    ];
    for slot in arenas {
        assert_eq!(SlotId::try_from(u8::from(slot)).unwrap(), slot);
    }
}

#[test]
fn slot_id_invalid_byte() {
    assert!(matches!(
        SlotId::try_from(0xFF),
        Err(SlotIdError::NonExistentSlotId)
    ));
    assert!(SlotId::try_from(0x99).is_err());
}

#[test]
fn slot_id_display() {
    assert_eq!(SlotId::LuigiCircuit.to_string(), "Luigi Circuit");
    assert_eq!(SlotId::MapleTreeway.to_string(), "Maple Treeway");
    assert_eq!(SlotId::GCNDKMountain.to_string(), "GCN DK Mountain");
    assert_eq!(SlotId::GalaxyColosseum.to_string(), "Galaxy Colosseum");
}

// ===== Controller Tests =====

#[test]
fn controller_all() {
    for (byte, ctrl) in [
        (0u8, Controller::WiiWheel),
        (1, Controller::Nunchuk),
        (2, Controller::Classic),
        (3, Controller::Gamecube),
    ] {
        assert_eq!(u8::from(ctrl), byte);
        assert_eq!(Controller::try_from(byte).unwrap(), ctrl);
    }
}

#[test]
fn controller_invalid_byte() {
    assert!(Controller::try_from(4).is_err());
    assert!(Controller::try_from(255).is_err());
}

#[test]
fn controller_display() {
    assert_eq!(Controller::WiiWheel.to_string(), "Wii Wheel");
    assert_eq!(Controller::Nunchuk.to_string(), "Nunchuk");
    assert_eq!(Controller::Classic.to_string(), "Classic");
    assert_eq!(Controller::Gamecube.to_string(), "Gamecube");
}

// ===== GhostType Tests =====

#[test]
fn ghost_type_player_best() {
    assert_eq!(u8::from(GhostType::PlayerBest), 0x01);
    assert_eq!(GhostType::try_from(0x01u8).unwrap(), GhostType::PlayerBest);
}

#[test]
fn ghost_type_expert_staff() {
    assert_eq!(u8::from(GhostType::ExpertStaff), 0x26);
    assert_eq!(GhostType::try_from(0x26u8).unwrap(), GhostType::ExpertStaff);
}

#[test]
fn ghost_type_friend() {
    // Friend1 = 0x07, Friend30 = 0x24
    assert_eq!(u8::from(GhostType::Friend1), 0x07);
    assert_eq!(GhostType::try_from(0x07u8).unwrap(), GhostType::Friend1);
    assert_eq!(u8::from(GhostType::Friend30), 0x24);
    assert_eq!(GhostType::try_from(0x24u8).unwrap(), GhostType::Friend30);
}

#[test]
fn ghost_type_zero_invalid() {
    assert!(matches!(
        GhostType::try_from(0x00u8),
        Err(GhostTypeError::NonexistentGhostType)
    ));
}

#[test]
fn ghost_type_out_of_range() {
    assert!(GhostType::try_from(0x27u8).is_err());
    assert!(GhostType::try_from(0xFFu8).is_err());
}

// ===== TransmissionMod Tests =====

#[test]
fn transmission_mod_all() {
    for (byte, tm) in [
        (0u8, TransmissionMod::Vanilla),
        (1, TransmissionMod::AllInside),
        (2, TransmissionMod::AllBikeInside),
        (3, TransmissionMod::AllOutside),
    ] {
        assert_eq!(u8::from(tm), byte);
        assert_eq!(TransmissionMod::try_from(byte).unwrap(), tm);
    }
}

#[test]
fn transmission_mod_invalid_byte() {
    assert!(TransmissionMod::try_from(4).is_err());
}

#[test]
fn transmission_mod_display() {
    assert_eq!(TransmissionMod::Vanilla.to_string(), "Vanilla");
    assert_eq!(TransmissionMod::AllInside.to_string(), "All Inside");
    assert_eq!(
        TransmissionMod::AllBikeInside.to_string(),
        "All Bikes Inside"
    );
    assert_eq!(TransmissionMod::AllOutside.to_string(), "All Outside");
}

// ===== Combo Tests =====

#[test]
fn combo_valid_heavy() {
    let combo = Combo::new(Vehicle::WarioBike, Character::KingBoo).unwrap();
    assert_eq!(combo.character(), Character::KingBoo);
    assert_eq!(combo.vehicle(), Vehicle::WarioBike);
}

#[test]
fn combo_valid_small() {
    assert!(Combo::new(Vehicle::BulletBike, Character::BabyMario).is_ok());
}

#[test]
fn combo_valid_medium() {
    assert!(Combo::new(Vehicle::MachBike, Character::Mario).is_ok());
}

#[test]
fn combo_incongruent_heavy_vehicle_medium_character() {
    assert!(matches!(
        Combo::new(Vehicle::WarioBike, Character::Mario),
        Err(ComboError::IncongruentWeightClasses)
    ));
}

#[test]
fn combo_incongruent_small_vehicle_heavy_character() {
    assert!(matches!(
        Combo::new(Vehicle::BulletBike, Character::KingBoo),
        Err(ComboError::IncongruentWeightClasses)
    ));
}

#[test]
fn combo_display() {
    let combo = Combo::new(Vehicle::WarioBike, Character::KingBoo).unwrap();
    assert_eq!(combo.to_string(), "King Boo on Wario Bike");
}

// ===== StickInput Tests =====

#[test]
fn stick_input_new_valid() {
    let s = StickInput::new(7, 7).unwrap();
    assert_eq!(s.x(), 7);
    assert_eq!(s.y(), 7);
}

#[test]
fn stick_input_boundary() {
    assert!(StickInput::new(0, 0).is_ok());
    assert!(StickInput::new(14, 14).is_ok());
}

#[test]
fn stick_input_x_too_large() {
    assert!(matches!(
        StickInput::new(15, 7),
        Err(StickInputError::InvalidStickInput)
    ));
}

#[test]
fn stick_input_y_too_large() {
    assert!(matches!(
        StickInput::new(7, 15),
        Err(StickInputError::InvalidStickInput)
    ));
}

#[test]
fn stick_input_try_from_byte_persists() {
    let original = StickInput::new(5, 9).unwrap();
    let byte = original.to_byte();
    let decoded = StickInput::try_from(byte).unwrap();
    assert_eq!(decoded.x(), 5);
    assert_eq!(decoded.y(), 9);
}

#[test]
fn stick_input_byte_encoding() {
    // x is high nibble, y is low nibble
    let s = StickInput::new(0xA, 0xB).unwrap();
    assert_eq!(s.to_byte(), 0xAB);
}

#[test]
fn stick_input_try_from_invalid_byte() {
    // 0xFF: x = 0xF = 15 > 14
    assert!(StickInput::try_from(0xFF).is_err());
}

#[test]
fn stick_input_center_never_impossible() {
    let center = StickInput::new(7, 7).unwrap();
    for ctrl in [
        Controller::WiiWheel,
        Controller::Nunchuk,
        Controller::Classic,
        Controller::Gamecube,
    ] {
        assert!(
            !center.is_impossible(ctrl),
            "Center should be legal for {ctrl}"
        );
    }
}

#[test]
fn stick_input_corner_impossible_nunchuk_classic_gcn() {
    // [0, 0] is in the first 24 illegal inputs (shared by all non-WiiWheel)
    let corner = StickInput::new(0, 0).unwrap();
    assert!(corner.is_impossible(Controller::Nunchuk));
    assert!(corner.is_impossible(Controller::Classic));
    assert!(corner.is_impossible(Controller::Gamecube));
    assert!(!corner.is_impossible(Controller::WiiWheel));
}

#[test]
fn stick_input_gcn_only_illegal() {
    // [0, 11] is in the GCN/Classic-only block (index 24+)
    let s = StickInput::new(0, 11).unwrap();
    assert!(!s.is_impossible(Controller::Nunchuk));
    assert!(s.is_impossible(Controller::Classic));
    assert!(s.is_impossible(Controller::Gamecube));
    assert!(!s.is_impossible(Controller::WiiWheel));
}

#[test]
fn stick_input_wii_wheel_never_impossible() {
    // WiiWheel has no illegal stick inputs at all
    let corner = StickInput::new(0, 14).unwrap();
    assert!(!corner.is_impossible(Controller::WiiWheel));
}

// ===== DPadButton Tests =====

#[test]
fn dpad_button_none() {
    assert_eq!(DPadButton::try_from(0x00u8).unwrap(), DPadButton::None);
}

#[test]
fn dpad_button_directions() {
    assert_eq!(DPadButton::try_from(0x10u8).unwrap(), DPadButton::Up);
    assert_eq!(DPadButton::try_from(0x20u8).unwrap(), DPadButton::Down);
    assert_eq!(DPadButton::try_from(0x30u8).unwrap(), DPadButton::Left);
    assert_eq!(DPadButton::try_from(0x40u8).unwrap(), DPadButton::Right);
}

#[test]
fn dpad_button_invalid() {
    // (0x50 & 0x70) >> 4 = 5 which is not a valid direction
    assert!(DPadButton::try_from(0x50u8).is_err());
}

// ===== ControllerInput Tests =====

#[test]
fn controller_input_new_and_getters() {
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        true,
        false,
        false,
        DPadButton::Up,
        stick,
        5,
    );
    assert!(input.accelerator());
    assert!(!input.brake());
    assert!(!input.brake_drift());
    assert!(input.drift_flag());
    assert!(!input.item());
    assert!(!input.unknown_face_button());
    assert_eq!(input.dpad(), DPadButton::Up);
    assert_eq!(input.stick(), stick);
    assert_eq!(input.frame_duration(), 5);
}

#[test]
fn controller_input_face_buttons_equal() {
    let stick = StickInput::new(7, 7).unwrap();
    let a = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        1,
    );
    let b = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::Up,
        stick,
        5,
    );
    assert!(a.face_buttons_equal_to(b)); // dpad and duration don't affect face equality
}

// ===== InputData Tests =====

#[test]
fn input_data_empty_error() {
    assert!(matches!(
        InputData::new(vec![], false),
        Err(InputDataError::InputDataLengthTooShort)
    ));
}

#[test]
fn input_data_single_input_uncompressed() {
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        100,
    );
    let data = InputData::new(vec![input], false).unwrap();
    assert_eq!(data.controller_inputs().len(), 1);
    assert!(!data.compressed());
    assert_eq!(data.face_button_input_count(), 1);
    assert_eq!(data.stick_input_count(), 1);
    assert_eq!(data.dpad_button_input_count(), 1);
}

#[test]
fn input_data_too_short_bytes() {
    assert!(matches!(
        InputData::new_from_bytes(&[0u8; 4]),
        Err(InputDataError::InputDataLengthTooShort)
    ));
}

#[test]
fn input_data_get_input_at_frame_zero_none() {
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        10,
    );
    let data = InputData::new(vec![input], false).unwrap();
    assert!(data.get_input_at_frame(0).is_none());
}

#[test]
fn input_data_get_input_at_frame_valid() {
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        10,
    );
    let data = InputData::new(vec![input], false).unwrap();
    assert!(data.get_input_at_frame(1).is_some());
    assert!(data.get_input_at_frame(10).is_some());
    assert!(data.get_input_at_frame(11).is_none());
}

#[test]
fn input_data_from_jc_lc_compressed() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let input_data = ghost.input_data();
    assert_eq!(input_data.face_button_input_count(), 0x18);
    assert_eq!(input_data.stick_input_count(), 0x037B);
    assert_eq!(input_data.dpad_button_input_count(), 0x09);
    assert_eq!(input_data.controller_inputs().len(), 907);
    assert!(input_data.compressed());
    assert!(!input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn input_data_illegal_brake_input() {
    let ghost = Ghost::new_from_file("./test_ghosts/illegal_brake_input.rkg").unwrap();
    assert!(ghost.input_data().contains_illegal_brake_or_drift_inputs());
}

#[test]
fn input_data_illegal_drift_input() {
    let ghost = Ghost::new_from_file("./test_ghosts/illegal_drift_input.rkg").unwrap();
    assert!(ghost.input_data().contains_illegal_brake_or_drift_inputs());
}

#[test]
fn input_data_raw_data_reparse() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let original_inputs = ghost.input_data().controller_inputs().to_vec();
    let raw = ghost.input_data().raw_data();
    let reparsed = InputData::new_from_bytes(&raw).unwrap();
    assert_eq!(reparsed.controller_inputs(), original_inputs.as_slice());
}

#[test]
fn input_data_set_compressed() {
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        false,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        50,
    );
    let mut data = InputData::new(vec![input], false).unwrap();
    assert!(!data.compressed());
    data.set_compressed(true);
    assert!(data.compressed());
}

// ===== FooterType Tests =====

#[test]
fn footer_type_unknown_raw_data() {
    let bytes = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let footer = FooterType::Unknown(bytes.clone());
    assert!(footer.is_unknown());
    assert!(!footer.is_ctgp());
    assert!(!footer.is_sp());
    assert_eq!(footer.raw_data(), bytes.as_slice());
}

#[test]
fn footer_type_ctgp_detected() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let footer = ghost.footer().unwrap();
    assert!(footer.is_ctgp());
    assert!(!footer.is_sp());
    assert!(!footer.is_unknown());
}

#[test]
fn footer_type_sp_detected() {
    let ghost = Ghost::new_from_file("./test_ghosts/spv5.rkg").unwrap();
    let footer = ghost.footer().unwrap();
    assert!(footer.is_sp());
    assert!(!footer.is_ctgp());
    assert!(!footer.is_unknown());
}

#[test]
fn footer_type_none_for_vanilla_ghost() {
    // JC_LC.rkg is the uncompressed vanilla ghost without a CTGP/SP footer
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC.rkg").unwrap();
    // It may or may not have a footer; just ensure it doesn't panic
    let f = ghost.footer();
    assert!(f.is_none());
}

// ===== Header Tests =====

#[test]
fn header_not_rkgd_error() {
    let mut bytes = [0u8; 0x88];
    bytes[0..4].copy_from_slice(b"XXXX");
    assert!(matches!(
        Header::new_from_bytes(&bytes),
        Err(HeaderError::NotRKGD)
    ));
}

#[test]
fn header_wrong_size_error() {
    let bytes = [0u8; 0x87];
    assert!(matches!(
        Header::new_from_bytes(&bytes),
        Err(HeaderError::NotCorrectSize)
    ));
}

#[test]
fn header_from_jc_lc_compressed() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert_eq!(header.finish_time().to_string(), "01:03.904");
    assert_eq!(header.slot_id(), SlotId::LuigiCircuit);
    assert_eq!(header.lap_count(), 3);
    assert!(header.is_compressed());
    assert!(header.is_automatic_drift());
    assert_eq!(header.controller(), Controller::Classic);
    assert_eq!(header.ghost_type(), GhostType::ExpertStaff);
    assert_eq!(header.transmission_mod(), TransmissionMod::Vanilla);
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.location().country(), Country::NotSet);
}

#[test]
fn header_raw_data_persists() {
    let mut raw = [0u8; 0x88];
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .unwrap()
        .read_exact(&mut raw)
        .unwrap();
    let header = Header::new_from_bytes(&raw).unwrap();
    assert_eq!(header.raw_data(), raw);
}

#[test]
fn header_mii_crc16_computed() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    // CRC16 of the embedded Mii raw data matches stored value
    assert_eq!(header.mii_crc16(), 0x06F4);
    // Calling twice gives same result (computed from mii, not stored)
    assert_eq!(header.mii_crc16(), header.mii_crc16());
}

#[test]
fn header_lap_split_times_count() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert_eq!(header.lap_split_times().len(), 3);
}

#[test]
fn header_lap_split_times_values() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert_eq!(header.lap_split_time(0).unwrap().to_string(), "00:25.540");
    assert_eq!(header.lap_split_time(1).unwrap().to_string(), "00:19.127");
    assert_eq!(header.lap_split_time(2).unwrap().to_string(), "00:19.237");
}

#[test]
fn header_lap_split_time_out_of_bounds() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert!(matches!(
        header.lap_split_time(3),
        Err(HeaderError::LapSplitIndexError)
    ));
}

#[test]
fn header_lap_splits_sum_to_finish_time() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let total: InGameTime = header.lap_split_times().iter().copied().sum();
    assert_eq!(total.igt_to_millis(), header.finish_time().igt_to_millis());
}

#[test]
fn header_9_laps() {
    let header = Header::new_from_path("./test_ghosts/9laps_test.rkg").unwrap();
    assert_eq!(header.lap_count(), 9);
    assert_eq!(header.lap_split_times().len(), 9);
}

#[test]
fn header_setter_finish_time_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let new_time = InGameTime::new(2, 0, 0).unwrap();
    header.set_finish_time(new_time);
    assert_eq!(header.finish_time().to_string(), "02:00.000");
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert_eq!(reparsed.finish_time().to_string(), "02:00.000");
}

#[test]
fn header_setter_slot_id_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    header.set_slot_id(SlotId::RainbowRoad);
    assert_eq!(header.slot_id(), SlotId::RainbowRoad);
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert_eq!(reparsed.slot_id(), SlotId::RainbowRoad);
}

#[test]
fn header_setter_controller_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    header.set_controller(Controller::WiiWheel);
    assert_eq!(header.controller(), Controller::WiiWheel);
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert_eq!(reparsed.controller(), Controller::WiiWheel);
}

#[test]
fn header_setter_ghost_type_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    header.set_ghost_type(GhostType::WorldRecord);
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert_eq!(reparsed.ghost_type(), GhostType::WorldRecord);
}

#[test]
fn header_setter_automatic_drift_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    header.set_automatic_drift(false);
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert!(!reparsed.is_automatic_drift());
}

#[test]
fn header_setter_lap_split_time_persists_in_raw() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let new_time = InGameTime::new(0, 20, 0).unwrap();
    header.set_lap_split_time(0, new_time);
    let reparsed = Header::new_from_bytes(&header.raw_data()).unwrap();
    assert_eq!(reparsed.lap_split_time(0).unwrap().to_string(), "00:20.000");
}

#[test]
fn header_set_lap_split_oob_is_noop() {
    let mut header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let original_time = header.lap_split_time(0).unwrap().igt_to_millis();
    header.set_lap_split_time(99, InGameTime::new(0, 1, 0).unwrap());
    assert_eq!(
        header.lap_split_time(0).unwrap().igt_to_millis(),
        original_time
    );
}

// ===== Ghost Tests =====

#[test]
fn ghost_too_short_error() {
    assert!(matches!(
        Ghost::new_from_bytes(&[0u8; 0x8F]),
        Err(GhostError::DataLengthTooShort)
    ));
}

#[test]
fn ghost_from_file_ctgp() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert!(ghost.footer().is_some());
    assert!(ghost.footer().unwrap().is_ctgp());
    assert!(ghost.should_preserve_external_footer());
}

#[test]
fn ghost_from_file_sp() {
    let ghost = Ghost::new_from_file("./test_ghosts/spv5.rkg").unwrap();
    assert!(ghost.footer().is_some());
    assert!(ghost.footer().unwrap().is_sp());
}

#[test]
fn ghost_new_no_footer() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        100,
    );
    let input_data = InputData::new(vec![input], false).unwrap();
    let ghost = Ghost::new(header, input_data);
    assert!(ghost.footer().is_none());
    assert!(!ghost.should_preserve_external_footer());
}

#[test]
fn ghost_new_syncs_compression_true() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        100,
    );
    let input_data = InputData::new(vec![input], true).unwrap();
    let ghost = Ghost::new(header, input_data);
    assert!(ghost.header().is_compressed());
}

#[test]
fn ghost_new_syncs_compression_false() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        false,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        100,
    );
    let input_data = InputData::new(vec![input], false).unwrap();
    let ghost = Ghost::new(header, input_data);
    assert!(!ghost.header().is_compressed());
}

#[test]
fn ghost_new_decompressed_length_correct() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let stick = StickInput::new(7, 7).unwrap();
    let input = ControllerInput::new(
        true,
        false,
        false,
        false,
        false,
        false,
        DPadButton::None,
        stick,
        100,
    );
    let input_data = InputData::new(vec![input], false).unwrap();

    let face = input_data.face_button_input_count();
    let stick_count = input_data.stick_input_count();
    let dpad = input_data.dpad_button_input_count();
    let expected_len = 8u16 + face * 2 + stick_count * 2 + dpad * 2;

    let ghost = Ghost::new(header, input_data);
    assert_eq!(
        ghost.header().decompressed_input_data_length(),
        expected_len
    );
}

#[test]
fn ghost_file_crc32_valid() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let raw = ghost.raw_data();
    let expected = crc32(&raw[..raw.len() - 4]);
    assert_eq!(ghost.file_crc32(), expected);
}

#[test]
fn ghost_base_crc32_is_prefix_crc() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let mut buf = Vec::from(ghost.header().raw_data());
    buf.extend_from_slice(&ghost.input_data().raw_data());
    assert_eq!(ghost.base_crc32(), crc32(&buf));
}

#[test]
fn ghost_raw_data_last_4_bytes_are_file_crc32() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let raw = ghost.raw_data();
    let stored = u32::from_be_bytes(raw[raw.len() - 4..].try_into().unwrap());
    assert_eq!(stored, ghost.file_crc32());
}

#[test]
fn ghost_raw_data_reparse_identical() {
    let ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let raw = ghost.raw_data();
    let reparsed = Ghost::new_from_bytes(&raw).unwrap();
    assert_eq!(reparsed.header().finish_time().to_string(), "01:03.904");
    assert_eq!(reparsed.header().slot_id(), SlotId::LuigiCircuit);
    assert_eq!(reparsed.header().lap_count(), 3);
    assert_eq!(
        reparsed.input_data().controller_inputs().len(),
        ghost.input_data().controller_inputs().len()
    );
}

#[test]
fn ghost_zero_finish_time() {
    let ghost = Ghost::new_from_file("./test_ghosts/0m00s000.rkg").unwrap();
    assert_eq!(ghost.header().finish_time().to_string(), "00:00.000");
}

#[test]
fn ghost_preserve_footer_affects_raw_data_size() {
    let mut ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    let raw_with_footer = ghost.raw_data();
    ghost.set_should_preserve_external_footer(false);
    let raw_without_footer = ghost.raw_data();
    // Footer is present, so with-footer should be larger
    assert!(raw_with_footer.len() > raw_without_footer.len());
}

#[test]
fn ghost_set_input_data_compressed_syncs_header() {
    let mut ghost = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").unwrap();
    assert!(ghost.header().is_compressed());
    ghost.set_input_data_compressed(false);
    assert!(!ghost.header().is_compressed());
    assert!(!ghost.input_data().compressed());
}

#[test]
fn ghost_9_laps() {
    let ghost = Ghost::new_from_file("./test_ghosts/9laps_test.rkg").unwrap();
    assert_eq!(ghost.header().lap_count(), 9);
    assert_eq!(ghost.header().lap_split_times().len(), 9);
}

// ===== Location Tests =====

#[test]
fn location_default_is_not_set() {
    let loc = Location::default();
    assert_eq!(loc.country(), Country::NotSet);
}

#[test]
fn location_find_does_not_panic_on_unknown() {
    let _ = Location::find(0xFE, 0xFE, Some(Version::Vanilla));
    let _ = Location::find(0x00, 0x00, None);
}

#[test]
fn location_find_exact_vanilla_not_set() {
    // NotSet (0xFF, 0xFF) - just check it doesn't panic
    let _ = Location::find_exact(0xFF, 0xFF, Version::Vanilla);
}
