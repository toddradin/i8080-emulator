mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::{self, File};
use std::io::Read;

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
    let buffer = load_roms()?;
    let mut cpu = Cpu::new();

    while cpu.pc < buffer.len() as u16 {
        let instr = Instruction::from(&buffer[cpu.pc as usize..]);
        let (next_pc, cycles) = cpu.execute(&instr);
        println!(
            "{:#x?} \t\t\t next_pc: {:#x?} \t cycles: {}",
            instr, next_pc, cycles
        );

        cpu.pc = next_pc;
    }

    Ok(())
}
