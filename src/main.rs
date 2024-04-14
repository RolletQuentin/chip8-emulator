//! # Chip8 emulator
//!
//! This is a simple Chip8 emulator written in Rust.

use pixel::Pixel;

mod cpu;
mod pixel;

const CPU_SPEED: u32 = 4; // 4 instructions per cycle

fn main() {
    let mut cpu = cpu::CPU::new();
    let mut display = pixel::PixelDisplay::new();
    let mut pixels = Pixel::init();

    let mut continue_emulation = true;

    let start_emulation = cpu.load_rom("roms/BC_test.ch8");

    if start_emulation {
        while continue_emulation {
            continue_emulation = listen(&mut cpu, &mut display);

            for _ in 0..CPU_SPEED {
                cpu.execute_opcode(cpu.get_opcode(), &mut display, &mut pixels);
            }

            display.draw(&pixels);
            cpu.count_down();
        }
    }
}

fn listen(cpu: &mut cpu::CPU, screen: &mut pixel::PixelDisplay) -> bool {
    let mut continue_emulation = true;

    if let Some(key) = screen.get_key_down() {
        match key {
            minifb::Key::Escape => continue_emulation = false,
            minifb::Key::NumPad7 => cpu.key[0] = true,
            minifb::Key::NumPad8 => cpu.key[1] = true,
            minifb::Key::NumPad9 => cpu.key[2] = true,
            minifb::Key::NumPadAsterisk => cpu.key[3] = true,
            minifb::Key::NumPad4 => cpu.key[4] = true,
            minifb::Key::NumPad5 => cpu.key[5] = true,
            minifb::Key::NumPad6 => cpu.key[6] = true,
            minifb::Key::NumPadMinus => cpu.key[7] = true,
            minifb::Key::NumPad1 => cpu.key[8] = true,
            minifb::Key::NumPad2 => cpu.key[9] = true,
            minifb::Key::NumPad3 => cpu.key[10] = true,
            minifb::Key::NumPadPlus => cpu.key[11] = true,
            minifb::Key::Right => cpu.key[12] = true,
            minifb::Key::NumPad0 => cpu.key[13] = true,
            minifb::Key::NumPadDot => cpu.key[14] = true,
            minifb::Key::NumPadEnter => cpu.key[15] = true,
            _ => {}
        }
    }

    if let Some(key) = screen.get_key_up() {
        match key {
            minifb::Key::NumPad7 => cpu.key[0] = false,
            minifb::Key::NumPad8 => cpu.key[1] = false,
            minifb::Key::NumPad9 => cpu.key[2] = false,
            minifb::Key::NumPadAsterisk => cpu.key[3] = false,
            minifb::Key::NumPad4 => cpu.key[4] = false,
            minifb::Key::NumPad5 => cpu.key[5] = false,
            minifb::Key::NumPad6 => cpu.key[6] = false,
            minifb::Key::NumPadMinus => cpu.key[7] = false,
            minifb::Key::NumPad1 => cpu.key[8] = false,
            minifb::Key::NumPad2 => cpu.key[9] = false,
            minifb::Key::NumPad3 => cpu.key[10] = false,
            minifb::Key::NumPadPlus => cpu.key[11] = false,
            minifb::Key::Right => cpu.key[12] = false,
            minifb::Key::NumPad0 => cpu.key[13] = false,
            minifb::Key::NumPadDot => cpu.key[14] = false,
            minifb::Key::NumPadEnter => cpu.key[15] = false,
            _ => {}
        }
    }

    continue_emulation
}
