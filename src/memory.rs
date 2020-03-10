use crate::{rom::Rom, PROGRAM_START_INDEX};

pub struct Memory([u8; 4096]);

impl Memory {
    pub fn new() -> Self {
        Self([0; 4096])
    }

    pub fn load_rom(&mut self, rom: Rom) {
        let mut i = PROGRAM_START_INDEX;
        for opcode in rom.data {
            println!("{}", opcode);
            self.0[i] = opcode;
            i += 1;
        }
    }

    pub fn get_opcode(&self, index: usize) -> u16 {
        let first_byte: u16 = self.0[index].into();
        let second_byte: u16 = self.0[index + 1].into();
        first_byte << 8 | second_byte
    }
}
