use crate::condition_codes::ConditionCodes;
use crate::instruction::{Instruction, Operand};
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

    pub fn execute(&mut self, instruction: &Instruction) -> (u16, u8) {
        // possibly rename this to something more appropriate if other
        // instructions will use this
        macro_rules! unconditional {
            ($func:ident, $addr:ident) => {
                (self.$func($addr), instruction.cycles())
            };
            ($func:ident) => {
                (self.$func(), instruction.cycles())
            };
        }

        macro_rules! conditional_branch {
            ($func:ident, $addr:ident) => {
                match self.$func($addr) {
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
            ($func:ident, $addr:ident) => {
                match self.$func($addr) {
                    None => (
                        self.pc.wrapping_add(instruction.size()),
                        instruction.cycles(),
                    ),
                    Some(next_pc) => (next_pc, instruction.cycles() + 6),
                }
            };
            ($func:ident) => {
                match self.$func() {
                    None => (
                        self.pc.wrapping_add(instruction.size()),
                        instruction.cycles(),
                    ),
                    Some(next_pc) => (next_pc, instruction.cycles() + 6),
                }
            };
        }

        macro_rules! logical_non_immediate {
            ($func:ident, $operand: ident) => {{
                let val = match $operand {
                    Operand::A => self.registers.a,
                    Operand::B => self.registers.b,
                    Operand::C => self.registers.c,
                    Operand::D => self.registers.d,
                    Operand::E => self.registers.e,
                    Operand::H => self.registers.h,
                    Operand::L => self.registers.l,
                    Operand::M => self.memory[self.registers.get_hl() as usize],
                    _ => panic!(
                        "{:#x?} only accepts registers or a memory location",
                        instruction
                    ),
                };
                self.$func(val);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
        }

        macro_rules! logical_immediate {
            ($func:ident, $val: ident) => {{
                self.$func($val);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
        }

        macro_rules! flag_or_register_modify {
            ($func:ident) => {{
                self.$func();
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
        }

        let (pc, cycles) = match *instruction {
            Instruction::NOP => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::JMP(addr) => unconditional!(jmp, addr),
            Instruction::JC(addr) => conditional_branch!(jc, addr),
            Instruction::JNC(addr) => conditional_branch!(jnc, addr),
            Instruction::JZ(addr) => conditional_branch!(jz, addr),
            Instruction::JNZ(addr) => conditional_branch!(jnz, addr),
            Instruction::JP(addr) => conditional_branch!(jp, addr),
            Instruction::JM(addr) => conditional_branch!(jm, addr),
            Instruction::JPE(addr) => conditional_branch!(jpe, addr),
            Instruction::JPO(addr) => conditional_branch!(jpo, addr),
            Instruction::PCHL => unconditional!(pchl),
            Instruction::CALL(addr) => unconditional!(call, addr),
            Instruction::CC(addr) => conditional_subroutine!(cc, addr),
            Instruction::CNC(addr) => conditional_subroutine!(cnc, addr),
            Instruction::CZ(addr) => conditional_subroutine!(cz, addr),
            Instruction::CNZ(addr) => conditional_subroutine!(cnz, addr),
            Instruction::CP(addr) => conditional_subroutine!(cp, addr),
            Instruction::CM(addr) => conditional_subroutine!(cm, addr),
            Instruction::CPE(addr) => conditional_subroutine!(cpe, addr),
            Instruction::CPO(addr) => conditional_subroutine!(cpo, addr),
            Instruction::RET => unconditional!(ret),
            Instruction::RC => conditional_subroutine!(rc),
            Instruction::RNC => conditional_subroutine!(rnc),
            Instruction::RZ => conditional_subroutine!(rz),
            Instruction::RNZ => conditional_subroutine!(rnz),
            Instruction::RP => conditional_subroutine!(rp),
            Instruction::RM => conditional_subroutine!(rm),
            Instruction::RPE => conditional_subroutine!(rpe),
            Instruction::RPO => conditional_subroutine!(rpo),
            Instruction::RST(addr) => unconditional!(rst, addr),
            Instruction::ANA(op) => logical_non_immediate!(ana, op),
            Instruction::XRA(op) => logical_non_immediate!(xra, op),
            Instruction::ORA(op) => logical_non_immediate!(ora, op),
            Instruction::CMP(op) => logical_non_immediate!(cmp, op),
            Instruction::ANI(val) => logical_immediate!(ani, val),
            Instruction::XRI(val) => logical_immediate!(xri, val),
            Instruction::ORI(val) => logical_immediate!(ori, val),
            Instruction::CPI(val) => logical_immediate!(cpi, val),
            Instruction::RLC => flag_or_register_modify!(rlc),
            Instruction::RRC => flag_or_register_modify!(rrc),
            Instruction::RAL => flag_or_register_modify!(ral),
            Instruction::RAR => flag_or_register_modify!(rar),
            Instruction::CMA => flag_or_register_modify!(cma),
            Instruction::STC => flag_or_register_modify!(stc),
            Instruction::CMC => flag_or_register_modify!(cmc),
            Instruction::DAA => flag_or_register_modify!(daa),
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
        if self.condition_codes.carry {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    fn jpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.parity {
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
        if self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.call(addr))
        } else {
            None
        }
    }

    fn cpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.parity {
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
        if self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rnc(&self) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rz(&self) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rnz(&self) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rp(&self) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rm(&self) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rpe(&self) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.ret())
        } else {
            None
        }
    }

    fn rpo(&self) -> Option<u16> {
        if !self.condition_codes.parity {
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
        self.memory = addr;
        self.sp -= 2; 
        0
        //TODO:
    }

    fn pop(&self, addr: u16) -> u16 {
        // TODO
        self.memory
        self.sp += 2;
        self.memory[sp]
    }

    
    fn dad(&self, val: u16) {
        let res: u16 = val + 
        //TODO:
    }

    fn dcx(&self, val: u16) -> u16 {
        let res: u16 = val - 1;
        res
    }

    fn inx(&self, val: u16) -> u16 {
        let res: u16 = val + 1;
        res
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

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let instr = Instruction::NOP;
        let (_, cycles) = cpu.execute(&instr);
        assert_eq!(cycles, Instruction::NOP.cycles());
    }

    #[test]
    fn test_jmp() {
        let mut cpu = Cpu::new();
        let (next_pc, _) = cpu.execute(&Instruction::JMP(0x10FF));
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jc() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JC(0x10FF);
        cpu.condition_codes.carry = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.carry = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jnc() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JNC(0x10FF);
        cpu.condition_codes.carry = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.carry = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jz() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JZ(0x10FF);
        cpu.condition_codes.zero = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.zero = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jnz() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JNZ(0x10FF);
        cpu.condition_codes.zero = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.zero = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jp() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JP(0x10FF);
        cpu.condition_codes.sign = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.sign = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jm() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JM(0x10FF);
        cpu.condition_codes.sign = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.sign = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jpe() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JPE(0x10FF);
        cpu.condition_codes.parity = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.parity = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_jpo() {
        let mut cpu = Cpu::new();
        let instr = Instruction::JPO(0x10FF);
        cpu.condition_codes.parity = true;
        let (next_pc, _) = cpu.execute(&instr);
        assert_ne!(next_pc, 0x10FF);
        cpu.condition_codes.parity = false;
        let (next_pc, _) = cpu.execute(&instr);
        assert_eq!(next_pc, 0x10FF);
    }

    #[test]
    fn test_pchl() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x01;
        cpu.registers.l = 0x02;
        let (next_pc, _) = cpu.execute(&Instruction::PCHL);
        assert_eq!(next_pc, 0x0102);
    }

    // TODO add CALL and RET test once push/pop are complete

    #[test]
    fn test_ana() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFC;
        cpu.registers.b = 0xF;
        cpu.execute(&Instruction::ANA(Operand::B));
        assert_eq!(cpu.registers.a, 0xC);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
    }

    #[test]
    fn test_xra() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFC;
        cpu.registers.b = 0x1;
        cpu.execute(&Instruction::XRA(Operand::B));
        assert_eq!(cpu.registers.a, 0xFD);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_ora() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x33;
        cpu.registers.b = 0xF;
        cpu.execute(&Instruction::ORA(Operand::B));
        assert_eq!(cpu.registers.a, 0x3F);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
    }

    #[test]
    fn test_cmp() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xA;
        cpu.registers.b = 0x5;
        cpu.execute(&Instruction::CMP(Operand::B));
        assert_eq!(cpu.registers.a, 0xA);
        assert_eq!(cpu.registers.b, 0x5);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);

        cpu.registers.a = 0x2;
        cpu.registers.b = 0x5;
        cpu.execute(&Instruction::CMP(Operand::B));
        assert_eq!(cpu.registers.a, 0x2);
        assert_eq!(cpu.registers.b, 0x5);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_ani() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x3A;
        cpu.execute(&Instruction::ANI(0xF));
        assert_eq!(cpu.registers.a, 0xA);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
    }

    #[test]
    fn test_xri() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x3B;
        cpu.execute(&Instruction::XRI(0x81));
        assert_eq!(cpu.registers.a, 0xBA);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_ori() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xB5;
        cpu.execute(&Instruction::ORI(0xF));
        assert_eq!(cpu.registers.a, 0xBF);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_cpi() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4A;
        cpu.execute(&Instruction::CPI(0x40));
        assert_eq!(cpu.registers.a, 0x4A);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);

        cpu.registers.a = 0x2;
        cpu.execute(&Instruction::CPI(0x40));
        assert_eq!(cpu.registers.a, 0x2);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xF2;
        cpu.execute(&Instruction::RLC);
        assert_eq!(cpu.registers.a, 0xE5);
        assert_eq!(cpu.condition_codes.carry, true);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xF2;
        cpu.execute(&Instruction::RRC);
        assert_eq!(cpu.registers.a, 0x79);
        assert_eq!(cpu.condition_codes.carry, false);
    }

    #[test]
    fn test_ral() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xB5;
        cpu.execute(&Instruction::RAL);
        assert_eq!(cpu.registers.a, 0x6A);
        assert_eq!(cpu.condition_codes.carry, true);
    }

    #[test]
    fn test_rar() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x6A;
        cpu.condition_codes.carry = true;
        cpu.execute(&Instruction::RAR);
        assert_eq!(cpu.registers.a, 0xB5);
        assert_eq!(cpu.condition_codes.carry, false);
    }

    #[test]
    fn test_cma() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x51;
        cpu.execute(&Instruction::CMA);
        assert_eq!(cpu.registers.a, 0xAE);
    }

    #[test]
    fn test_stc() {
        let mut cpu = Cpu::new();
        cpu.execute(&Instruction::STC);
        assert_eq!(cpu.condition_codes.carry, true);
    }

    #[test]
    fn test_cmc() {
        let mut cpu = Cpu::new();
        let instr = Instruction::CMC;
        cpu.condition_codes.carry = false;
        cpu.execute(&instr);
        assert_eq!(cpu.condition_codes.carry, true);
        cpu.condition_codes.carry = true;
        cpu.execute(&instr);
        assert_eq!(cpu.condition_codes.carry, false);
    }

    #[test]
    fn test_daa() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x9B;
        cpu.condition_codes.carry = false;
        cpu.condition_codes.aux_carry = false;
        cpu.execute(&Instruction::DAA);
        assert_eq!(cpu.registers.a, 0x1);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x38;
        cpu.registers.e = 0x00;
        cpu.execute(&Instruction::INX(Operand::D));
        assert_eq!(cpu.registers.d, 0x39);
        assert_eq!(cpu.registers.e, 0xFF);
        cpu.sp = 0xFF;
        cpu.execute(&Instruction::INX(Operand::SP));
        assert_eq!(cpu.sp, 0x00);
    }

    #[test]
    fn test_dcx() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x98;
        cpu.registers.l = 0x00;
        cpu.execute(&Instruction::DCX(Operand::H));
        assert_eq!(cpu.registers.h, 0x97);
        assert_eq!(cpu.registers.l, 0xFF);
    }

    #[test]
    fn test_dad() {
        let mut cpu = Cpu::new();
        //TODO:
        cpu.execute(&Instruction::DAD);
    }

    #[test]
    fn test_push() {
        let mut cpu = Cpu::new();
        //TODO:
        cpu.execute(&Instruction::PUSH);
    }

    #[test]
    fn test_pop() {
        let mut cpu = Cpu::new();
        //TODO:
        cpu.execute(&Instruction::POP);
    }

    #[test]
    fn test_ei() {
        let mut cpu = Cpu::new();
        cpu.execute(&Instruction::EI);
        assert_eq!(cpu.interrupts_enabled, true);
    }

    #[test]
    fn test_di() {
        let mut cpu = Cpu::new();
        cpu.interrupts_enabled = true;
        cpu.execute(&Instruction::DI);
        assert_eq!(cpu.interrupts_enabled, false);
    }

    //TODO: not implemented yet
    #[test]
    fn test_input() { //IN opcode ('in' is a reserved keyword)
    
    }

    //TODO: not implemented yet
    #[test]
    fn test_output() { //OUT opcode

    }

    //#[test]
    //fn test_rim() {
    //not used in Space Invaders
    //}

    //#[test]
    //fn test_sim() {
    //not used in Space Invaders
    //}
}
