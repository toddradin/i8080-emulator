mod condition_codes;
mod cpu;
mod instruction;

use cpu::{ Cpu };
use instruction::Instruction;
use std::io::Read;
use std::fs::{ self, File };

fn load_roms() -> std::io::Result<Vec<u8>> {
    let mut files: Vec<u8> = Vec::new();
    let mut file = File::open("../roms/invaders.h")?;
    file.read_to_end(&mut files)?;
    file = File::open("../roms/invaders.g")?;
    file.read_to_end(&mut files)?;
    file = File::open("../roms/invaders.f")?;
    file.read_to_end(&mut files)?;
    file = File::open("../roms/invaders.e")?;
    file.read_to_end(&mut files)?;
    Ok(files)
}

fn main() -> Result<(), std::io::Error> {
    let mut buffer = load_roms()?;

    let mut cpu = Cpu::new();
    let mut pc = cpu.pc;

    println!("buffer len: {:#x?}", buffer.len());

    while cpu.pc < buffer.len() as u16 {
        let instr = Instruction::from(&buffer[cpu.pc as usize..]);
        println!("{:#x?}", instr);
        let (next_pc, cycles) = cpu.execute(&instr);
        println!("{:#x?} {}", next_pc, cycles);

        cpu.pc = next_pc;
    }

    Ok(())
}
