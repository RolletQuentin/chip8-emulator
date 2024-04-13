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

    let start_emulation = cpu.load_rom("roms/maze.ch8");

    if start_emulation {
        while continue_emulation {
            continue_emulation = listen(&mut display);

            for _ in 0..CPU_SPEED {
                cpu.execute_opcode(cpu.get_opcode(), &mut display, &mut pixels);
            }

            display.draw(&pixels);
            cpu.count_down();
        }
    }
}

fn listen(screen: &mut pixel::PixelDisplay) -> bool {
    let mut continue_emulation = true;

    if let Some(key) = screen.get_key() {
        match key {
            minifb::Key::Escape => continue_emulation = false,
            _ => {}
        }
    }

    continue_emulation
}
