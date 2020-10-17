use crate::condition_codes::ConditionCodes;
use crate::instruction::Instruction;
use crate::registers::Registers;

#[allow(dead_code)]
#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    sp: u8,
    pub pc: u16,
    memory: Vec<u8>, // This will eventually point to the loaded rom
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
            Instruction::JC(addr) => match self.jc(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JNC(addr) => match self.jnc(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JZ(addr) => match self.jz(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JNZ(addr) => match self.jnz(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JP(addr) => match self.jp(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JM(addr) => match self.jm(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JPE(addr) => match self.jpe(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::JPO(addr) => match self.jpo(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::PCHL => self.pchl(),
            Instruction::CALL(addr) => self.call(addr),
            Instruction::CC(addr) => match self.cc(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CNC(addr) => match self.cnc(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CZ(addr) => match self.cz(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CNZ(addr) => match self.cnz(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CP(addr) => match self.cp(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CM(addr) => match self.cm(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CPE(addr) => match self.cpe(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::CPO(addr) => match self.cpo(addr) {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RET => self.ret(),
            Instruction::RC => match self.rc() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RNC => match self.rnc() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RZ => match self.rz() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RNZ => match self.rnz() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RP => match self.rp() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RM => match self.rm() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RPE => match self.rpe() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RPO => match self.rpo() {
                None => self.pc.wrapping_add(instruction.size()),
                Some(next_pc) => next_pc,
            },
            Instruction::RST(addr) => self.rst(addr),
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
        self.jump(addr)
    }

    fn jc(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.cy {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.cy {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.z {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.z {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.s {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.s {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.p {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.p {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jump(&self, addr: u16) -> u16 {
        addr
    }

    fn pchl(&self) -> u16 {
        self.registers.get_hl()
    }

    fn cc(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.cy {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.cy {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.z {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.z {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.s {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.s {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.p {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.p {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn call(&self, addr: u16) -> u16 {
        let pc = self.pc;
        self.push(pc);
        addr
    }

    fn rc(&self) -> Option<u16> {
        if self.condition_codes.cy {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rnc(&self) -> Option<u16> {
        if !self.condition_codes.cy {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rz(&self) -> Option<u16> {
        if self.condition_codes.z {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rnz(&self) -> Option<u16> {
        if !self.condition_codes.z {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rp(&self) -> Option<u16> {
        if !self.condition_codes.s {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rm(&self) -> Option<u16> {
        if self.condition_codes.s {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rpe(&self) -> Option<u16> {
        if self.condition_codes.p {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rpo(&self) -> Option<u16> {
        if !self.condition_codes.p {
            Some(self.ret())
        } else {
            None
        }
    }

    fn ret(&self) -> u16 {
        self.pop()
    }

    fn rst(&self, addr: u8) -> u16 {
        self.call(addr as u16)
    }

    fn push(&self, addr: u16) -> u16 {
        // TODO
        0
    }

    fn pop(&self) -> u16 {
        // TODO
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOP
    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let instr = Instruction::NOP;
        let (next_pc, cycles) = cpu.execute(&instr);
        assert_eq!(cycles, Instruction::NOP.cycles());
    }
}
