#[macro_use]
extern crate bitflags;
extern crate gl;
extern crate i8080;
extern crate sdl2;
use crate::display::Display;
use crate::i8080::cpu::Cpu;
use crate::io::{ControllerPort, Key, SpaceInvadersIO};
use crate::memory::SpaceInvadersMemory;

mod display;
mod io;
mod memory;

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
struct Game {
    events: sdl2::EventPump,
    machine: SpaceInvadersIO,
    cpu: Cpu<SpaceInvadersMemory>,
    display: Display,
}

impl Game {
    fn new() -> Self {
        let memory = SpaceInvadersMemory::new();
        let machine = SpaceInvadersIO::new();
        let cpu = Cpu::new(memory);

        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let display = Display::new(sdl_context);
        Game {
            events: event_pump,
            machine: machine,
            cpu: cpu,
            display: display,
        }
    }
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        const HERTZ: i32 = 2_000_000;
        const FPS: u8 = 60;
        const CYCLES_PER_FRAME: i32 = HERTZ / FPS as i32;
        const CYCLES_PER_HALF_FRAME: i32 = CYCLES_PER_FRAME / 2;

        let mut next_interrupt = 0x8;
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        self.machine.press(key, port);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        self.machine.release(key, port);
                    }
                }
                _ => {}
            }
        }

        // After every CYCLES_PER_HALF_FRAME, an interrupt should be triggered.
        // This will be run twice so that the correct number of cycles per
        // frame is reached.
        for _ in 0..2 {
            self.cpu.step(&mut self.machine, CYCLES_PER_HALF_FRAME);
            self.cpu.interrupt(next_interrupt);
            next_interrupt = if next_interrupt == 0x08 { 0x10 } else { 0x08 };
        }
        self.display.draw_display_whole(&mut self.cpu);
        std::thread::sleep(std::time::Duration::from_millis(15));

        emscripten_main_loop::MainLoopEvent::Continue
    }
}

fn main() -> Result<(), std::io::Error> {
    let game = Game::new();
    emscripten_main_loop::run(game);

    Ok(())
}
