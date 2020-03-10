mod chip8;
pub mod rom;

use chip8::Chip8;
use rom::Rom;
use std::fs::File;

fn main() {
    let filename = "resources/PONG";
    let file = File::open(filename).unwrap();
    let rom = Rom::from(file);
    let chip8 = Chip8::new(rom);

    dbg!(chip8);
}
