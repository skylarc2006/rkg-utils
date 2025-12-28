// TODO: error handling
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct CTGPVersion {
    major: u8,
    minor: u8,
    revision: u16,
}

impl CTGPVersion {
    pub fn new(major: u8, minor: u8, revision: u16) -> Self {
        Self {
            major,
            minor,
            revision
        }
    }
    
    pub fn from(bytes: &[u8]) -> Option<Vec<Self>> {
        let mut possible_versions = Vec::new();
        match bytes {
            &[0x01, 0x03, 0x01, 0x42] => { possible_versions.push(CTGPVersion::new(1, 3, 1044)); },
            &[0x01, 0x03, 0x01, 0x48] => { possible_versions.push(CTGPVersion::new(1, 3, 1046)); },
            
            &[0x01, 0x03, 0x01, 0x50] => { 
                possible_versions.push(CTGPVersion::new(1, 3, 1044));
                possible_versions.push(CTGPVersion::new(1, 3, 1048));
                possible_versions.push(CTGPVersion::new(1, 3, 1050));
                possible_versions.push(CTGPVersion::new(1, 3, 1054));
            },
            
            &[0x01, 0x03, 0x01, 0x78] => { possible_versions.push(CTGPVersion::new(1, 3, 1058)); },
            &[0x01, 0x03, 0x01, 0x80] => { possible_versions.push(CTGPVersion::new(1, 3, 1060)); },
            &[0x01, 0x03, 0x01, 0x82] => { possible_versions.push(CTGPVersion::new(1, 3, 1062)); },
            &[0x01, 0x03, 0x01, 0x92] => { possible_versions.push(CTGPVersion::new(1, 3, 1064)); },
            &[0x01, 0x03, 0x02, 0x16] => { possible_versions.push(CTGPVersion::new(1, 3, 1066)); },
            
            &[0x01, 0x03, 0x02, 0x18] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1068));
                possible_versions.push(CTGPVersion::new(1, 3, 1070));
            },
            
            &[0x01, 0x03, 0x02, 0x28] => { possible_versions.push(CTGPVersion::new(1, 3, 1074)); },
            &[0x01, 0x03, 0x02, 0x78] => { possible_versions.push(CTGPVersion::new(1, 3, 1076)); },
            
            &[0x01, 0x03, 0x02, 0x84] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1080));
                possible_versions.push(CTGPVersion::new(1, 3, 1082));
            },
            
            &[0x01, 0x03, 0x02, 0x86] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1084));
                possible_versions.push(CTGPVersion::new(1, 3, 1086));
                possible_versions.push(CTGPVersion::new(1, 3, 1088));
                possible_versions.push(CTGPVersion::new(1, 3, 1090));
                possible_versions.push(CTGPVersion::new(1, 3, 1092));
            },
            
            &[0x01, 0x03, 0x02, 0x96] => { possible_versions.push(CTGPVersion::new(1, 3, 1094)); },
            
            &[0x01, 0x03, 0x03, 0x00] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094));
                possible_versions.push(CTGPVersion::new(1, 3, 1096));
                possible_versions.push(CTGPVersion::new(1, 3, 1098));
            },
            
            &[0x01, 0x03, 0x03, 0x12] => { possible_versions.push(CTGPVersion::new(1, 3, 1100)); },
            &[0x01, 0x03, 0x04, 0x10] => { possible_versions.push(CTGPVersion::new(1, 3, 1182)); },
            
            // Unfinished!
            _ => { return None; }
            
        }
        Some(possible_versions)
    }
}

impl Display for CTGPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}.{:02}.{:04}",
            self.major, self.minor, self.revision
        )
    }
}
