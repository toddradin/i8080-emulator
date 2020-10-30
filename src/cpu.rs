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
        // Macro for unconditional instructions. This macro will call the
        // provided function name ($func) along with an address ($addr) if
        // provided. This will return a tuple of (next_pc, cycles).
        macro_rules! unconditional {
            ($func:ident, $addr:ident) => {
                (self.$func($addr), instruction.cycles())
            };
            ($func:ident) => {
                (self.$func(), instruction.cycles())
            };
        }

        // Macro for a conditional branch instruction. This macro will call the
        // provided function name ($func) along with an address ($addr). If 
        // Some is returned from the function, the condition has been met. If
        // met, return a tuple with the returned address and the number of
        // cycles. Otherwise return a tuple with the pc incremented by the 
        // instruction size and the number of cycles.
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

        // Macro for a conditional subroutine instruction. This macro will call 
        // the provided function name ($func) along with an address ($addr) if 
        // provided. If Some is returned from the function, the condition has 
        // been met. If met, the instruction's higher cycle value is taken. 
        // Otherwise, take the default instruction size. The higher value is 
        // always the lower value + 6. Return a tuple with the next pc and the
        // number of cycles.
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

        // Macro for a logical non immediate instruction. This macro will call 
        // the provided function name ($func) along with an operand ($operand).
        // The operands accepted are either registers or memory addresses. 
        // Match the operand to the cpu's register or memory location and call 
        // the function. Return a tuple with the new pc and instruction cycles.
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

        // Macro for logical immediate instructions. This macro will call 
        // the provided function name ($func) along with a memory address 
        // ($val) and return a tuple with the new pc and number of cycles.
        macro_rules! logical_immediate {
            ($func:ident, $val: ident) => {{
                self.$func($val);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
        }

        // Macro for the instructions that modify flags or registers (Rotate
        // and Special groups). This macro will call the provided function
        // name ($func) and return a tuple with the new pc and number of 
        // cycles.
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
    // Unconditionally jump to the provided address.
    fn jmp(&self, addr: u16) -> u16 {
        self.jump(addr)
    }

    // Conditionally jump to the provided address if the carry flag is set.
    fn jc(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.carry {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the carry flag is not set.
    fn jnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the zero flag is set.
    fn jz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the zero flag is not set.
    fn jnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the sign flag is not set.
    fn jp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the sign flag is set.
    fn jm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the parity flag is set.
    fn jpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // Conditionally jump to the provided address if the parity flag is not set.
    fn jpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.parity {
            Some(self.jump(addr))
        } else {
            None
        }
    }

    // All of the different jump commands will call this. Return the address as
    // the pc will be set to the returned value.
    fn jump(&self, addr: u16) -> u16 {
        addr
    }

    // Return the H & L register as they will be moved to the pc.
    fn pchl(&self) -> u16 {
        self.registers.get_hl()
    }

    // Conditionally call a subroutine if the carry flag is set.
    fn cc(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the carry flag is not set.
    fn cnc(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the zero flag is set.
    fn cz(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the zero flag is not set.
    fn cnz(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the sign flag is not set.
    fn cp(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the sign flag is set.
    fn cm(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the parity flag is set.
    fn cpe(&self, addr: u16) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the parity flag is not set.
    fn cpo(&self, addr: u16) -> Option<u16> {
        if !self.condition_codes.parity {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Call a subroutine. First, push a return address onto the stack and then
    // return the new address the pc will be set to.
    fn call(&self, addr: u16) -> u16 {
        let pc = self.pc;
        self.push(pc);
        addr
    }

    // Conditionally call a return if the carry flag is set.
    fn rc(&self) -> Option<u16> {
        if self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the carry flag is not set.
    fn rnc(&self) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the zero flag is set.
    fn rz(&self) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the zero flag is not set.
    fn rnz(&self) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the sign flag is not set.
    fn rp(&self) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the sign flag is set.
    fn rm(&self) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the parity flag is set.
    fn rpe(&self) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the parity flag is not set.
    fn rpo(&self) -> Option<u16> {
        if !self.condition_codes.parity {
            Some(self.ret())
        } else {
            None
        }
    }

    // Unconditionally return from a subroutine, which pops an adress off the
    // stack.
    fn ret(&self) -> u16 {
        self.pop()
    }

    // Restart instruction. Pushes the pc onto the stack and returns a return
    // address.
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

    // The specified byte is logically ANDed bit by bit with the contents of
    // the accumulator. See and(&mut self, val).
    fn ana(&mut self, val: u8) {
        self.and(val)
    }

    // The specified byte is logically ANDed bit by bit with the contents of
    // the immediate address. See and(&mut self, val).
    fn ani(&mut self, val: u8) {
        self.and(val)
    }

    // The specified byte is EXCLUSIVE-ORed bit by bit with the contents of
    // the accumulator. See xra(&mut self, val).
    fn xra(&mut self, val: u8) {
        self.xor(val)
    }

    // The specified byte is EXCLUSIVE-ORed bit by bit with the contents of
    // the immediate address. See xra(&mut self, val).
    fn xri(&mut self, val: u8) {
        self.xor(val)
    }

    // The specified byte is logically ORed bit by bit with the contents of
    // the accumulator. See or(&mut self, val).
    fn ora(&mut self, val: u8) {
        self.or(val)
    }

    // The specified byte is logically ORed bit by bit with the contents of
    // the immediate address. See or(&mut self, val).
    fn ori(&mut self, val: u8) {
        self.or(val)
    }

    // The specified byte is compared with the contents of the accumulator. See
    // compare(&mut self, val).
    fn cmp(&mut self, val: u8) {
        self.compare(val)
    }

    // The specified byte is compared with the contents of the immediate
    // address. See compare(&mut self, val).
    fn cpi(&mut self, val: u8) {
        self.compare(val)
    }

    // The specified byte is logically ANDed bit by bit with the contents of
    // the accumulator or immediate address. The Carry bit is reset to zero.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn and(&mut self, val: u8) {
        // The 8080 logical AND instructions set the flag to reflect the
        // logical OR of bit 3 of the values involved in the AND operation.
        let aux_carry = ((self.registers.a | val) & 0x8) == 0x8;
        self.registers.a = self.registers.a & val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
        self.condition_codes.set_aux_carry(aux_carry);
    }

    // The specified byte is EXCLUSIVE-ORed bit by bit with the contents of
    // the accumulator or immediate address. The Carry and Auxiliary Carry bits
    // are reset.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn xor(&mut self, val: u8) {
        self.registers.a = self.registers.a ^ val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
        self.condition_codes.set_aux_carry(false);
    }

    // The specified byte is logically ORed bit by bit with the contents of the
    // accumulator or immediate address. The Carry and Auxiliary Carry bits are
    // reset.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn or(&mut self, val: u8) {
        self.registers.a = self.registers.a | val;

        self.condition_codes.reset_carry();
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
        self.condition_codes.set_aux_carry(false);
    }

    // The specified byte is compared to the contents of the accumulator. The
    // comparison is performed by internally subtracting the contents of REG
    // from the accumulator (leaving both unchanged) and setting the condition
    // bits according to the result. The Zero bit is set if the  quantities
    // are equal, and reset if they are unequal. Since a subtract operation is
    // performed, the Carry bit will be set if there is no carry out of bit 7,
    // indicating that the contents of REG are greater than the contents of
    // the accumulator, and reset otherwise.
    // Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn compare(&mut self, val: u8) {
        let val = self.registers.a.wrapping_sub(val);

        self.condition_codes.set_carry(self.registers.a < val);
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
        // Set aux_carry if the lower nibble of the accumulator is less than
        // the lower nibble of the value after subtraction.
        self.condition_codes
            .set_aux_carry((self.registers.a & 0xF) < (val & 0xF));
    }

    // Rotate the accumulator left. The Carry bit is set equal to the
    // high-order bit of the accumulator. The contents of the accumulator are
    // rotated one bit position to the left, with the high-order bit being
    // transferred to the low-order bit position of the accumulator.
    // Condition bits affected: Carry
    fn rlc(&mut self) {
        let carry = (self.registers.a & 0x80) >> 7;
        self.registers.a = self.registers.a << 1 | carry;
        self.condition_codes.carry = (self.registers.a & 0x01) > 0;
    }

    // Rotate the accumulator right. The carry bit is set equal to the
    // low-order bit of the accumulator. The contents of the accumulator are
    // rotated one bit position to the right, with the low-order bit being
    // transferred to the high-order bit position of the accumulator.
    // Condition bits affected: Carry
    fn rrc(&mut self) {
        let carry = (self.registers.a & 0x80) << 7;
        self.registers.a = self.registers.a >> 1 | carry;
        self.condition_codes.carry = (self.registers.a & 0x80) > 0;
    }

    // Rotate the accumulator left through carry. The contents of the
    // accumulator are rotated one bit position to the left. The high-order bit
    // of the accumulator replaces the Carry bit, while the Carry bit replaces
    // the high-order bit of the accumulator.
    // Condition bits affected: Carry
    fn ral(&mut self) {
        let carry_bit = if self.condition_codes.carry { 1 } else { 0 };
        let high_bit = self.registers.a & 0x80;
        self.registers.a = (self.registers.a << 1) | carry_bit;
        self.condition_codes.carry = high_bit == 0x80;
    }

    // Rotate the accumulator left through carry. The contents of the
    // accumulator are rotated one bit position to the right. The low-order bit
    // of the accumulator replaces the carry bit, while the carry bit replaces
    // the high-order bit of the accumulator.
    // Condition bits affected: Carry
    fn rar(&mut self) {
        let carry_bit = if self.condition_codes.carry { 1 } else { 0 };
        let low_bit = self.registers.a & 0x01;
        self.registers.a = (self.registers.a >> 1) | (carry_bit << 7);
        self.condition_codes.carry = low_bit == 0x01;
    }

    // Each bit of the contents of the accumulator is complemented (producing
    // the one's complement).
    // Condition bits affected: None
    fn cma(&mut self) {
        self.registers.a = !self.registers.a
    }

    // Set the carry bit is set to one.
    // Condition bits affected: Carry
    fn stc(&mut self) {
        self.condition_codes.carry = true
    }

    // Complement carry. If the Carry bit is not set, set it. If the Carry
    // bit is set, reset it.
    // Condition bits affected: Carry
    fn cmc(&mut self) {
        self.condition_codes.carry = !self.condition_codes.carry
    }

    // The eight-bit hexadecimal number in the accumulator is adjusted to form
    // two four-bit binary encoded digits.
    // Condition bits affected: Zero, Sign, Parity, Carry, Auxiliary Carry
    fn daa(&mut self) {
        // If the least significant four bits of the accumulator represents a
        // number greater than 9, or if the Auxiliary Carry bit is equal to
        // one, the accumulator is incremented by six. Otherwise, no
        // incrementing occurs. If a carry out of the least significant four
        // bits occurs, the Auxiliary Carry bit is set; otherwise it is reset.
        if (self.registers.a & 0x0F > 0x9) || self.condition_codes.aux_carry {
            let high_bit = self.registers.a & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x06);
            self.condition_codes.aux_carry = (self.registers.a & 0x8) < high_bit;
        }
        // If the most significant four bits of the accumulator now represent a
        // number greater than 9, or if the normal carry bit is equal to one,
        // the most significant four bits of the accumulator are incremented
        // by six. Otherwise, no incrementing occurs. If a carry out of the
        // most significant four bits occurs. the Carry bit is set; otherwise,
        // it is unaffected.
        if (self.registers.a & 0xF0 > 0x90) || self.condition_codes.carry {
            let high_bit = (self.registers.a >> 4) & 0x8;
            self.registers.a = self.registers.a.wrapping_add(0x60);
            if ((self.registers.a >> 4) & 0x8) < high_bit {
                self.condition_codes.set_carry(true);
            }
        }
        self.condition_codes.set_zero(self.registers.a);
        self.condition_codes.set_sign(self.registers.a);
        self.condition_codes.set_parity(self.registers.a);
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
        assert_eq!(cpu.condition_codes.aux_carry, true);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);

        cpu.registers.a = 0x2;
        cpu.registers.b = 0x5;
        cpu.execute(&Instruction::CMP(Operand::B));
        assert_eq!(cpu.registers.a, 0x2);
        assert_eq!(cpu.registers.b, 0x5);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
        assert_eq!(cpu.condition_codes.aux_carry, true);
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
        assert_eq!(cpu.condition_codes.aux_carry, true);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);
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
        assert_eq!(cpu.condition_codes.aux_carry, false);

        cpu.registers.a = 0x2;
        cpu.execute(&Instruction::CPI(0x40));
        assert_eq!(cpu.registers.a, 0x2);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.sign, false);
        assert_eq!(cpu.condition_codes.zero, false);
        assert_eq!(cpu.condition_codes.parity, false);
        assert_eq!(cpu.condition_codes.aux_carry, false);
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
