use crate::condition_codes::ConditionCodes;
use crate::instruction::{Instruction, Operand};
use crate::registers::Registers;

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
        // TODO
        0
    }

    fn pop(&self) -> u16 {
        // TODO
        0
    }

    fn ana(&mut self, val: u8) {
        self.and(val)
    }

    fn ani(&mut self, val: u8) {
        self.and(val)
    }

    fn xra(&mut self, val: u8) {
        self.xor(val)
    }

    fn xri(&mut self, val: u8) {
        self.xor(val)
    }

    fn ora(&mut self, val: u8) {
        self.or(val)
    }

    fn ori(&mut self, val: u8) {
        self.or(val)
    }

    fn cmp(&mut self, val: u8) {
        self.compare(val)
    }

    fn cpi(&mut self, val: u8) {
        self.compare(val)
    }

    fn and(&mut self, val: u8) {
        self.registers.a = self.registers.a & val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
    }

    fn xor(&mut self, val: u8) {
        self.registers.a = self.registers.a ^ val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
    }

    fn or(&mut self, val: u8) {
        self.registers.a = self.registers.a | val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
    }

    fn compare(&mut self, val: u8) {
        let val = self.registers.a.wrapping_sub(val);

        self.condition_codes.set_carry(self.registers.a < val);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
    }

    fn rlc(&mut self) {
        let carry = (self.registers.a & 0x80) >> 7;
        self.registers.a = self.registers.a << 1 | carry;
        self.condition_codes.carry = (self.registers.a & 0x01) > 0;
    }

    fn rrc(&mut self) {
        let carry = (self.registers.a & 0x80) << 7;
        self.registers.a = self.registers.a >> 1 | carry;
        self.condition_codes.carry = (self.registers.a & 0x80) > 0;
    }

    fn ral(&mut self) {
        let carry_bit = if self.condition_codes.carry { 1 } else { 0 };
        let high_bit = self.registers.a & 0x80;
        self.registers.a = (self.registers.a << 1) | carry_bit;
        self.condition_codes.carry = high_bit == 0x80;
    }

    fn rar(&mut self) {
        let carry_bit = if self.condition_codes.carry { 1 } else { 0 };
        let low_bit = self.registers.a & 0x01;
        self.registers.a = (self.registers.a >> 1) | (carry_bit << 7);
        self.condition_codes.carry = low_bit == 0x01;
    }

    fn cma(&mut self) {
        self.registers.a = !self.registers.a
    }

    fn stc(&mut self) {
        self.condition_codes.carry = true
    }

    fn cmc(&mut self) {
        self.condition_codes.carry = !self.condition_codes.carry
    }

    fn daa(&mut self) {
        if (self.registers.a & 0x0F > 0x9) || self.condition_codes.aux_carry {
            let high_bit = self.registers.a & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x06);
            self.condition_codes.aux_carry = (self.registers.a & 0x8) < high_bit;
        }
        if (self.registers.a & 0xF0 > 0x90) || self.condition_codes.carry {
            let high_bit = (self.registers.a >> 4) & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x60);
            if ((self.registers.a >> 4) & 0x8) < high_bit {
                self.condition_codes.set_carry(true);
            }
        }
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
    }

    // arithmetic group
    fn add(&mut self, val: u8) {                                        // val is value containted in register
        let reg_a = self.registers.a;                                   
        let res: u16 = (reg_a as u16).wrapping_add(val as u16);
        // put result in accumulator 
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(res > 0xFF);
        self.condition_codes.set_aux_carry((reg_a & 0x0F) + (val & 0x0F) > 0x0F);
    }

    fn inr(&mut self, val: u8) -> u8 {
        let res = val.wrapping_add(1);   
        // update flags         
        self.condition_codes.set_zero(res);
        self.condition_codes.set_sign(res);
        self.condition_codes.set_parity(res);
        self.condition_codes.set_aux_carry((val & 0x0F) == 0x00);
        res
    }

    fn dcr(&mut self, val: u8) -> u8 {
        let res = val.wrapping_sub(1);   
        // update flags         
        self.condition_codes.set_zero(res);
        self.condition_codes.set_sign(res);
        self.condition_codes.set_parity(res);
        self.condition_codes.set_aux_carry((val & 0x0F) == 0x00);
        res
    } 

    fn adc(&mut self, val: u8) {
        let reg_a = self.registers.a;
        let carry: u8 = if self.condition_codes.carry {1} else {0};
        let res = (reg_a as u16).wrapping_add(val as u16).wrapping_add(carry as u16);
        // put result in accumulator 
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(res > 0xFF);
        self.condition_codes.set_aux_carry((reg_a & 0x0F) + (val & 0x0F) + (carry & 0x0F) > 0x0F);
    }

    fn sub(&mut self, val: u8){
        let reg_a = self.registers.a;                                   
        let res: u16 = (reg_a as u16).wrapping_sub(val as u16);
        // put result in accumulator 
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(reg_a < val);
        self.condition_codes.set_aux_carry((reg_a as i8 & 0x0F) - (val as i8 & 0x0F) >= 0);
    }

    fn sbb(&mut self, val: u8){
        let reg_a = self.registers.a;
        let borrow: u8 = if self.condition_codes.carry {1} else {0};                                   
        let res: u16 = (reg_a as u16).wrapping_sub(val as u16).wrapping_sub(borrow as u16);
        // put result in accumulator 
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(reg_a < val);
        self.condition_codes.set_aux_carry((reg_a as i8 & 0x0F) - (val as i8 & 0x0F - (borrow as i8)) >= 0);
    }

    fn adi(&mut self, val: u8) {
        self.add(val);
    }

    fn aci(&mut self, val: u8) {
        self.adc(val);
    }

    fn sui(&mut self, val: u8){
        self.sub(val);
    }

    fn sbi(&mut self, val: u8){
        self.sbb(val);
    }

    // inx
    // dcx
    // dad

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
}
