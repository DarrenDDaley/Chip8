mod display;
mod memory;

use std::process;

pub fn main() {

    let mut memory = memory::RAM::new();

    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let display = display::Display::new("Chip 8 Emulator".to_string(),
                                                800,
                                                600,
                                                1);
    display.initialize();
}
