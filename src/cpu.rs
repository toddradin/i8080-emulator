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
        Cpu {
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            h: Default::default(),
            l: Default::default(),

            sp: 0x00,
            pc: 0x0,

            memory: Default::default(),
            condition_codes: Default::default(),
            interrupts_enabled: true,
        }
    }

    pub fn execute(&self, instruction: &Instruction) -> (u16, u8) {
        let pc = match *instruction {
            Instruction::NOP => self.pc.wrapping_add(instruction.size()),
            Instruction::JMP(addr) => self.jmp(addr),
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
        let bytes = u16::to_le_bytes(addr);
        ((bytes[1] as u16) << 8) | (bytes[0] as u16)
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
