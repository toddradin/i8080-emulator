#[macro_use]
extern crate bitflags;
extern crate i8080;
extern crate sdl2;
use crate::display::Display;
use crate::io::{ControllerPort, Key, SpaceInvadersIO};
use crate::memory::SpaceInvadersMemory;

mod display;
mod io;
mod memory;

use i8080::cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn keycode_to_key(keycode: Keycode) -> Option<(Key, ControllerPort)> {
    let key = match keycode {
        Keycode::Num0 => (Key::CREDIT, ControllerPort::P1),
        Keycode::Num2 => (Key::START2P, ControllerPort::P1),
        Keycode::Num1 => (Key::START1P, ControllerPort::P1),
        Keycode::W => (Key::SHOOT1P, ControllerPort::P1),
        Keycode::A => (Key::LEFT1P, ControllerPort::P1),
        Keycode::D => (Key::RIGHT1P, ControllerPort::P1),
        Keycode::I => (Key::SHOOT2P, ControllerPort::P2),
        Keycode::J => (Key::LEFT2P, ControllerPort::P2),
        Keycode::L => (Key::RIGHT2P, ControllerPort::P2),
        _ => return None,
    };

    Some(key)
}

fn main() -> Result<(), std::io::Error> {
    let memory = SpaceInvadersMemory::new();
    let machine = &mut SpaceInvadersIO::new();
    let cpu = &mut Cpu::new(memory);

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
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        machine.press(key, port);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        machine.release(key, port);
                    }
                }
                _ => {}
            }
        }

        // After every CYCLES_PER_HALF_FRAME, an interrupt should be triggered.
        // This will be run twice so that the correct number of cycles per
        // frame is reached.
        for _ in 0..2 {
            cpu.step(machine, CYCLES_PER_HALF_FRAME);
            cpu.interrupt(next_interrupt);
            next_interrupt = if next_interrupt == 0x08 { 0x10 } else { 0x08 };
        }

        display.draw_display_whole(cpu);
    }

    Ok(())
}
