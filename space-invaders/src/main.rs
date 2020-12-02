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
mod sound;

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

        // Each step runs all the instructions in order to reach the required
        // cycles per frame.
        cpu.step(machine);
        display.draw_display_whole(cpu);
    }

    Ok(())
}
