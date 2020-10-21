mod instruction;

use instruction::Instruction;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let buffer = fs::read("../roms/invaders.h")?;

    let mut pc = 0;
    while pc < buffer.len() {
        let instr = Instruction::from(&buffer[pc..]);
        instr.print(&buffer[pc..]);
        pc += instr.size() as usize;
    }

    Ok(())
}
