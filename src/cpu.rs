use rand;
use rand::Rng;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;


const OPCODE_SIZE: usize = 2;

pub struct Output<'a> {
    pub video_memory: &'a [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub video_memory_changed: bool,
    pub beep: bool,
}

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
    video_memory_changed : bool,
    video_memory: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    keypad: [bool; 16],
    keypad_register: usize,
    keypad_waiting: bool,
    beep: bool,

}

impl CPU {
    pub fn new(memory: [u8; 4096]) -> Self {
        let cpu  = CPU {
            registers: [0; 16],
            register_i: 0x200,
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory,
            video_memory_changed: false,
            video_memory: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            keypad: [false; 16],
            keypad_register: 0,
            keypad_waiting: false,
            beep: false,
        };
        cpu
    }

    pub fn cpu_cycle(&mut self, keypad: [bool; 16]) -> Output {
        self.keypad = keypad;
        self.video_memory_changed = false;

        if self.keypad_waiting {
            for i in 0..keypad.len() {
                if keypad[i] {
                    self.keypad_waiting = false;
                    self.registers[self.keypad_register] = i as u8;
                    break;
                }
            }
        }
        else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

           if self.sound_timer > 0 {
               self.sound_timer -= 1;
           }
            self.opcode_execute();
        }

        Output {
            video_memory: &self.video_memory,
            video_memory_changed: self.video_memory_changed,
            beep: self.sound_timer > 0,
        }

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
            (0x00, 0x00, 0x0e, 0x00) => self.opcode_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.opcode_00ee(),
            (0x01, _, _, _) => self.opcode_1nnn(nnn),
            (0x02, _, _, _) => self.opcode_2nnn(nnn),
            (0x03, _, _, _) => self.opcode_3xkk(x, kk),
            (0x04, _, _, _) => self.opcode_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.opcode_5xy0(x, y),
            (0x06, _, _, _) => self.opcode_6xkk(x, kk),
            (0x07, _, _, _) => self.opcode_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.opcode_8xy0(x, y),
            (0x08, _, _, 0x01) => self.opcode_8xy1(x, y),
            (0x08, _, _, 0x02) => self.opcode_8xy2(x, y),
            (0x08, _, _, 0x03) => self.opcode_8xy3(x, y),
            (0x08, _, _, 0x04) => self.opcode_8xy4(x, y),
            (0x08, _, _, 0x05) => self.opcode_8xy5(x, y),
            (0x08, _, _, 0x06) => self.opcode_8xy6(x, y),
            (0x08, _, _, 0x07) => self.opcode_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.opcode_8xye(x, y),
            (0x09, _, _, 0x00) => self.opcode_9xy0(x, y),
            (0x0a, _, _, _) => self.opcode_annn(nnn),
            (0x0b, _, _, _) => self.opcode_bnn(nnn),
            (0x0c, _, _, _) => self.opcode_cxkk(x, kk),
            (0x0d, _, _, _) => self.opcode_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.opcode_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.opcode_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.opcode_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.opcode_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.opcode_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.opcode_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.opcode_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.opcode_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.opcode_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.opcode_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.opcode_fx65(x),

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

