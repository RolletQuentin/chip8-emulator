//! CPU module
//!
//! This module contains the CPU struct and its implementation.
//! The CPU struct contains the registers, memory, stack, and other
//! components needed to emulate the Chip8 CPU.

use crate::pixel::{Pixel, PixelDisplay};

const NUMBER_OPCODES: usize = 35;

pub struct CPU {
    pub memory: [i16; 4096],
    pub v: [i16; 16],       // Registers
    pub i: u16,             // Index register
    pub jump: [u16; 16],    // Jump address
    pub number_jump: u16,   // Number of jumps
    pub game_counter: u16,  // Game counter
    pub sound_counter: u16, // Sound counter
    pub pc: u16,            // Program counter
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            jump: [0; 16],
            number_jump: 0,
            game_counter: 0,
            sound_counter: 0,
            pc: 0x200,
        };

        // Load font
        cpu.load_font();

        cpu
    }

    pub fn count_down(&mut self) {
        if self.game_counter > 0 {
            self.game_counter -= 1;
        }
        if self.sound_counter > 0 {
            self.sound_counter -= 1;
        }
    }

    pub fn load_rom(&mut self, rom: &str) -> bool {
        let rom = std::fs::read(rom).unwrap();
        let mut i = 0x200;

        for byte in rom.iter() {
            self.memory[i] = *byte as i16;
            i += 1;
        }

        true
    }

    pub fn load_font(&mut self) {
        // 0
        self.memory[0] = 0xF0;
        self.memory[1] = 0x90;
        self.memory[2] = 0x90;
        self.memory[3] = 0x90;
        self.memory[4] = 0xF0;

        // 1
        self.memory[5] = 0x20;
        self.memory[6] = 0x60;
        self.memory[7] = 0x20;
        self.memory[8] = 0x20;
        self.memory[9] = 0x70;

        // 2
        self.memory[10] = 0xF0;
        self.memory[11] = 0x10;
        self.memory[12] = 0xF0;
        self.memory[13] = 0x80;
        self.memory[14] = 0xF0;

        // 3
        self.memory[15] = 0xF0;
        self.memory[16] = 0x10;
        self.memory[17] = 0xF0;
        self.memory[18] = 0x10;
        self.memory[19] = 0xF0;

        // 4
        self.memory[20] = 0x90;
        self.memory[21] = 0x90;
        self.memory[22] = 0xF0;
        self.memory[23] = 0x10;
        self.memory[24] = 0x10;

        // 5
        self.memory[25] = 0xF0;
        self.memory[26] = 0x80;
        self.memory[27] = 0xF0;
        self.memory[28] = 0x10;
        self.memory[29] = 0xF0;

        // 6
        self.memory[30] = 0xF0;
        self.memory[31] = 0x80;
        self.memory[32] = 0xF0;
        self.memory[33] = 0x90;
        self.memory[34] = 0xF0;

        // 7
        self.memory[35] = 0xF0;
        self.memory[36] = 0x10;
        self.memory[37] = 0x20;
        self.memory[38] = 0x40;
        self.memory[39] = 0x40;

        // 8
        self.memory[40] = 0xF0;
        self.memory[41] = 0x90;
        self.memory[42] = 0xF0;
        self.memory[43] = 0x90;
        self.memory[44] = 0xF0;

        // 9
        self.memory[45] = 0xF0;
        self.memory[46] = 0x90;
        self.memory[47] = 0xF0;
        self.memory[48] = 0x10;
        self.memory[49] = 0xF0;

        // A
        self.memory[50] = 0xF0;
        self.memory[51] = 0x90;
        self.memory[52] = 0xF0;
        self.memory[53] = 0x90;
        self.memory[54] = 0x90;

        // B
        self.memory[55] = 0xE0;
        self.memory[56] = 0x90;
        self.memory[57] = 0xE0;
        self.memory[58] = 0x90;
        self.memory[59] = 0xE0;

        // C
        self.memory[60] = 0xF0;
        self.memory[61] = 0x80;
        self.memory[62] = 0x80;
        self.memory[63] = 0x80;
        self.memory[64] = 0xF0;

        // D
        self.memory[65] = 0xE0;
        self.memory[66] = 0x90;
        self.memory[67] = 0x90;
        self.memory[68] = 0x90;
        self.memory[69] = 0xE0;

        // E
        self.memory[70] = 0xF0;
        self.memory[71] = 0x80;
        self.memory[72] = 0xF0;
        self.memory[73] = 0x80;
        self.memory[74] = 0xF0;

        // F
        self.memory[75] = 0xF0;
        self.memory[76] = 0x80;
        self.memory[77] = 0xF0;
        self.memory[78] = 0x80;
        self.memory[79] = 0x80;
    }

    pub fn get_opcode(&self) -> u16 {
        ((self.memory[self.pc as usize] << 8) + self.memory[(self.pc + 1) as usize]) as u16
    }

    pub fn execute_opcode(
        &mut self,
        opcode: u16,
        screen: &mut PixelDisplay,
        pixels: &mut Vec<Pixel>,
    ) {
        let jump = Jump::new();
        let action = jump.get_action(opcode);

        let b3 = (opcode & 0x0F00) >> 8;
        let b2 = (opcode & 0x00F0) >> 4;
        let b1 = opcode & 0x000F;

        match action {
            1 => {
                // 00E0 : clear the screen
                Pixel::clear_screen(pixels);
                screen.draw(pixels);
            }
            2 => {
                // 00EE : return from a subroutine
                if self.number_jump > 0 {
                    self.pc = self.jump[(self.number_jump - 1) as usize];
                    self.number_jump -= 1;
                }
            }
            3 => {
                // 1NNN : jump to address 1NNN
                self.pc = (b3 << 8) + (b2 << 4) + b1;
                self.pc -= 2;
            }
            4 => {
                // 2NNN : call subroutine at address 2NNN
                self.jump[self.number_jump as usize] = self.pc;

                if self.number_jump < 15 {
                    self.number_jump += 1;
                }

                self.pc = (b3 << 8) + (b2 << 4) + b1;
                self.pc -= 2;
            }
            5 => {
                // 3XNN : skip next instruction if V[X] == NN
                if self.v[b3 as usize] == ((b2 << 4) + b1) as i16 {
                    self.pc += 2;
                }
            }
            6 => {
                // 4XNN : skip next instruction if V[X] != NN
                if self.v[b3 as usize] != ((b2 << 4) + b1) as i16 {
                    self.pc += 2;
                }
            }
            7 => {
                // 5XY0 : skip next instruction if V[X] == V[Y]
                if self.v[b3 as usize] == self.v[b2 as usize] {
                    self.pc += 2;
                }
            }
            8 => {
                // 6XNN : set V[X] = NN
                self.v[b3 as usize] = ((b2 << 4) + b1) as i16;
            }
            9 => {
                // 7XNN : set V[X] = V[X] + NN
                self.v[b3 as usize] += ((b2 << 4) + b1) as i16;
            }
            10 => {
                // 8XY0 : set V[X] = V[Y]
                self.v[b3 as usize] = self.v[b2 as usize];
            }
            11 => {
                // 8XY1 : set V[X] = V[X] OR V[Y]
                self.v[b3 as usize] |= self.v[b2 as usize];
            }
            12 => {
                // 8XY2 : set V[X] = V[X] AND V[Y]
                self.v[b3 as usize] &= self.v[b2 as usize];
            }
            13 => {
                // 8XY3 : set V[X] = V[X] XOR V[Y]
                self.v[b3 as usize] ^= self.v[b2 as usize];
            }
            14 => {
                // 8XY4 : set V[X] = V[X] + V[Y], set V[F] = carry
                if (self.v[b3 as usize] + self.v[b2 as usize]) > 255 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[b3 as usize] += self.v[b2 as usize];
            }
            15 => {
                // 8XY5 : set V[X] = V[X] - V[Y], set V[F] = NOT borrow
                self.v[0xF] = (self.v[b3 as usize] > self.v[b2 as usize]) as i16;
                self.v[b3 as usize] -= self.v[b2 as usize];
            }
            16 => {
                // 8XY6 : set V[X] = V[X] SHR 1
                self.v[0xF] = self.v[b3 as usize] & 0x01;
                self.v[b3 as usize] >>= 1;
            }
            17 => {
                // 8XY7 : set V[X] = V[Y] - V[X], set V[F] = NOT borrow
                self.v[0xF] = (self.v[b2 as usize] > self.v[b3 as usize]) as i16;
                self.v[b3 as usize] = self.v[b2 as usize] - self.v[b3 as usize];
            }
            18 => {
                // 8XYE : set V[X] = V[X] SHL 1
                self.v[0xF] = self.v[b3 as usize] >> 7;
                self.v[b3 as usize] <<= 1;
            }
            19 => {
                // 9XY0 : skip next instruction if V[X] != V[Y]
                if self.v[b3 as usize] != self.v[b2 as usize] {
                    self.pc += 2;
                }
            }
            20 => {
                // ANNN : set I = NNN
                self.i = (b3 << 8) + (b2 << 4) + b1;
            }
            21 => {
                // BNNN : jump to address NNN + V[0]
                self.pc = (b3 << 8) + (b2 << 4) + b1 + self.v[0] as u16;
                self.pc -= 2;
            }
            22 => {
                // CXNN : set V[X] = random byte AND NN
                self.v[b3 as usize] = ((rand::random::<u16>()) % ((b2 << 4) + b1 + 1)) as i16;
            }
            23 => {
                Pixel::draw_screen(self, pixels, b1, b2, b3);
            }
            24 => {
                // EX9E : skip next instruction if key with the value of V[X] is pressed
                // if self.v[b3] == 0 {
                //     self.pc += 2;
                // }
            }
            25 => {
                // EXA1 : skip next instruction if key with the value of V[X] is not pressed
                // if self.v[b3] != 0 {
                //     self.pc += 2;
                // }
            }
            26 => {
                // FX07 : set V[X] = delay timer value
                self.v[b3 as usize] = self.game_counter as i16;
            }
            27 => {
                // FX0A : wait for a key press, store the value of the key in V[X]
                // self.v[b3] = 0;
            }
            28 => {
                // FX15 : set delay timer = V[X]
                self.game_counter = self.v[b3 as usize] as u16;
            }
            29 => {
                // FX18 : set sound timer = V[X]
                self.sound_counter = self.v[b3 as usize] as u16;
            }
            30 => {
                // FX1E : set I = I + V[X]
                self.v[0xF] = (self.i + self.v[b3 as usize] as u16 > 0xFFF) as i16;
                self.i += self.v[b3 as usize] as u16;
            }
            31 => {
                // FX29 : set I = location of sprite for digit V[X]
                self.i = (self.v[b3 as usize] * 5) as u16;
            }
            32 => {
                // FX33 : store BCD representation of V[X] in memory locations I, I+1, and I+2

                self.memory[self.i as usize] =
                    (self.v[b3 as usize] - (self.v[b3 as usize] % 100) / 100) as i16;

                self.memory[(self.i + 1) as usize] =
                    (((self.v[b3 as usize] - self.v[b3 as usize] % 10) / 10) % 10) as i16;

                self.memory[(self.i + 2) as usize] = (self.v[b3 as usize]
                    - self.memory[self.i as usize] * 100
                    - 10 * self.memory[(self.i + 1) as usize])
                    as i16;
            }
            33 => {
                // FX55 : store registers V[0] through V[X] in memory starting at location I
                for j in 0..=b3 {
                    self.memory[(self.i + j) as usize] = self.v[j as usize] as i16;
                }
            }
            34 => {
                // FX65 : read registers V[0] through V[X] from memory starting at location I
                for j in 0..=b3 {
                    self.v[j as usize] = self.memory[(self.i + j) as usize] as i16;
                }
            }
            _ => println!("Unknown opcode: {:X}", opcode),
        }

        self.pc += 2;
    }
}

