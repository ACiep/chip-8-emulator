use crate::{memory::Memory, rom::Rom, PROGRAM_START_INDEX};

#[allow(dead_code)]
pub struct Chip8 {
    memory: Memory,
    program_counter: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new(rom: Rom) -> Self {
        let mut memory = Memory::new();
        memory.load_rom(rom);
        Self {
            memory,
            program_counter: PROGRAM_START_INDEX,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn cycle(&mut self) {
        let opcode: u16 = self.memory.get_opcode(self.program_counter);
        println!("Executing opcode: {:X?}.", opcode);

        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            _ => {
                eprintln!("Unknown opcode: {:X?}.", opcode);
                std::process::exit(1);
            }
        }
    }
}
