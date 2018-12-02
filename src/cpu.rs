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
    register_i : usize,
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

    pub fn opcode_execute(&mut self) {

        let opcode = self.opcode_fetch();

        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;


        let pc_change = match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.opcode_clear_screen(),
            (0x00, 0x00, 0x0e, 0x0e) => self.opcode_return_subroutine(),
            (0x01, _, _, _) => self.opcode_jump_nnn(nnn),
            (0x02, _, _, _) => self.opcode_call_subroutine_nnn(nnn),
            (0x03, _, _, _) => self.opcode_skip_equal_vxkk(x, kk),
            (0x04, _, _, _) => self.opcode_skip_not_equal_vxkk(x, kk),
            (0x05, _, _, 0x00) => self.opcode_skip_equal_vxvy(x, y),
            (0x06, _, _, _) => self.opcode_set_vxkk(x, kk),
            (0x07, _, _, _) => self.opcode_add_vxkk(x, kk),
            (0x08, _, _, 0x00) => self.opcode_set_vxvy(x, y),
            (0x08, _, _, 0x01) => self.opcode_or_vxvy(x, y),
            (0x08, _, _, 0x02) => self.opcode_and_vxvy(x, y),
            (0x08, _, _, 0x03) => self.opcode_xor_vxvy(x, y),
            (0x08, _, _, 0x04) => self.opcode_addcarry_vxvy(x, y),
            (0x08, _, _, 0x05) => self.opcode_subtract_vxvy(x, y),
            (0x08, _, _, 0x06) => self.opcode_dividecarry_vxvy(x, y),
            (0x08, _, _, 0x07) => self.opcode_subtractnotborrow_vxvy(x, y),
            (0x08, _, _, 0x0e) => self.opcode_mutiplecarry_vxvy(x, y),
            (0x09, _, _, 0x00) => self.opcode_9xy0(x, y),
            (0x0a, _, _, _,) => self.opcode_annn(nnn),



            _ => ProgramCounter::Next
        };

        match pc_change {
            ProgramCounter::Next => self.program_counter += OPCODE_SIZE,
            ProgramCounter::Skip => self.program_counter += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(address) => self.program_counter = address
        }
    }

    fn opcode_fetch(&mut self) -> u16 {
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

    fn opcode_jump_nnn(&self, nnn: usize) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn opcode_call_subroutine_nnn(&mut self, nnn: usize) -> ProgramCounter {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;
        ProgramCounter::Jump(nnn)
    }

    fn opcode_skip_equal_vxkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] == kk)
    }

    fn opcode_skip_not_equal_vxkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] != kk)
    }

    fn opcode_skip_equal_vxvy(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] == self.registers[y])
    }

    fn opcode_set_vxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.registers[x] = kk;
        ProgramCounter::Next
    }

    fn opcode_add_vxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.registers[x] = self.registers[x] + kk;
        ProgramCounter::Next
    }

    fn opcode_set_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] = self.registers[y];
        ProgramCounter::Next
    }

    fn opcode_or_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] |= self.registers[y];
        ProgramCounter::Next
    }

    fn opcode_and_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] &= self.registers[y];
        ProgramCounter::Next
    }

    fn opcode_xor_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] ^= self.registers[y];
        ProgramCounter::Next
    }

    fn opcode_addcarry_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        let result = (self.registers[x] as u16) + (self.registers[y] as u16);
        self.registers[x] = result as u8;

        self.registers[0x0f] = if result > 0xFF { 1 } else { 0 };
        ProgramCounter::Next
    }

    fn opcode_subtract_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
         self.registers[0x0f] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
         self.registers[x] -= self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_dividecarry_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
         self.registers[0x0f] =  self.registers[x] & 1;
         self.registers[x] /= 2;

        ProgramCounter::Next;
    }

    fn opcode_subtractnotborrow_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[0x0f] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
        self.registers[x] -= self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_mutiplecarry_vxvy(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[0x0f] = (self.registers[x] & 0b10000000) >> 7;
        self.registers[x] *= 2;
    }

    fn opcode_9xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] != self.registers[y])
    }

    fn opcode_annn(&mut self, nnn: usize) -> ProgramCounter {
        self.register_i = nnn;

        ProgramCounter::Next
    }
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;