use crate::{memory::Memory, rom::Rom};

#[allow(dead_code)]
pub struct Chip8 {
    // rom: Rom,
    memory: Memory,
    program_counter: usize,
}

impl Chip8 {
    pub fn new(rom: Rom) -> Self {
        let mut memory = Memory::new();
        memory.load_rom(rom);
        Self {
            // rom,
            memory,
            program_counter: 0,
        }
    }

    pub fn cycle(&mut self) {
        // fetch opcode
        let opcode: u16 = self.memory.get_opcode(self.program_counter);
        self.program_counter += 2;
        println!("{:X?}", opcode);

        // decode opcode
        // execute opcode

        // update timers
    }
}
