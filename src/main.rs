extern crate sdl2;

mod display;
mod memory;
mod cpu;

use std::process;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

pub fn main() {

    let mut memory = memory::RAM::new();

    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
}
