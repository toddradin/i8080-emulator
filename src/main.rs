mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;
use std::process;

fn load_roms(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut addr = 0x00;
    for f in ['h', 'g', 'f', 'e'].iter() {
        let mut file = File::open(format!("roms/invaders.{}", f))?;
        file.read(&mut buffer[addr..addr + 0x800])?;
        addr += 0x800;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    //match load_test_rom(&mut cpu.memory) {
    //    Ok(_) => (),
    //    Err(error) => panic!("Problem opening the file: {:?}", error),
    //}

    let offset = 0x100;
    let buffer = include_bytes!("../roms/cpudiag.bin");
    cpu.memory[offset as usize..(buffer.len() + offset as usize)].copy_from_slice(buffer);
    cpu.pc = 0x100;

    cpu.memory[368] = 0x7;
    let debug = false;

    let mut i = 0;
    while cpu.pc < cpu.memory.len() as u16 {
        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);
        println!("{:?} {:?}", i, instr);
        let (next_pc, cycles) = cpu.execute(&instr);
        cpu.pc = next_pc;

        if debug {
            println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
            println!("cycles: {}", cycles);
            println!("{:#x?}", cpu.condition_codes);
            println!("{:#x?}\n", cpu.registers);
        }
        i += 1;

        if cpu.pc == 0x689 {
            panic!("Encountered error at instruction {:#x?}", instr);
        }

        if cpu.pc == 0x69B {
            println!("Successfully ran test rom");
            process::exit(0)
        }
    }

    Ok(())
}