    fn opcode_00e0(&mut self) -> ProgramCounter {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                self.video_memory[y][x] = 0;
            }
        }
        self.video_memory_changed = true;
        ProgramCounter::Next
    }

    fn opcode_00ee(&mut self) -> ProgramCounter {
        self.stack_pointer -= 1;
        ProgramCounter::Jump(self.stack[self.stack_pointer])
    }

    fn opcode_1nnn(&self, nnn: usize) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn opcode_2nnn(&mut self, nnn: usize) -> ProgramCounter {
        self.stack[self.stack_pointer] = self.program_counter + OPCODE_SIZE;
        self.stack_pointer += 1;

        ProgramCounter::Jump(nnn)
    }

    fn opcode_3xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] == kk)
    }

    fn opcode_4xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] != kk)
    }

    fn opcode_5xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] == self.registers[y])
    }

    fn opcode_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.registers[x] = kk;

        ProgramCounter::Next
    }

    fn opcode_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let vx = self.registers[x] as u16;
        let val = kk as u16;
        let result = vx + val;
        self.registers[x] = result as u8;

        ProgramCounter::Next
    }

    fn opcode_8xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] = self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_8xy1(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] |= self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] &= self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[x] ^= self.registers[y];

        ProgramCounter::Next
    }

    fn opcode_8xy4(&mut self, x: usize, y: usize) -> ProgramCounter {
        let result = (self.registers[x] as u16) + (self.registers[y] as u16);
        self.registers[x] = result as u8;

        self.registers[0x0f] = if result > 0xFF { 1 } else { 0 };

        ProgramCounter::Next
    }

    fn opcode_8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
         self.registers[0x0f] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
         self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);

        ProgramCounter::Next
    }

    fn opcode_8xy6(&mut self, x: usize, y: usize) -> ProgramCounter {
         self.registers[0x0f] =  self.registers[x] & 1;
         self.registers[x] /= 2;

        ProgramCounter::Next
    }

    fn opcode_8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[0x0f] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);

        ProgramCounter::Next
    }

    fn opcode_8xye(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.registers[0x0f] = (self.registers[x] & 0b10000000) >> 7;
        self.registers[x] *= 2;

        ProgramCounter::Next
    }

    fn opcode_9xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.registers[x] != self.registers[y])
    }

    fn opcode_annn(&mut self, nnn: usize) -> ProgramCounter {
        self.register_i = nnn;

        ProgramCounter::Next
    }

    fn opcode_bnn(&self, nnn: usize) ->  ProgramCounter {
        ProgramCounter::Jump((self.registers[0] as usize) + nnn)
    }

    fn opcode_cxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let mut range = rand::thread_rng();
        self.registers[x] = range.gen::<u8>() & kk;

        ProgramCounter::Next
    }

    fn opcode_dxyn(&mut self, x: usize, y: usize, n: usize) -> ProgramCounter {
        self.registers[0x0f] = 0;
        for byte in 0..n  {
            let y = (self.registers[y] as usize + byte) % CHIP8_HEIGHT;
                for bit in 0..8 {
                let x = (self.registers[x] as usize + bit) % CHIP8_WIDTH;
                let color = (self.memory[self.register_i + byte] >> (7 - bit)) & 1;
                self.registers[0x0f] |= color & self.video_memory[y][x];
                self.video_memory[y][x] ^= color;
            }
        }

        self.video_memory_changed = true;
        ProgramCounter::Next
    }

    fn opcode_ex9e(&self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.keypad[self.registers[x] as usize])
    }

    fn opcode_exa1(&self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(!self.keypad[self.registers[x] as usize])
    }

    fn opcode_fx07(&mut self, x: usize) -> ProgramCounter {
        self.registers[x] = self.delay_timer;

        ProgramCounter::Next
    }

    fn opcode_fx0a(&mut self, x: usize) -> ProgramCounter {
        self.keypad_waiting = true;
        self.keypad_register = x;

        ProgramCounter::Next
    }

    fn opcode_fx15(&mut self, x: usize) -> ProgramCounter {
        self.delay_timer = self.registers[x];

        ProgramCounter::Next
    }

    fn opcode_fx18(&mut self, x: usize) -> ProgramCounter {
        self.sound_timer = self.registers[x];

        ProgramCounter::Next
    }

    fn opcode_fx1e(&mut self, x: usize) -> ProgramCounter {
        self.register_i += (self.registers[x] as usize);
        self.registers[0x0f] = if self.register_i > 0xF00 { 1 } else { 0 };

        ProgramCounter::Next
    }

    fn opcode_fx29(&mut self, x: usize) -> ProgramCounter {
        self.register_i = (self.registers[x] as usize) * 5;

        ProgramCounter::Next
    }

    fn opcode_fx33(&mut self, x: usize) -> ProgramCounter {
        self.memory[self.register_i] = self.registers[x] / 100;
        self.memory[self.register_i + 1] = (self.registers[x] % 100) / 10;
        self.memory[self.register_i + 2] = self.registers[x] % 10;

        ProgramCounter::Next
    }

    fn opcode_fx55(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1 {
            self.memory[self.register_i + i] = self.registers[i];
        }
        ProgramCounter::Next
    }

    fn opcode_fx65(&mut self, x: usize) -> ProgramCounter {
        for i in 0..x + 1  {
            self.registers[i] = self.memory[self.register_i + i];
        }
        ProgramCounter::Next
    }
}