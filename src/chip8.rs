use crate::rom::Rom;

#[derive(Debug)]
pub struct Chip8 {
    rom: Rom,
}

impl Chip8 {
    pub fn new(rom: Rom) -> Self {
        Self { rom }
    }
}
