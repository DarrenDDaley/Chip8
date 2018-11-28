use CHIP8_WIDTH;
use CHIP8_HEIGHT;

pub struct CPU {
    registers : [u8; 16],
    register_i : u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    memory : [u8; 4096],
    video_ram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]
}

impl CPU {
    pub fn new(memory: [u8; 4096],) -> CPU {
        let mut cpu  = CPU {
            registers: [0; 16],
            register_i: 0x200,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory,
            video_ram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT]
        };
    }

    fn fetch_opcode(&mut self) -> u16 {
        return self.memory[self.program_counter] << 8 |
               self.memory[self.program_counter + 1];
    }

    fn execute_opccode(&mut self) {
        match self.fetch_opcode() {
        }
    }
}

