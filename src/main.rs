mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;
use std::process;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;


const WIDTH: u32 = 224;
const HEIGHT: u32 = 256;
const SCALE_UP: u32 = 4;


fn load_roms(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut addr = 0x00;
    for f in ['h', 'g', 'f', 'e'].iter() {
        let mut file = File::open(format!("roms/invaders.{}", f))?;
        file.read(&mut buffer[addr..addr + 0x800])?;
        addr += 0x800;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    match load_roms(&mut cpu.memory) {
       Ok(_) => (),
       Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let offset = 0x100;
    let buffer = include_bytes!("../roms/cpudiag.bin");
    cpu.memory[offset as usize..(buffer.len() + offset as usize)].copy_from_slice(buffer);
    cpu.pc = 0x100;

    cpu.memory[368] = 0x7;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("i8080", WIDTH * SCALE_UP, HEIGHT * SCALE_UP)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit => {
                    // TODO: figure out how to exit program
                    break 'running
                },
                _ => {
                    // this where keyboard input would go
                }
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);
        let (next_pc, cycles) = cpu.execute(&instr);
        cpu.pc = next_pc; 
        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
    
    Ok(())
}
