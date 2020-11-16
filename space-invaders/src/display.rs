use i8080::cpu::Cpu;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

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

    pub fn draw_display(&mut self, cpu: &Cpu) {
        self.canvas.clear();
        for video_ram_byte in 0x2400..0x4000 {
            let offset = video_ram_byte - 0x2400;
            let x = (offset % 28) * 8;
            let y = offset / 28;
            if cpu.memory[video_ram_byte] > 0 {
                let cv = self.create_color_vect(cpu.memory[video_ram_byte]);
                for bit in 0..8 {
                    if cv[bit] {
                        self.canvas.set_draw_color(Color::WHITE);
                        self.draw_pixel((x + bit) as u32, y as u32);
                        self.canvas.set_draw_color(Color::BLACK);
                    }
                }
            }
        }
        self.canvas.present();
    }

    fn draw_pixel(&mut self, x: u32, y: u32) {
        self.canvas.fill_rect(Rect::new(
            (x * SCALE_FACTOR) as i32, // relative x value for start of rectangle
            (y * SCALE_FACTOR) as i32, // relative w value for start of rectangle
            SCALE_FACTOR,              // width of single pixel scaled for screen
            SCALE_FACTOR,              // height of single pixel scaled for screen
        ));
    }

    fn create_color_vect(&mut self, byte: u8) -> [bool; 8] {
        let mut vect = [false; 8];
        let mut cmp_byte: u8 = 1;
        for bit in (0..8).rev() {
            vect[bit] = if byte & cmp_byte != 0 { true } else { false };
            cmp_byte <<= 1;
        }
        vect
    }
}
