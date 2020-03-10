use crate::{memory::Memory, opcode::OPCode, rom::Rom, screen::Screen, PROGRAM_START_INDEX};
use rand::prelude::*;

#[allow(dead_code)]
pub struct Chip8 {
    memory: Memory,
    program_counter: usize,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    i: u16,
    stack: Vec<usize>,
    screen: Screen,
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
            v: [0; 16],
            i: 0,
            stack: Vec::new(),
            screen: Screen::new(),
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory.get_opcode(self.program_counter);
        println!("Executing opcode: {:X?}.", opcode.0);

        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: OPCode) {
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.0 & 0x000F {
                0x0000 => {
                    self.screen.clear();
                    self.next_instruction();
                }
                0x000E => {
                    self.program_counter = self.stack.pop().unwrap();
                }
                _ => {
                    Self::unknown_opcode(opcode);
                }
            },
            0x1000 => {
                self.program_counter = opcode.nnn().into();
            }
            0x2000 => {
                self.stack.push(self.program_counter + 2);
                self.program_counter = opcode.nnn().into();
            }
            0x3000 => {
                if self.v[opcode.x()] == opcode.nn() {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            0x4000 => {
                if self.v[opcode.x()] != opcode.nn() {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            0x5000 => {
                if self.v[opcode.x()] == self.v[opcode.y() as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            0x6000 => {
                self.v[opcode.x()] = opcode.nn();
                self.next_instruction();
            }
            0x7000 => {
                self.v[opcode.x()] += opcode.nn();
                self.next_instruction();
            }
            0x8000 => match opcode.0 & 0x000F {
                0x0000 => {
                    self.v[opcode.x()] = self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0001 => {
                    self.v[opcode.x()] = self.v[opcode.x()] | self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0002 => {
                    self.v[opcode.x()] = self.v[opcode.x()] & self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0003 => {
                    self.v[opcode.x()] = self.v[opcode.x()] ^ self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0004 => {
                    if self.v[opcode.x()] > (0xFF - self.v[opcode.y()]) {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[opcode.x()] += self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0005 => {
                    if opcode.y() > opcode.x() {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[opcode.x()] -= self.v[opcode.y()];
                    self.next_instruction();
                }
                0x0006 => {
                    self.v[0xF] = self.v[opcode.x()];
                    self.v[opcode.x()] = self.v[opcode.x()] >> 1;
                    self.next_instruction();
                }
                0x0007 => {
                    if self.v[opcode.x()] > self.v[opcode.y()] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[opcode.x()] = self.v[opcode.y()] - self.v[opcode.x()];
                    self.next_instruction();
                }
                0x000E => {
                    self.v[0xF] = self.v[opcode.x()];
                    self.v[opcode.x()] = self.v[opcode.x()] << 1;
                    self.next_instruction();
                }
                _ => Self::unknown_opcode(opcode),
            },
            0x9000 => {
                if self.v[opcode.x()] != self.v[opcode.y()] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            }
            0xA000 => {
                self.i = opcode.nnn().into();
                self.next_instruction();
            }
            0xB000 => {
                self.program_counter = (opcode.nnn() + self.v[0]) as usize;
            }
            0xC000 => {
                let mut rng = thread_rng();
                self.v[opcode.x()] = opcode.nn() & rng.gen_range(0, 255);
                self.next_instruction();
            }
            0xD000 => {} // TODO draw
            0xE000 => match opcode.0 & 0x00FF {
                0x009E => {} // TODO keys
                0x00A1 => {} // TODO keys
                _ => {
                    Self::unknown_opcode(opcode);
                }
            },
            0xF000 => match opcode.0 & 0x0FF {
                0x0007 => {
                    self.v[opcode.x()] = self.delay_timer;
                    self.next_instruction();
                }
                0x000A => {} // TODO
                0x0015 => {
                    self.delay_timer = self.v[opcode.x()];
                    self.next_instruction();
                }
                0x0018 => {
                    self.sound_timer = self.v[opcode.x()];
                    self.next_instruction();
                }
                0x001E => {
                    let x: u16 = self.v[opcode.x()].into();
                    if self.i + x > 0xFFF {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.i += x;
                    self.next_instruction();
                }
                0x0029 => {} // TODO
                0x0033 => {} // TODO
                0x0055 => {} // TODO
                0x0065 => {} // TODO
                _ => {
                    Self::unknown_opcode(opcode);
                }
            },
            _ => {
                Self::unknown_opcode(opcode);
            }
        }
    }

    fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    fn skip_instruction(&mut self) {
        self.program_counter += 4;
    }

    fn unknown_opcode(opcode: OPCode) {
        eprintln!("Unknown opcode: {:X?}.", opcode.0);
        std::process::exit(1);
    }
}
