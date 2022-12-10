use crate::{
    display::{Display, HEIGHT, WIDTH},
    keypad::Keypad,
};
use rand::Rng;

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub v: [u8; 16],
    // peripherals
    pub display: Display,
    pub keypad: Keypad,
    // stack
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u8,
    // delay timer
    pub dt: u8,
    // sound timer
    pub st: u8,
}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            display: Display::new(),
            keypad: Keypad::new(),
            sp: 0,
            st: 0,
            dt: 0,
        }
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200;
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.stack = [0; 16];
        self.display.cls();
        for i in 0..16 {
            self.keypad.key_up(i as usize);
        }
        self.sp = 0;
        self.dt = 0;
        for (i, _) in FONT_SET.iter().enumerate() {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn execute(&mut self) {
        let opcode: u16 = read_word(self.memory, self.pc);
        self.process_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn process_opcode(&mut self, opcode: u16) {
        // get opcode parameters
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // split into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        // increment program counter
        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),

            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),

            (0x01, _, _, _) => self.op_1nnn(nnn),

            (0x02, _, _, _) => self.op_2nnn(nnn),

            (0x03, _, _, _) => self.op_3xkk(x, kk),

            (0x04, _, _, _) => self.op_4xkk(x, kk),

            (0x05, _, _, 0x00) => self.op_5xy0(x, y),

            (0x06, _, _, _) => self.op_6xkk(x, kk),

            (0x07, _, _, _) => self.op_7xkk(x, kk),

            (0x08, _, _, 0x00) => self.op_8xy0(x, y),

            (0x08, _, _, 0x01) => self.op_8xy1(x, y),

            (0x08, _, _, 0x02) => self.op_8xy2(x, y),

            (0x08, _, _, 0x03) => self.op_8xy3(x, y),

            (0x08, _, _, 0x04) => self.op_8xy4(x, y),

            (0x08, _, _, 0x05) => self.op_8xy5(x, y),

            (0x08, _, _, 0x06) => self.op_8xy6(x, y),

            (0x08, _, _, 0x07) => self.op_8xy7(x, y),

            (0x08, _, _, 0x0E) => self.op_8xye(x, y),

            (0x09, _, _, 0x00) => self.op_9xy0(x, y),

            (0x0A, _, _, _) => self.op_annn(nnn),

            (0x0B, _, _, _) => self.op_bnnn(nnn),

            (0x0C, _, _, _) => self.op_cxkk(x, kk),

            (0x0D, _, _, _) => self.op_dxyn(x, y, n),

            (0x0E, _, 0x09, 0x0E) => self.op_ex9e(x),

            (0x0E, _, 0x0A, 0x01) => self.op_exa1(x),

            (0x0F, _, 0x00, 0x07) => self.op_fx07(x),

            (0x0F, _, 0x00, 0x0A) => self.op_fx0a(x),

            (0x0F, _, 0x01, 0x05) => self.op_fx15(x),

            (0x0F, _, 0x01, 0x08) => self.op_fx18(x),

            (0x0F, _, 0x01, 0x0E) => self.op_fx1e(x),

            (0x0F, _, 0x02, 0x09) => self.op_fx29(x),

            (0x0F, _, 0x03, 0x03) => self.op_fx33(x),

            (0x0F, _, 0x05, 0x05) => self.op_fx55(x),

            (0x0F, _, 0x06, 0x05) => self.op_fx65(x),

            (_, _, _, _) => (),
        }
    }

    fn op_00e0(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.display.memory[y][x] = 0;
            }
        }
    }

    fn op_00ee(&mut self) {
        self.pc = self.stack[(self.sp - 1) as usize];
        self.sp -= 1;
    }

    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[0] = self.pc;
        self.pc = nnn;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.v[x] as u16;
        let vy = self.v[y] as u16;
        let result = vx + vy;
        self.v[x] = result as u8;
        self.v[0x0F] = if result > 0xFF { 1 } else { 0 };
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    fn op_8xy6(&mut self, x: usize, _y: usize) {
        self.v[0x0F] = self.v[x] & 1;
        self.v[x] >>= 1;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0F] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }

    fn op_8xye(&mut self, x: usize, _y: usize) {
        self.v[0x0F] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
    }

    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {
        self.v[0x0F] = 0;
        for byte in 0..n as usize {
            let y = (self.v[y] as usize + byte) % HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + byte) % WIDTH;
                let pixel = (self.memory[(self.i + byte as u16) as usize] >> (7 - bit)) & 1;
                self.v[0x0F] |= pixel & self.display.memory[y][x];
                self.display.memory[y][x] ^= pixel;
            }
        }
    }

    fn op_ex9e(&mut self, x: usize) {
        self.pc += if self.keypad.is_key_down(self.v[x] as usize) {
            2
        } else {
            0
        };
    }

    fn op_exa1(&mut self, x: usize) {
        self.pc += if self.keypad.is_key_down(self.v[x] as usize) {
            0
        } else {
            2
        };
    }

    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
    }

    fn op_fx0a(&mut self, x: usize) {
        self.pc -= 2;
        for (i, key) in self.keypad.keys.iter().enumerate() {
            if *key {
                self.v[x] = i as u8;
                self.pc += 2;
            }
        }
    }

    fn op_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
    }

    fn op_fx18(&mut self, x: usize) {
        self.st = self.v[x];
    }

    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as u16;
        self.v[0x0F] = if self.i > 0x0F00 { 1 } else { 0 };
    }

    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as u16) * 5;
    }

    fn op_fx33(&mut self, x: usize) {
        self.memory[self.i as usize] = self.v[x] / 100;
        self.memory[(self.i + 1) as usize] = (self.v[x] % 100) / 10;
        self.memory[(self.i + 2) as usize] = self.v[x] % 10;
    }

    fn op_fx55(&mut self, x: usize) {
        for i in 0..(x + 1) {
            self.memory[(self.i + i as u16) as usize] = self.v[i];
        }
    }

    fn op_fx65(&mut self, x: usize) {
        for i in 0..(x + 1) {
            self.v[i] = self.memory[(self.i + i as u16) as usize];
        }
    }
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;
