#[macro_use]
extern crate bitflags;
extern crate i8080;

mod display;
mod io;

use crate::display::Display;
use crate::io::{Key, SpaceInvadersIO};

use i8080::cpu::Cpu;
use i8080::instruction::Instruction;

use std::fs::File;
use std::io::Read;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time::Duration;

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

    // TEST DISPLAY WITH JUNK MEMORY -- to be removed
    let mut diag = 128;
    for i in 0..9 {
        cpu.memory[0x2400 + 28 * i] = diag;
        println!("{:#x?}: {:b}", 0x2400 + 28 * i, cpu.memory[0x2400 + 28 * i]);
        diag >>= 1;
    }

    let debug = false;
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

        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);
        let (next_pc, cycles) = cpu.execute(&instr, machine);
        cpu.pc = next_pc;
        if debug {
            println!("{:?}", instr);
            println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
            println!("cycles: {}", cycles);
            println!("{:#x?}", cpu.condition_codes);
            println!("{:#x?}\n", cpu.registers);
        }

        // TODO: work on interuppts and timing

        if !cpu.interrupts_enabled {
            display.draw_display(cpu);
        }
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
