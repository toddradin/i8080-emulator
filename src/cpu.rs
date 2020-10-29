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

        macro_rules! modify_flags_registers {
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
            Instruction::RLC => modify_flags_registers!(rlc),
            Instruction::RRC => modify_flags_registers!(rrc),
            Instruction::RAL => modify_flags_registers!(ral),
            Instruction::RAR => modify_flags_registers!(rar),
            Instruction::CMA => modify_flags_registers!(cma),
            Instruction::STC => modify_flags_registers!(stc),
            Instruction::CMC => modify_flags_registers!(cmc),
            Instruction::DAA => modify_flags_registers!(daa),
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
        self.registers.a = (self.registers.a >> 1) | carry_bit;
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
        if (self.registers.a & 0x0F > 0x9) || self.condition_codes.accumulator {
            let high_bit = self.registers.a & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x06);
            self.condition_codes.accumulator = (self.registers.a & 0x8) < high_bit;
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
