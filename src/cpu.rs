use crate::condition_codes::ConditionCodes;
use crate::instruction::Instruction;
use crate::registers::Registers;

use std::process;

#[allow(dead_code)]
pub struct Cpu {
    pub registers: Registers,
    pub sp: u8,
    pub pc: u16,
    pub memory: [u8; 0xFFFF],
    pub condition_codes: ConditionCodes,
    interrupts_enabled: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            sp: 0,
            pc: 0,
            memory: [0; 0xFFFF],
            condition_codes: Default::default(),
            interrupts_enabled: false,
        }
    }

    pub fn execute(&self, instruction: &Instruction) -> (u16, u8) {
        // possibly rename this to something more appropriate if other
        // instructions will use this
        macro_rules! unconditional_instruction {
            ($F:ident, $P:ident) => {
                (self.$F($P), instruction.cycles())
            };
            ($F:ident) => {
                (self.$F(), instruction.cycles())
            };
        }

        macro_rules! conditional_branch {
            ($F:ident, $P:ident) => {
                match self.$F($P) {
                    None => (
                        self.pc.wrapping_add(instruction.size()),
                        instruction.cycles(),
                    ),
                    Some(next_pc) => (next_pc, instruction.cycles()),
                }
            };
        }

        // When an instruction's action is taken, the instruction's higher
        // cycle value is taken. Otherwise, take the lower value. The higher
        // value is always the lower value + 6.
        macro_rules! conditional_subroutine {
            ($F:ident, $P:ident) => {
                match self.$F($P) {
                    None => (
                        self.pc.wrapping_add(instruction.size()),
                        instruction.cycles(),
                    ),
                    Some(next_pc) => (next_pc, instruction.cycles() + 6),
                }
            };
            ($F:ident) => {
                match self.$F() {
                    None => (
                        self.pc.wrapping_add(instruction.size()),
                        instruction.cycles(),
                    ),
                    Some(next_pc) => (next_pc, instruction.cycles() + 6),
                }
            };
        }

        let (pc, cycles) = match *instruction {
            Instruction::NOP => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::JMP(addr) => unconditional_instruction!(jmp, addr),
            Instruction::JC(addr) => conditional_branch!(jc, addr),
            Instruction::JNC(addr) => conditional_branch!(jnc, addr),
            Instruction::JZ(addr) => conditional_branch!(jz, addr),
            Instruction::JNZ(addr) => conditional_branch!(jnz, addr),
            Instruction::JP(addr) => conditional_branch!(jp, addr),
            Instruction::JM(addr) => conditional_branch!(jm, addr),
            Instruction::JPE(addr) => conditional_branch!(jpe, addr),
            Instruction::JPO(addr) => conditional_branch!(jpo, addr),
            Instruction::PCHL => unconditional_instruction!(pchl),
            Instruction::CALL(addr) => unconditional_instruction!(call, addr),
            Instruction::CC(addr) => conditional_subroutine!(cc, addr),
            Instruction::CNC(addr) => conditional_subroutine!(cnc, addr),
            Instruction::CZ(addr) => conditional_subroutine!(cz, addr),
            Instruction::CNZ(addr) => conditional_subroutine!(cnz, addr),
            Instruction::CP(addr) => conditional_subroutine!(cp, addr),
            Instruction::CM(addr) => conditional_subroutine!(cm, addr),
            Instruction::CPE(addr) => conditional_subroutine!(cpe, addr),
            Instruction::CPO(addr) => conditional_subroutine!(cpo, addr),
            Instruction::RET => unconditional_instruction!(ret),
            Instruction::RC => conditional_subroutine!(rc),
            Instruction::RNC => conditional_subroutine!(rnc),
            Instruction::RZ => conditional_subroutine!(rz),
            Instruction::RNZ => conditional_subroutine!(rnz),
            Instruction::RP => conditional_subroutine!(rp),
            Instruction::RM => conditional_subroutine!(rm),
            Instruction::RPE => conditional_subroutine!(rpe),
            Instruction::RPO => conditional_subroutine!(rpo),
            Instruction::RST(addr) => unconditional_instruction!(rst, addr),
            _ => unimplemented!(
                "execute instruction {:#x?} has not yet been implemented",
                instruction
            ),
        };
        (pc, cycles)
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

    //TODO: remove extra space
    fn push(&self, addr: u16) -> u16 {
        self.memory = addr;
        self.sp -= 2; 
        0
    }

    fn pop(&self, addr: u16) -> u16 {
        // TODO
        self.memory
        self.sp += 2;
        self.memory[sp]
    }

    fn dad(&self, val: u16) {
        let res: u16 = val + 
    }

    fn dcx(&self) {

    }

    fn inx(&self, val: u16) {
        let res: u16 = val + 1;
        val

    }

    fn hlt(&self) {
        //Halt instruction. exits program.
        process::exit(1);
    }

    fn input(&self) { //IN is a reserved keyword
        //TODO: for now doesn't do anything
        //Emulator 101 says to implement later
        //http://www.emulator101.com/io-and-special-group.html
    }

    fn output(&self) { //OUT
        //TODO: for now doesn't do anything
        //Emulator 101 says to implement later
        //http://www.emulator101.com/io-and-special-group.html
    }

    fn ei(&self) {
        self.interrupts_enabled = true;
    }

    fn di(&self) {
        self.interrupts_enabled = false;
    }

    fn nop(&self) {
        //no operation
    }

    // fn rim(&self) {
    //  not used for Space Invaders
    // }

    // fn sim(&sim) {
    //  not used for Space Invaders
    //}
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
