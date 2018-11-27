mod display;
mod memory;

use std::process;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

pub fn main() {

    let mut memory = memory::RAM::new();

    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let display = display::Display::new();
    display.initialize();
}
