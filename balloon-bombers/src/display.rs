use i8080::cpu::Cpu;

use crate::i8080::memory_bus::MemoryMap;
use crate::memory::BalloonBombersMemory;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const WIDTH: u32 = 224;
const HEIGHT: u32 = 256;
const SCALE_FACTOR: u32 = 2;

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub fn new(context: sdl2::Sdl) -> Self {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window("i8080", WIDTH * SCALE_FACTOR, HEIGHT * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        Display { canvas: canvas }
    }

    pub fn draw_display_whole(&mut self, cpu: &mut Cpu<BalloonBombersMemory>) {
        self.canvas.clear();
        for offset in 0x0..0x1C00 {
            let video_ram_byte = offset + 0x2400;
            let x = offset / 32;
            let y = 248 - ((offset % 32) * 8);
            let byte = cpu.memory.read(video_ram_byte);
            if byte > 0 {
                self.draw_byte(byte, x as u32, y as u32);
            }
        }
        self.canvas.present();
    }

    fn draw_byte(&mut self, byte: u8, x: u32, y: u32) {
        let mut cmp_byte: u8 = 1;
        for bit in (0..8).rev() {
            if byte & cmp_byte != 0 {
                self.canvas.set_draw_color(
                    if (y >= 190 && y <= 220) || (y >= 240 && x >= 15 && x <= 135) {
                        Color::GREEN
                    } else if y >= 30 && y <= 50 {
                        Color::RED
                    } else {
                        Color::WHITE
                    },
                );
                let _res = self.canvas.fill_rect(Rect::new(
                    (x * SCALE_FACTOR) as i32, // relative x value for start of rectangle
                    ((y + bit) * SCALE_FACTOR) as i32, // relative w value for start of rectangle
                    SCALE_FACTOR,              // width of single pixel scaled for screen
                    SCALE_FACTOR,              // height of single pixel scaled for screen
                ));
                self.canvas.set_draw_color(Color::BLACK);
            }
            cmp_byte <<= 1;
        }
    }
}
