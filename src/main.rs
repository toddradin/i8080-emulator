mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;

extern crate sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV};

use std::time::Duration;
use std::thread;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    position: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        //Referenced from Rust SDL2 documentation:
        //https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs
        for dst in out.iter_mut() {
            //required for scaling the volume properly
            let pre_scale = *self.data.get(self.position).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            *dst = (scaled_signed_float + 128.0) as u8;
            self.position += 1;
        }
    }
}

fn load_roms(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut addr = 0x00;
    for f in ['h', 'g', 'f', 'e'].iter() {
        let mut file = File::open(format!("roms/invaders.{}", f))?;
        file.read(&mut buffer[addr..addr + 0x800])?;
        addr += 0x800;
    }
    Ok(())
}

// fn play_audio() {
//     //game uses OUT 3 and OUT 5 ports for sound.
//     //watch for when output bits change and play sound when they do.
//     //source: emulator101.com/cocoa-port-pt-5---sound.html

//     let sdl_context = sdl2::init().unwrap();

//     let audio_subsystem = sdl_context.audio().unwrap();
//     let desired_spec = AudioSpecDesired {
//         freq: Some(44100),
//         channels: Some(1),
//         samples: None
//     };
    
//     //checking port 3
//     if out_p3 != prev_out_p3 {
//         //TODO: UFO high and low repeating
//         //special case to be figured out in testing
//         if(out_p3 & 0x1) && !(prev_out_p3 & 0x1) {

//         }

//         //shoot
//         if(out_p3 & 0x2) && !(prev_out_p3 & 0x2) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/shoot.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         //player death
//         if(out_p3 & 0x4) && !(prev_out_p3 & 0x4) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/explosion.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         //invader death
//         if(out_p3 & 0x8) && !(prev_out_p3 & 0x8) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/invaderkilled.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         prev_out_p3 = out_p3;
//     }

//     //checking port 5
//     if out_p5 != prev_out_p5 {
//         //invader 1
//         if(out_p5 & 0x1) && !(prev_out_p5 & 0x1) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/fastinvader1.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         //invader 2
//         if(out_p5 & 0x2) && !(prev_out_p5 & 0x2) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/fastinvader2.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         //invader 3
//         if(out_p5 & 0x4) && !(prev_out_p5 & 0x4) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/fastinvader3.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         //invader 4
//         if(out_p5 & 0x8) && !(prev_out_p5 & 0x8) {
//             let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
//                 let wav = AudioSpecWAV::load_wav("./sfx/fastinvader4.wav").unwrap();

//                 let data = wav.buffer().to_vec();

//                 Sound {
//                 data: data,
//                 volume: 0.20,
//                 position: 0,
//             }
//             }).unwrap();
//             audio_device.resume();
//             thread::sleep(Duration::from_millis(16));
//         }

//         prev_out_p5 = out_p5;
//     }
// }

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    match load_roms(&mut cpu.memory) {
        Ok(_) => (),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let sdl_context = sdl2::init().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };

    let mut i = 0;
    while cpu.pc < cpu.memory.len() as u16 {
        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);

        //AUDIO TEST
        //Plays the specified sound effect after each instruction
        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            let wav = AudioSpecWAV::load_wav("./sfx/fastinvader3.wav").unwrap();

            let data = wav.buffer().to_vec();

            Sound {
            data: data,
            volume: 0.20,
            position: 0,
        }
        }).unwrap();
        audio_device.resume();
        thread::sleep(Duration::from_millis(16));
        //

        let (next_pc, cycles) = cpu.execute(&instr);
        cpu.pc = next_pc;

        println!("{:?} {:?}", i, instr);
        println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
        println!("cycles: {}", cycles);
        println!("{:#x?}", cpu.condition_codes);
        println!("{:#x?}\n", cpu.registers);
        i += 1;
    }

    Ok(())
}
