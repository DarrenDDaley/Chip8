use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const OPCODE_SIZE: usize = 2;

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize)
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        match condition {
            true => ProgramCounter::Skip,
            _ => ProgramCounter::Next
        }
    }
}

pub struct CPU {
    registers : [u8; 16],
    register_i : u16,
    program_counter: usize,
    stack: [usize; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    memory : [u8; 4096],
    pub video_ram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]
}

impl CPU {
    pub fn new(memory: [u8; 4096]) -> Self {
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
        cpu
    }

    pub fn opcode_execute(&mut self, opcode: u16) {
        let pc_change = match opcode {
            0x00E0 => self.opcode_clear_screen(),
            0x00EE => self.opcode_return_subroutine(),

            _ => ProgramCounter::Next
        };

        match pc_change {
            ProgramCounter::Next => self.program_counter += OPCODE_SIZE,
            ProgramCounter::Skip => self.program_counter += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(address) => self.program_counter = address
        }
    }

    pub fn opcode_fetch(&mut self) -> u16 {
        return (self.memory[self.program_counter] as u16) << 8 |
               (self.memory[self.program_counter + 1] as u16);
    }

    fn opcode_clear_screen(&mut self) -> ProgramCounter {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                self.video_ram[y][x] = 0;
            }
        }
        ProgramCounter::Next
    }

    fn opcode_return_subroutine(&mut self) -> ProgramCounter {
        let pointer = self.stack_pointer;

        self.stack_pointer -= 1;
        ProgramCounter::Jump(self.stack[pointer])
    }
}

