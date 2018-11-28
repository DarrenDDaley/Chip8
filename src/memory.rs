use CHIP8_WIDTH;
use CHIP8_HEIGHT;

use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct RAM {
    pub memory : [u8; 4096],
    pub video_ram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]
}

impl RAM {
    pub fn new() -> RAM {
        let mut ram = RAM {
            memory: [0; 4096],
            video_ram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT]
        };

        for i in 0x0..0x50  {
            ram.memory[i] = FONTSET[i];
        }
        ram
    }

    pub fn load_rom(&mut self) -> Result<(), String> {

        let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            return  Err("file path to the rom is required".to_string());
        }

        let rom_path = &args[1];
        let mut buffer = Vec::new();
        
        let mut rom = File::open(rom_path).map_err(|e| e.description().to_string())?;
        rom.read_to_end(&mut buffer).map_err(|e| e.description().to_string())?;

        for (i, byte) in buffer.bytes().enumerate() {
            self.memory[i+0x200] = byte.map_err(|e| e.description().to_string())?;
        }
        
        Ok(())
    }
}

static FONTSET: [u8; 80] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
     0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
     0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
     0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
     0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
     0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
     0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
     0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];
