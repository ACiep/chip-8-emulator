mod chip8;
pub mod memory;
pub mod rom;

use chip8::Chip8;
use rom::Rom;
use std::fs::File;

pub const PROGRAM_START_INDEX: usize = 512;

fn main() {
    let filename = "resources/PONG";
    let file = File::open(filename).unwrap();
    let rom = Rom::from(file);
    let mut chip8 = Chip8::new(rom);

    // loop {
    chip8.cycle();

    //     if chip8.should_draw() {
    //         draw()
    //     }

    //     chip8.get_key();
    // }

    // dbg!(chip8);
}
