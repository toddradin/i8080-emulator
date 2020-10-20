use crate::condition_codes::ConditionCodes;
use crate::instruction::{ Instruction, Operand, Register };

#[allow(dead_code)]
#[derive(Default)]
struct Cpu {
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
    pc: u8,

    memory: Vec<u8>,
    condition_codes: ConditionCodes,
    interrupts_enabled: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&self, instruction: Instruction) -> u8 {
        match instruction {
            Instruction::NOP => {},
            Instruction::LXI(reg, val) => self.lxi(reg, val),
            _ => unimplemented!(
                "execute instruction {:#x?} has not yet been implemented",
                instruction
            ),
        }
        instruction.cycles()
    }
}

impl Cpu {
    fn lxi(&self, reg: Register, operand: Operand) {
        let val = match operand {
            Operand::D16(val) => val,
            _=> panic!()
        };
        let v = Instruction::to_bytes_16(val);
        println!("{:?}", v);
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
