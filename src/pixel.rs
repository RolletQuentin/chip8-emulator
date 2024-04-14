//! # Pixel

use crate::cpu::CPU;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const BLACK: u8 = 0;
const WHITE: u8 = 1;
const LENGTH_WIDTH: u16 = 64;
const LENGTH_HEIGHT: u16 = 32;
const PIXEL_SIZE: u16 = 8;
const WIDTH: u32 = (LENGTH_WIDTH * PIXEL_SIZE) as u32;
const HEIGHT: u32 = (LENGTH_HEIGHT * PIXEL_SIZE) as u32;

pub struct PixelDisplay {
    window: Window,
    buffer: Vec<u32>,
}

impl PixelDisplay {
    pub fn new() -> PixelDisplay {
        let mut window = Window::new(
            "Chip8 Emulator",
            WIDTH as usize,
            HEIGHT as usize,
            WindowOptions::default(),
        )
        .unwrap();

        window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));

        PixelDisplay {
            window,
            buffer: vec![0; (WIDTH * HEIGHT) as usize],
        }
    }

    pub fn draw(&mut self, pixels: &Vec<Pixel>) {
        for pixel in pixels {
            let x = pixel.position.x * PIXEL_SIZE;
            let y = pixel.position.y * PIXEL_SIZE;
            let color = if pixel.color == BLACK { 0 } else { 0xFFFFFF };

            for i in 0..PIXEL_SIZE {
                for j in 0..PIXEL_SIZE {
                    let index = (y + i) as u32 * WIDTH + x as u32 + j as u32;
                    self.buffer[index as usize] = color;
                }
            }
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }

    pub fn get_key_down(&mut self) -> Option<Key> {
        let keys = [
            Key::Escape,
            Key::NumPad7,
            Key::NumPad8,
            Key::NumPad9,
            Key::NumPadAsterisk,
            Key::NumPad4,
            Key::NumPad5,
            Key::NumPad6,
            Key::NumPadMinus,
            Key::NumPad1,
            Key::NumPad2,
            Key::NumPad3,
            Key::NumPadPlus,
            Key::Right,
            Key::NumPad0,
            Key::NumPadDot,
            Key::NumPadEnter,
        ];

        for key in keys.iter() {
            if self.window.is_key_down(*key) {
                return Some(*key);
            }
        }

        None
    }

    pub fn get_key_up(&mut self) -> Option<Key> {
        let keys = [
            Key::Escape,
            Key::NumPad7,
            Key::NumPad8,
            Key::NumPad9,
            Key::NumPadAsterisk,
            Key::NumPad4,
            Key::NumPad5,
            Key::NumPad6,
            Key::NumPadMinus,
            Key::NumPad1,
            Key::NumPad2,
            Key::NumPad3,
            Key::NumPadPlus,
            Key::Right,
            Key::NumPad0,
            Key::NumPadDot,
            Key::NumPadEnter,
        ];

        for key in keys.iter() {
            if self.window.is_key_released(*key) {
                return Some(*key);
            }
        }

        None
    }

    pub fn wait_key_pressed(&mut self, cpu: &mut CPU) {
        loop {
            if let Some(key) = self.get_key_down() {
                match key {
                    Key::NumPad7 => cpu.key[0] = true,
                    Key::NumPad8 => cpu.key[1] = true,
                    Key::NumPad9 => cpu.key[2] = true,
                    Key::NumPadAsterisk => cpu.key[3] = true,
                    Key::NumPad4 => cpu.key[4] = true,
                    Key::NumPad5 => cpu.key[5] = true,
                    Key::NumPad6 => cpu.key[6] = true,
                    Key::NumPadMinus => cpu.key[7] = true,
                    Key::NumPad1 => cpu.key[8] = true,
                    Key::NumPad2 => cpu.key[9] = true,
                    Key::NumPad3 => cpu.key[10] = true,
                    Key::NumPadPlus => cpu.key[11] = true,
                    Key::Right => cpu.key[12] = true,
                    Key::NumPad0 => cpu.key[13] = true,
                    Key::NumPadDot => cpu.key[14] = true,
                    Key::NumPadEnter => cpu.key[15] = true,
                    _ => {}
                }
            }
            break;
        }
    }
}

struct Position {
    x: u16,
    y: u16,
}

pub struct Pixel {
    position: Position,
    color: u8,
}

impl Pixel {
    pub fn new(x: u16, y: u16, color: u8) -> Pixel {
        Pixel {
            position: Position { x, y },
            color,
        }
    }

    pub fn init() -> Vec<Pixel> {
        let mut pixels = Vec::new();

        for i in 0..LENGTH_HEIGHT {
            for j in 0..LENGTH_WIDTH {
                let color = BLACK;
                let pixel = Pixel::new(j, i, color);
                pixels.push(pixel);
            }
        }

        pixels
    }

    pub fn clear_screen(pixels: &mut Vec<Pixel>) {
        for pixel in pixels.iter_mut() {
            pixel.color = BLACK;
        }
    }

    pub fn draw_screen(cpu: &mut CPU, pixels: &mut Vec<Pixel>, b1: u16, b2: u16, b3: u16) {
        let mut x;
        let mut y;
        let mut code;
        let mut shift;
        cpu.v[0xF] = 0;

        for k in 0..b1 {
            // Get the code of the line to draw
            code = cpu.memory[(cpu.i + k) as usize];

            // Get the ordinate of the line to draw
            y = ((cpu.v[b2 as usize] + k as u8) % LENGTH_HEIGHT as u8) as u16;

            shift = 7;
            for j in 0..8 {
                // Get the abscissa of the pixel to draw
                x = ((cpu.v[b3 as usize] + j) % LENGTH_WIDTH as u8) as u16;

                // Get the color of the pixel to draw
                if ((code) & (0x1 << shift)) != 0 {
                    let color = if pixels[(y * LENGTH_WIDTH + x) as usize].color == BLACK {
                        WHITE
                    } else {
                        cpu.v[0xF] = 1;
                        BLACK
                    };
                    pixels[(y * LENGTH_WIDTH + x) as usize].color = color;
                }
                shift -= 1;
            }
        }
    }
}
