use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl From<File> for Rom {
    fn from(mut file: File) -> Self {
        let mut rom = Self { data: Vec::new() };
        file.read_to_end(&mut rom.data).unwrap();
        rom
    }
}
