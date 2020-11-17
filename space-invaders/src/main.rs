#[macro_use]
extern crate bitflags;
extern crate i8080;
extern crate sdl2;
use crate::display::Display;
use crate::io::{Key, SpaceInvadersIO};

mod display;
mod io;

use i8080::cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;

fn load_roms(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut addr = 0x00;
    for f in ['h', 'g', 'f', 'e'].iter() {
        let mut file = File::open(format!("roms/invaders.{}", f))?;
        file.read(&mut buffer[addr..addr + 0x800])?;
        addr += 0x800;
    }
    Ok(())
}

fn keycode_to_key(keycode: Keycode) -> Option<Key> {
    let key = match keycode {
        Keycode::Num0 => Key::CREDIT,
        Keycode::Num2 => Key::START2P,
        Keycode::Num1 => Key::START1P,
        Keycode::W => Key::SHOOT1P,
        Keycode::A => Key::LEFT1P,
        Keycode::D => Key::RIGHT1P,
        Keycode::I => Key::SHOOT2P,
        Keycode::J => Key::LEFT2P,
        Keycode::L => Key::RIGHT2P,
        _ => return None,
    };

    Some(key)
}

fn main() -> Result<(), std::io::Error> {
    let cpu = &mut Cpu::new();
    let machine = &mut SpaceInvadersIO::new();
    match load_roms(&mut cpu.memory) {
        Ok(_) => (),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = Display::new(sdl_context);

    const HERTZ: i32 = 2_000_000;
    const FPS: u8 = 60;
    const CYCLES_PER_FRAME: i32 = HERTZ / FPS as i32;
    const CYCLES_PER_HALF_FRAME: i32 = CYCLES_PER_FRAME / 2;

    let mut next_interrupt = 0x8;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keycode_to_key(keycode) {
                        machine.press(key);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keycode_to_key(keycode) {
                        machine.release(key);
                    }
                }
                _ => {}
            }
        }

        // After every CYCLES_PER_HALF_FRAME, an interrupt should be triggered.
        // This will be run twice so that the correct number of cycles per
        // frame is reached.
        for _ in 0..2 {
            let mut cycles_to_run = CYCLES_PER_HALF_FRAME;
            while cycles_to_run > 0 {
                cycles_to_run -= cpu.step(machine) as i32;
            }

            if cpu.interrupts_enabled {
                match next_interrupt {
                    0x8 => {
                        display.draw_display(cpu, true);
                        cpu.interrupt(0x8);
                        next_interrupt = 0x10;
                    }
                    0x10 => {
                        display.draw_display(cpu, false);
                        cpu.interrupt(0x10);
                        next_interrupt = 0x8;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
