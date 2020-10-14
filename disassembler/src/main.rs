mod instruction;

fn main() -> Result<(), std::io::Error> {
    let buffer = std::fs::read("../../invaders.h")?;
    
    let mut pc = 0;
    while pc < buffer.len() {
        let instr = instruction::Instruction::from(&buffer[usize::from(pc)..]);
        println!("{:?}", instr);
        pc += instr.size() as usize;
    }

    Ok(())
}
