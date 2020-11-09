mod condition_codes;
mod cpu;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;
use rodio::Source;

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

fn play_audio() {
    //game uses OUT 3 and OUT 5 ports for sound.
    //watch for when output bits change and play sound when they do.
    //emulator101.com/cocoa-port-pt-5---sound.html

    //checking port 3
    if out_p3 != prev_out_p3 {
        //UFO
        if(out_p3 & 0x1) && !(prev_out_p3 & 0x1) {

        }

        //shoot
        if(out_p3 & 0x2) && !(prev_out_p3 & 0x2) {

        }
    }
    //ufo (high)

    
    //ufo (low)

    //fast invader 1

    //fast invader 2

    //fast invader 3

    //fast invader 4

    //shoot

    //death player

    //death invader


}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    match load_roms(&mut cpu.memory) {
        Ok(_) => (),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let mut i = 0;
    while cpu.pc < cpu.memory.len() as u16 {
        
        if cpu.memory[cpu.pc as usize] == 0xd3 {
            println!{"OUT TEST\n"};
            process::exit(1);
        }

        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);

        if instr == "OUT" {
            
        }

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
