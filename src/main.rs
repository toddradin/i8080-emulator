mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;

use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioDevice};
use std::time::Duration;

use std::process;

struct AudioData {
    bytes: Vec<u8>,
    position: usize
}

impl AudioCallback for AudioData {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        let (start, end) = (self.position, self.position + out.len());
        self.position += out.len();

        let audio_data = &self.bytes[start..end];
        for(src, dst) in audio_data.iter().zip(out.iter_mut()) {
            *dst = *src;
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

//     //rodio documentation source: docs.rs/rodio/0.13.0/rodio
//     let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
//     //checking port 3
//     if out_p3 != prev_out_p3 {
//         //UFO high and low
//         //special case to be figured out in testing
//         if(out_p3 & 0x1) && !(prev_out_p3 & 0x1) {

//         }

//         //shoot
//         if(out_p3 & 0x2) && !(prev_out_p3 & 0x2) {
//             let sound = File::open("./sfx/shoot.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         //player death
//         if(out_p3 & 0x4) && !(prev_out_p3 & 0x4) {
//             let sound = File::open("./sfx/explosion.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         //invader death
//         if(out_p3 & 0x8) && !(prev_out_p3 & 0x8) {
//             let sound = File::open("./sfx/invaderkilled.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         prev_out_p3 = out_p3;
//     }

//     //checking port 5
//     if out_p5 != prev_out_p5 {
//         //invader 1
//         if(out_p5 & 0x1) && !(prev_out_p5 & 0x1) {
//             let sound = File::open("./sfx/fastinvader1.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         //invader 2
//         if(out_p5 & 0x2) && !(prev_out_p5 & 0x2) {
//             let sound = File::open("./sfx/fastinvader2.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         //invader 3
//         if(out_p5 & 0x4) && !(prev_out_p5 & 0x4) {
//             let sound = File::open("./sfx/fastinvader3.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
//         }

//         //invader 4
//         if(out_p5 & 0x8) && !(prev_out_p5 & 0x8) {
//             let sound = File::open("./sfx/fastinvader4.wav").unwrap();
//             let source = rodio::Decoder::new(BufReader::new(sound)).unwrap();
//             device.resume();
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
    let audio_subsytem = sdl_context.audio().unwrap();

    let mut i = 0;
    while cpu.pc < cpu.memory.len() as u16 {
        
        if cpu.memory[cpu.pc as usize] == 0xd3 {
            println!{"OUT TEST\n"};
            process::exit(1);
        }

        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);

        // if instr == "OUT" {
        //     play_audio();
        // }

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
