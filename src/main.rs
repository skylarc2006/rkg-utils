pub mod rkg;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use bitreader::BitReader;

use crate::rkg::header::finish_time::FinishTime;

fn main() {
    // get the path of the current executable and then go 2 directories up (since .exe is in debug/build)
    let mut ghost_file_path: PathBuf = env::current_exe().expect("Failed to get current exe path");
    ghost_file_path.pop();
    ghost_file_path.pop();
    ghost_file_path.pop();
    ghost_file_path.push("test_ghosts");
    ghost_file_path.push("JC_LC.rkg");

    // Path to rkg
    let path: &Path = Path::new(&ghost_file_path);
    let display: std::path::Display<'_> = path.display();

    let mut rkg_data: Vec<u8> = Vec::new();

    // Open file and extract bytes
    match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => {
            println!("File opened successfully!");
            for byte_result in file.bytes() {
                match byte_result {
                    Err(why) => panic!("Failed to read byte: {}", why),
                    Ok(byte) => rkg_data.push(byte),
                }
            }
        },
    };
    
    let mut rkg_reader: BitReader<'_> = BitReader::new(&rkg_data);
    let rkgd_bytes: [u8; _] = rkg_reader.read_u32(32).expect("Failed to read rkgd").to_be_bytes();
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

    println!("RKGD: {}", rkgd);
}
