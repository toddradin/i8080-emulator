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

        // Macro for the arithmetic and logic unit (ALU) non immediate instruction. 
        // This macro will call the provided function name ($func) along with an 
        // operand ($operand). The operands accepted are either registers or memory 
        // addresses. Match the operand to the cpu's register or memory location and  
        // call the function. Return a tuple with the new pc and instruction cycles.
        macro_rules! alu_non_immediate {
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
        
        // Macro for arithmetic and logic unit (ALU) immediate instructions. 
        // This macro will call the provided function name ($func) along with a 
        // memory address ($val) and return a tuple with the new pc and number 
        // of cycles.
        macro_rules! alu_immediate {
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
            Instruction::ANA(op) => alu_non_immediate!(ana, op),
            Instruction::XRA(op) => alu_non_immediate!(xra, op),
            Instruction::ORA(op) => alu_non_immediate!(ora, op),
            Instruction::CMP(op) => alu_non_immediate!(cmp, op),
            Instruction::ANI(val) => alu_immediate!(ani, val),
            Instruction::XRI(val) => alu_immediate!(xri, val),
            Instruction::ORI(val) => alu_immediate!(ori, val),
            Instruction::CPI(val) => alu_immediate!(cpi, val),
            Instruction::RLC => flag_or_register_modify!(rlc),
            Instruction::RRC => flag_or_register_modify!(rrc),
            Instruction::RAL => flag_or_register_modify!(ral),
            Instruction::RAR => flag_or_register_modify!(rar),
            Instruction::CMA => flag_or_register_modify!(cma),
            Instruction::STC => flag_or_register_modify!(stc),
            Instruction::CMC => flag_or_register_modify!(cmc),
            Instruction::DAA => flag_or_register_modify!(daa),
            Instruction::ADD(op) => alu_non_immediate!(add, op),
            Instruction::ADC(op) => alu_non_immediate!(adc, op),
            Instruction::SUB(op) => alu_non_immediate!(sub, op),
            Instruction::SBB(op) => alu_non_immediate!(sbb, op),
            Instruction::ADI(val) => alu_immediate!(adi, val),
            Instruction::ACI(val) => alu_immediate!(aci, val),
            Instruction::SUI(val) => alu_immediate!(sui, val),
            Instruction::SBI(val) => alu_immediate!(sbi, val),
            Instruction::INR(op) => {
                self.inr(op);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }
            Instruction::DCR(op) => {
                self.dcr(op);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }
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
        self.condition_codes.carry = (self.registers.a & 0x1) > 0;
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
        let low_bit = self.registers.a & 0x1;
        self.registers.a = (self.registers.a >> 1) | (carry_bit << 7);
        self.condition_codes.carry = low_bit == 0x1;
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
        if (self.registers.a & 0xF > 0x9) || self.condition_codes.aux_carry {
            let high_bit = self.registers.a & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x6);
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

    // The specified byte is added to the contents of the accumulator. 
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn add(&mut self, val: u8) {
        let reg_a = self.registers.a;
        let res: u16 = (reg_a as u16).wrapping_add(val as u16);
        // put result in accumulator
        self.registers.a = res as u8;
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(res > 0xFF);
        self.condition_codes
            .set_aux_carry((reg_a & 0xF) + (val & 0xF) > 0xF);
    }

    // The specified register or memory byte is incremented by one.
    // Condition bits affected: Zero, Sign, Parity, Auxiliary Carry
    fn inr(&mut self, reg: Operand) {
        let res = match reg {
            Operand::A => {
                self.registers.a = self.registers.a.wrapping_add(1);
                self.registers.a
            }
            Operand::B => {
                self.registers.b = self.registers.b.wrapping_add(1);
                self.registers.b
            }
            Operand::C => {
                self.registers.c = self.registers.c.wrapping_add(1);
                self.registers.c
            }
            Operand::D => {
                self.registers.d = self.registers.d.wrapping_add(1);
                self.registers.d
            }
            Operand::E => {
                self.registers.e = self.registers.e.wrapping_add(1);
                self.registers.e
            }
            Operand::H => {
                self.registers.h = self.registers.h.wrapping_add(1);
                self.registers.h
            }
            Operand::L => {
                self.registers.l = self.registers.l.wrapping_add(1);
                self.registers.l
            }
            Operand::M => {
                let hl = self.registers.get_hl() as usize;
                self.memory[hl] = self.memory[hl].wrapping_add(1);
                self.memory[hl]
            }
            _ => panic!("INR only accepts registers or a memory location"),
        };
        // update flags
        self.condition_codes.set_zero(res);
        self.condition_codes.set_sign(res);
        self.condition_codes.set_parity(res);
        self.condition_codes.set_aux_carry((res & 0xF) == 0x0);
    }

    // The specified register or memory byte is decremented by one.
    // Condition bits affected: Zero, Sign, Parity, Auxiliary Carry
    fn dcr(&mut self, reg: Operand) {
        let res = match reg {
            Operand::A => {
                self.registers.a = self.registers.a.wrapping_sub(1);
                self.registers.a
            }
            Operand::B => {
                self.registers.b = self.registers.b.wrapping_sub(1);
                self.registers.b
            }
            Operand::C => {
                self.registers.c = self.registers.c.wrapping_sub(1);
                self.registers.c
            }
            Operand::D => {
                self.registers.d = self.registers.d.wrapping_sub(1);
                self.registers.d
            }
            Operand::E => {
                self.registers.e = self.registers.e.wrapping_sub(1);
                self.registers.e
            }
            Operand::H => {
                self.registers.h = self.registers.h.wrapping_sub(1);
                self.registers.h
            }
            Operand::L => {
                self.registers.l = self.registers.l.wrapping_sub(1);
                self.registers.l
            }
            Operand::M => {
                let location = self.registers.get_hl() as usize;
                self.memory[location] = self.memory[location].wrapping_sub(1);
                self.memory[location]
            }
            _ => panic!("DCR only accepts registers or a memory location"),
        };
        // update flags
        self.condition_codes.reset_carry();
        self.condition_codes.set_zero(res);
        self.condition_codes.set_sign(res);
        self.condition_codes.set_parity(res);
        self.condition_codes.set_aux_carry((res & 0xF) != 0xF);
    }
    
    // The specified byte plus the content of the Carry bit is added to the contents 
    // of the accumulator.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry  
    fn adc(&mut self, val: u8) {
        let reg_a = self.registers.a;
        let carry: u8 = if self.condition_codes.carry { 1 } else { 0 };
        let res = (reg_a as u16)
            .wrapping_add(val as u16)
            .wrapping_add(carry as u16);
        // put result in accumulator
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(res > 0xFF);
        self.condition_codes
            .set_aux_carry((reg_a & 0xF) + (val & 0xF) + (carry & 0xF) > 0xF);
    }

    // The specified byte is subtracted from the accumulator. If there is no carry 
    // out of the high-order bit position, indicating that a borrow occurred, the 
    // Carry bit is set; otherwise it is reset.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry 
    fn sub(&mut self, val: u8) {
        let reg_a = self.registers.a;
        let res: u16 = (reg_a as u16).wrapping_sub(val as u16);
        // put result in accumulator
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(reg_a < val);
        self.condition_codes
            .set_aux_carry((reg_a as i8 & 0xF) - (val as i8 & 0xF) >= 0);
    }

    // The Carry bit is internally added to the contents of the specified byte. This 
    // value is then subtracted from the accumulator. 
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry 
    fn sbb(&mut self, val: u8) {
        let reg_a = self.registers.a;
        let borrow: u8 = if self.condition_codes.carry { 1 } else { 0 };
        let res: u16 = (reg_a as u16)
            .wrapping_sub(val as u16)
            .wrapping_sub(borrow as u16);
        // put result in accumulator
        self.registers.a = res as u8;
        // update flags
        self.condition_codes.set_zero(res as u8);
        self.condition_codes.set_sign(res as u8);
        self.condition_codes.set_parity(res as u8);
        self.condition_codes.set_carry(reg_a < val);
        self.condition_codes
            .set_aux_carry((reg_a as i8 & 0xF) - (val as i8 & 0xF - (borrow as i8)) >= 0);
    }

    // The byte of immediate data is added to the contents of the accumulator. 
    // See add(&mut self, val); 
    fn adi(&mut self, val: u8) {
        self.add(val);
    }

    // The byte of immediate data is added to the contents of the accumulator plus 
    // the contents of the carry bit. See adc(&mut self, val);
    fn aci(&mut self, val: u8) {
        self.adc(val);
    }

    // The byte of immediate data is subtracted from the contents of the accumulator
    // See. sub(&mut self, val).
    fn sui(&mut self, val: u8) {
        self.sub(val);
    }

    // The Carry bit is internally added to the byte of immediate data. This value 
    // is then subtracted from the accumulator. See sub(&mut self, val).
    fn sbi(&mut self, val: u8) {
        self.sbb(val);
    }
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
        cpu.registers.h = 0x1;
        cpu.registers.l = 0x2;
        let (next_pc, _) = cpu.execute(&Instruction::PCHL);
        assert_eq!(next_pc, 0x102);
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
    fn test_add() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x6C;
        cpu.registers.d = 0x2E;
        cpu.execute(&Instruction::ADD(Operand::D));

        assert_eq!(cpu.registers.a, 0x9A);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }

    #[test]
    fn test_adc() {
        // carry bit not set
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x42;
        cpu.registers.c = 0x3D;
        cpu.condition_codes.carry = false;
        cpu.execute(&Instruction::ADC(Operand::C));

        assert_eq!(cpu.registers.a, 0x7F);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
        assert_eq!(cpu.condition_codes.aux_carry, false);

        // carry bit set
        cpu.registers.a = 0x42;
        cpu.registers.c = 0x3D;
        cpu.condition_codes.carry = true;
        cpu.execute(&Instruction::ADC(Operand::C));

        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }

    #[test]
    fn test_sub() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x3E;
        cpu.execute(&Instruction::SUB(Operand::A));

        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, true);
        assert_eq!(cpu.condition_codes.parity, true);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }

    #[test]
    fn test_sbb() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x4;
        cpu.registers.l = 0x2;
        cpu.condition_codes.carry = true;
        cpu.execute(&Instruction::SBB(Operand::L));

        assert_eq!(cpu.registers.a, 0x1);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }

    #[test]
    fn test_inr() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x99;
        cpu.execute(&Instruction::INR(Operand::A));

        assert_eq!(cpu.registers.a, 0x9A);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
        assert_eq!(cpu.condition_codes.aux_carry, false);
    }

    #[test]
    fn test_dcr() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x3A;
        cpu.registers.l = 0x7C;
        cpu.memory[0x3A7C] = 0x40;
        cpu.execute(&Instruction::DCR(Operand::M));

        assert_eq!(cpu.memory[0x3A7C], 0x3F);
        assert_eq!(cpu.condition_codes.carry, false);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, true);
        assert_eq!(cpu.condition_codes.aux_carry, true);
    }
}
