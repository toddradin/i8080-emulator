use crate::condition_codes::ConditionCodes;
use crate::instruction::{ Instruction, Register };

#[allow(dead_code)]
#[derive(Default)]
pub struct Cpu {
    // Registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    // Pointers
    sp: u8,
    pub pc: u16,

    memory: Vec<u8>,
    condition_codes: ConditionCodes,
    interrupts_enabled: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute(&self, instruction: &Instruction) -> (u16, u8) {
        let pc = match *instruction {
            Instruction::NOP => self.pc.wrapping_add(instruction.size()),
            Instruction::JMP(addr) => self.jmp(addr),
            Instruction::JNZ(addr) => self.jnz(addr),
            _ => unimplemented!(
                "execute instruction {:#x?} has not yet been implemented",
                instruction
            ),
        };

        (pc, instruction.cycles())
    }
}

impl Cpu {
    fn jmp(&self, addr: u16) -> u16 {
        addr
    }
    
    fn jnz(&self, addr: u16) -> u16 {
        addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOP
    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let pc = cpu.execute(Instruction::NOP);
        assert_eq!(pc, Instruction::NOP.cycles());
    }
}
