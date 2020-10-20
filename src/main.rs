mod condition_codes;
mod cpu;
mod instruction;

use cpu::{ Cpu };
use instruction::Instruction;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let buffer = fs::read("../roms/invaders.h")?;

    let mut cpu = Cpu::new();
    let mut pc = cpu.pc;

    println!("buffer len: {}", buffer.len());

    while cpu.pc < buffer.len() as u16 {
        let instr = Instruction::from(&buffer[cpu.pc as usize..]);
        println!("{:#x?}", instr);
        let (next_pc, cycles) = cpu.execute(&instr);
        println!("{} {}", next_pc, cycles);

        cpu.pc = next_pc;
    }

    Ok(())
}