struct Jump {
    mask: [u16; NUMBER_OPCODES],
    id: [u16; NUMBER_OPCODES],
}

impl Jump {
    pub fn new() -> Jump {
        let mut jp = Jump {
            mask: [0; NUMBER_OPCODES],
            id: [0; NUMBER_OPCODES],
        };

        // 0NNN
        jp.mask[0] = 0x0000;
        jp.id[0] = 0x0FFF;

        // 00E0
        jp.mask[1] = 0xFFFF;
        jp.id[1] = 0x00E0;

        // 00EE
        jp.mask[2] = 0xFFFF;
        jp.id[2] = 0x00EE;

        // 1NNN
        jp.mask[3] = 0xF000;
        jp.id[3] = 0x1000;

        // 2NNN
        jp.mask[4] = 0xF000;
        jp.id[4] = 0x2000;

        // 3XNN
        jp.mask[5] = 0xF000;
        jp.id[5] = 0x3000;

        // 4XNN
        jp.mask[6] = 0xF000;
        jp.id[6] = 0x4000;

        // 5XY0
        jp.mask[7] = 0xF00F;
        jp.id[7] = 0x5000;

        // 6XNN
        jp.mask[8] = 0xF000;
        jp.id[8] = 0x6000;

        // 7XNN
        jp.mask[9] = 0xF000;
        jp.id[9] = 0x7000;

        // 8XY0
        jp.mask[10] = 0xF00F;
        jp.id[10] = 0x8000;

        // 8XY1
        jp.mask[11] = 0xF00F;
        jp.id[11] = 0x8001;

        // 8XY2
        jp.mask[12] = 0xF00F;
        jp.id[12] = 0x8002;

        // 8XY3
        jp.mask[13] = 0xF00F;
        jp.id[13] = 0x8003;

        // 8XY4
        jp.mask[14] = 0xF00F;
        jp.id[14] = 0x8004;

        // 8XY5
        jp.mask[15] = 0xF00F;
        jp.id[15] = 0x8005;

        // 8XY6
        jp.mask[16] = 0xF00F;
        jp.id[16] = 0x8006;

        // 8XY7
        jp.mask[17] = 0xF00F;
        jp.id[17] = 0x8007;

        // 8XYE
        jp.mask[18] = 0xF00F;
        jp.id[18] = 0x800E;

        // 9XY0
        jp.mask[19] = 0xF00F;
        jp.id[19] = 0x9000;

        // ANNN
        jp.mask[20] = 0xF000;
        jp.id[20] = 0xA000;

        // BNNN
        jp.mask[21] = 0xF000;
        jp.id[21] = 0xB000;

        // CXNN
        jp.mask[22] = 0xF000;
        jp.id[22] = 0xC000;

        // DXYN
        jp.mask[23] = 0xF000;
        jp.id[23] = 0xD000;

        // EX9E
        jp.mask[24] = 0xF0FF;
        jp.id[24] = 0xE09E;

        // EXA1
        jp.mask[25] = 0xF0FF;
        jp.id[25] = 0xE0A1;

        // FX07
        jp.mask[26] = 0xF0FF;
        jp.id[26] = 0xF007;

        // FX0A
        jp.mask[27] = 0xF0FF;
        jp.id[27] = 0xF00A;

        // FX15
        jp.mask[28] = 0xF0FF;
        jp.id[28] = 0xF015;

        // FX18
        jp.mask[29] = 0xF0FF;
        jp.id[29] = 0xF018;

        // FX1E
        jp.mask[30] = 0xF0FF;
        jp.id[30] = 0xF01E;

        // FX29
        jp.mask[31] = 0xF0FF;
        jp.id[31] = 0xF029;

        // FX33
        jp.mask[32] = 0xF0FF;
        jp.id[32] = 0xF033;

        // FX55
        jp.mask[33] = 0xF0FF;
        jp.id[33] = 0xF055;

        // FX65
        jp.mask[34] = 0xF0FF;
        jp.id[34] = 0xF065;

        jp
    }

    pub fn get_action(&self, opcode: u16) -> u16 {
        let jp = Jump::new();

        for action in 0..NUMBER_OPCODES {
            let result = jp.mask[action] & opcode;

            if result == jp.id[action] {
                return action as u16;
            }
        }

        0
    }
}
