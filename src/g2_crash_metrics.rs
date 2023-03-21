use std::fmt;
use std::io::Cursor;
use anyhow::{anyhow, Error};
use binrw::{BinRead, NullString, BinReaderExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de;
use serde::de::Visitor;
use crate::utils::get_file_as_byte_vec;

#[derive(BinRead, Debug, Default, Serialize, Deserialize)]
#[br(magic = b"MC2G")]
pub struct G2CrashMetrics {
    pub version: u32,
    pub uptimems: u64,

    #[br(pad_size_to(0x100))]
    pub scene: NullStringS,

    #[br(pad_size_to(0x20))]
    pub net_role: NullStringS,

    #[br(pad_size_to(0x20))]
    pub online_server_version: NullStringS,

    #[br(pad_size_to(0x200))]
    pub system_info: NullStringS,

    #[br(pad_size_to(0x400))]
    pub settings_info: NullStringS,

    #[br(pad_size_to(0x4000))]
    pub gpu_crash_report: NullStringS,

    #[br(pad_size_to(0x80))]
    pub vr_data: NullStringS,

    #[br(pad_size_to(0x200))]
    pub vr_hdm_description: NullStringS,

    #[br(pad_size_to(0x80))]
    pub operating_system: NullStringS,

    #[br(pad_size_to(0x4000))]
    pub modules: NullStringS,

    #[br(pad_size_to(0x800))]
    pub callstack: NullStringS,

    #[br(pad_size_to(0x100))]
    pub camera: NullStringS,

    pub exception: G2Exception,

    pub unknown: [u8; 0x8],
}

#[derive(BinRead, Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct G2Exception {
    pub exception_code: u32,
    pub exception_flags: u32,
    pub exception_address: u64,
    pub exception_num_parameters: u64,
    pub exception_information_01: u64,
    pub exception_information_02: u64,
    pub exception_information_03: u64,
}

pub const CRASH_METRICS_SIZE: usize = 0x9388;
pub const RAND_SEQUENCE: [u8; CRASH_METRICS_SIZE] = {

    let mut array: [u8; CRASH_METRICS_SIZE] = [0; CRASH_METRICS_SIZE];
    let mut seed: i32 = 0;

    let mut i: usize = 0;
    while i < CRASH_METRICS_SIZE {
        let next_seed = seed.wrapping_mul(0x343fd).wrapping_add(0x269EC3) & 0x7FFFFFFF;
        seed = next_seed;
        array[i] = ((seed >> 0x10) & 0x7FFF) as u8;
        i += 1;
    }
    array
};

impl G2CrashMetrics {

    fn decipher(encrypted_data: Vec<u8>) -> Result<Vec<u8>, Error> {

        let mut data: Vec<u8> = vec![];
        for (i, seq) in RAND_SEQUENCE.iter().enumerate() {
            let byte: i16 = *seq as i16 - encrypted_data[i] as i16;
            data.push(byte as u8);
        }
        Ok(data)
    }

    pub fn new(path: &str) -> Result<Self, Error> {
        if let Ok(bytes) = get_file_as_byte_vec(path){

            if let Ok(deciphered_data) = Self::decipher(bytes) {

                let mut reader = Cursor::new(deciphered_data);
                Ok(reader.read_ne().unwrap())

            } else {
                Err(anyhow!("Failed to parse given g2cm file"))
            }
        } else {
            Err(anyhow!("Failed to find the given g2cm file"))
        }
    }
}

//wrapper struct to implement Serialize
#[derive(BinRead, Debug, Default)]
pub struct NullStringS {
    pub string : NullString,
}

impl fmt::Display for NullStringS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl NullStringS {

    pub fn from_str(s: &str) -> Option<Self> {

        Some(Self { string: NullString::from(s) })
    }
}

impl Serialize for NullStringS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.string.to_string())
    }
}

impl<'de> Deserialize<'de> for NullStringS {
    fn deserialize<D>(deserializer: D) -> Result<NullStringS, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NullStringS { string: NullString::default()})
    }
}

impl<'de> Visitor<'de> for NullStringS {
    type Value = NullStringS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(NullStringS::from_str(value).unwrap())
    }
}