use crate::condition_codes::ConditionCodes;
use crate::instruction::{Instruction, Operand};
use crate::registers::Registers;

use std::process;

#[allow(dead_code)]
pub struct Cpu {
    pub registers: Registers,
    pub sp: u16,
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

        // Macro for the instructions that modify flags or registers (Rotate
        // and Special groups). This macro will call the provided function
        // name ($func) and return a tuple with the new pc and number of
        // cycles.
        macro_rules! flag_or_register_modify {
            ($func:ident, $dst: ident, $src: ident) => {{
                self.$func($dst, $src);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
            ($func:ident, $addr: ident) => {{
                self.$func($addr);
                (
                    self.pc.wrapping_add(instruction.size()),
                    instruction.cycles(),
                )
            }};
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
            Instruction::PUSH(op) => flag_or_register_modify!(push, op),
            Instruction::POP(op) => flag_or_register_modify!(pop, op),
            Instruction::EI => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::DI => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::HLT => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::IN(input) => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::OUT(output) => (
                self.pc.wrapping_add(instruction.size()),
                instruction.cycles(),
            ),
            Instruction::ADD(op) => alu_non_immediate!(add, op),
            Instruction::ADC(op) => alu_non_immediate!(adc, op),
            Instruction::SUB(op) => alu_non_immediate!(sub, op),
            Instruction::SBB(op) => alu_non_immediate!(sbb, op),
            Instruction::ADI(val) => alu_immediate!(adi, val),
            Instruction::ACI(val) => alu_immediate!(aci, val),
            Instruction::SUI(val) => alu_immediate!(sui, val),
            Instruction::SBI(val) => alu_immediate!(sbi, val),
            Instruction::INR(op) => flag_or_register_modify!(inr, op),
            Instruction::DCR(op) => flag_or_register_modify!(dcr, op),
            Instruction::MOV(dest, src) => flag_or_register_modify!(mov, dest, src),
            Instruction::MVI(dest, val) => flag_or_register_modify!(mvi, dest, val),
            Instruction::LXI(dest, val) => flag_or_register_modify!(lxi, dest, val),
            Instruction::STAX(reg) => flag_or_register_modify!(stax, reg),
            Instruction::LDAX(reg) => flag_or_register_modify!(ldax, reg),
            Instruction::STA(addr) => flag_or_register_modify!(sta, addr),
            Instruction::LDA(addr) => flag_or_register_modify!(lda, addr),
            Instruction::SHLD(addr) => flag_or_register_modify!(shld, addr),
            Instruction::LHLD(addr) => flag_or_register_modify!(lhld, addr),
            Instruction::XCHG => flag_or_register_modify!(xchg),
            Instruction::XTHL => flag_or_register_modify!(xthl),
            Instruction::SPHL => flag_or_register_modify!(sphl),
            Instruction::DAD(val) => flag_or_register_modify!(dad, val),
            Instruction::INX(reg) => flag_or_register_modify!(inx, reg),
            Instruction::DCX(reg) => flag_or_register_modify!(dcx, reg),
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
    fn cc(&mut self, addr: u16) -> Option<u16> {
        if self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the carry flag is not set.
    fn cnc(&mut self, addr: u16) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the zero flag is set.
    fn cz(&mut self, addr: u16) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the zero flag is not set.
    fn cnz(&mut self, addr: u16) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the sign flag is not set.
    fn cp(&mut self, addr: u16) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the sign flag is set.
    fn cm(&mut self, addr: u16) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the parity flag is set.
    fn cpe(&mut self, addr: u16) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Conditionally call a subroutine if the parity flag is not set.
    fn cpo(&mut self, addr: u16) -> Option<u16> {
        if !self.condition_codes.parity {
            Some(self.call(addr))
        } else {
            None
        }
    }

    // Call a subroutine. First, push a return address onto the stack and then
    // return the new address the pc will be set to.
    fn call(&mut self, addr: u16) -> u16 {
        let pc = self.pc;
        //note: Moved some of the push() code here to help keep that function cleaner
        //can be changed later if there are issues with it.
        //self.push(pc);
        self.memory[self.sp as usize -1] = (pc >> 8) as u8;
        self.memory[self.sp as usize -2] = pc as u8;
        self.sp = self.sp.wrapping_sub(2);
        addr
    }

    // Conditionally call a return if the carry flag is set.
    fn rc(&mut self) -> Option<u16> {
        if self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the carry flag is not set.
    fn rnc(&mut self) -> Option<u16> {
        if !self.condition_codes.carry {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the zero flag is set.
    fn rz(&mut self) -> Option<u16> {
        if self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the zero flag is not set.
    fn rnz(&mut self) -> Option<u16> {
        if !self.condition_codes.zero {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the sign flag is not set.
    fn rp(&mut self) -> Option<u16> {
        if !self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the sign flag is set.
    fn rm(&mut self) -> Option<u16> {
        if self.condition_codes.sign {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the parity flag is set.
    fn rpe(&mut self) -> Option<u16> {
        if self.condition_codes.parity {
            Some(self.ret())
        } else {
            None
        }
    }

    // Conditionally call a return if the parity flag is not set.
    fn rpo(&mut self) -> Option<u16> {
        if !self.condition_codes.parity {
            Some(self.ret())
        } else {
            None
        }
    }

    // Unconditionally return from a subroutine, which pops an adress off the
    // stack.
    fn ret(&mut self) -> u16 {
        //note: Moved some of the push() code here to help keep that function cleaner
        //can be changed later if there are issues with it.
        //self.pop()
        let res = (self.memory[self.sp as usize] as u16) << 8 | self.memory[self.sp as usize + 1] as u16;
        self.sp = self.sp.wrapping_add(2);
        res
    }

    // Restart instruction. Pushes the pc onto the stack and returns a return
    // address.
    fn rst(&mut self, addr: u8) -> u16 {
        self.call(addr as u16)
    }

    // Push Data Onto Stack
    fn push(&mut self, reg: Operand) {
        match reg {
            Operand::B => {
                let res = self.registers.get_bc();
                self.memory[self.sp as usize -1] = (res >> 8) as u8;
                self.memory[self.sp as usize -2] = res as u8;
                self.sp = self.sp.wrapping_sub(2);
            }
            Operand::D => {
                let res = self.registers.get_de();
                self.memory[self.sp as usize -1] = (res >> 8) as u8;
                self.memory[self.sp as usize -2] = res as u8;
                self.sp = self.sp.wrapping_sub(2);
            }
            Operand::H => {
                let res = self.registers.get_hl();
                self.memory[self.sp as usize -1] = (res >> 8) as u8;
                self.memory[self.sp as usize -2] = res as u8;
                self.sp = self.sp.wrapping_sub(2);
            }
            Operand::PSW => {
                self.memory[self.sp as usize -1] = self.registers.a;
                self.memory[self.sp as usize -2] = self.condition_codes.flags_to_psw();
                self.sp = self.sp.wrapping_sub(2);
            }
            _ => {
                //TODO: write error message later
                unimplemented!();
                
            }
        };
    }

    // Pop Data Off Stack
    fn pop(&mut self, reg: Operand) {
        match reg {
            Operand::B => {
                self.registers.c = self.memory[self.sp as usize];
                self.registers.b = self.memory[self.sp as usize + 1];
                self.sp = self.sp.wrapping_add(2);
            }
            Operand::D => {
                self.registers.e = self.memory[self.sp as usize];
                self.registers.d = self.memory[self.sp as usize + 1];
                self.sp = self.sp.wrapping_add(2);
            }
            Operand::H => {
                self.registers.l = self.memory[self.sp as usize];
                self.registers.h = self.memory[self.sp as usize + 1];
                self.sp = self.sp.wrapping_add(2);
            }
            Operand::PSW => {
                // self.registers.a = self.memory[self.sp as usize];
                self.registers.a = self.memory[self.sp as usize + 1];
                let res = self.memory[self.sp as usize];
                self.condition_codes.psw_to_flags(res);
                self.sp = self.sp.wrapping_add(2);
            }
            _ => {
                //TODO: write error message later
                unimplemented!();
                
            }
        };
    }

    // Double Add
    // The 16-bit number in the specified register pair is added to the
    // 16-bit number held in the H and L registers using two's complement arithmetic.
    // The result replaces the contents of the H and L registers
    fn dad(&mut self, reg: Operand) {
        //let tmp = self.registers.get_hl(); 
        //let res = tmp.wrapping_add(val);  
        //self.condition_codes.set_carry((tmp + val) > 0xFFFF);   
        //self.registers.set_hl(res); 
        match reg {
            Operand::B => {
                let res = self.registers.get_bc();
                self.condition_codes.set_carry((res + self.registers.get_hl()) > 0xFFFF);
                self.registers.set_hl(res.wrapping_add(self.registers.get_hl()));
            }
            Operand::D => {
                let res = self.registers.get_de();
                self.condition_codes.set_carry((res + self.registers.get_hl()) > 0xFFFF);
                self.registers.set_hl(res.wrapping_add(self.registers.get_hl()));
            }
            Operand::H => {
                let res = self.registers.get_hl();
                self.condition_codes.set_carry((res + self.registers.get_hl()) > 0xFFFF);
                self.registers.set_hl(res.wrapping_add(self.registers.get_hl()));
            }
            Operand::SP => {
                let res = self.sp;
                self.condition_codes.set_carry((res + self.registers.get_hl()) > 0xFFFF);
                self.registers.set_hl(res.wrapping_add(self.registers.get_hl()));
            }
            _ => {
                //TODO: write error message later
                unimplemented!();
                
            }
        };
    }

    // Decrement Register Pair
    // The 16-bit number held in the specified register pair is decremented by one
    fn dcx(&mut self, reg: Operand) {
        match reg {
            Operand::B => {
                self.registers.set_bc(self.registers.get_bc().wrapping_sub(1));           
            }
            Operand::D => {
                self.registers.set_de(self.registers.get_de().wrapping_sub(1));
            }
            Operand::H => {
                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
            }
            Operand::SP => {
                self.sp = self.sp.wrapping_sub(1);
            }
            _ => {
                //TODO: write error message later
                unimplemented!();
                
            }
        };
    }

    // Increment Register Pair
    // The 16-bit number held in the specified register pair in incremented by one
    fn inx(&mut self, reg: Operand) {
        match reg {
            Operand::B => {
                self.registers.set_bc(self.registers.get_bc().wrapping_add(1));           
            }
            Operand::D => {
                self.registers.set_de(self.registers.get_de().wrapping_add(1));
            }
            Operand::H => {
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
            }
            Operand::SP => {
                self.sp = self.sp.wrapping_add(1);
            }
            _ => {
                //TODO: write error message later
                unimplemented!();
                
            }
        };
    }

    // Halt instruction
    // Emulator 101 says it may not be necessary to emulate and suggests exiting if encountered
    fn hlt(&self) {
        process::exit(1);
    }

    // IN: Input (in is a reserved keyword so 'fn input' is used instead)
    // An eight-bit data byte is read from input device number exp and replaces
    // the contents of the accumulator
    fn input(&self) { //IN is a reserved keyword
        //TODO: for now doesn't do anything
        //Emulator 101 says to revisit later
        //http://www.emulator101.com/io-and-special-group.html
    }

    // OUT: Output (changed to 'fn output' to match 'input')
    // The contents of the accumulator are sent to output device number exp
    fn output(&self) { //OUT
        //TODO: for now doesn't do anything
        //Emulator 101 says to revisit later
        //http://www.emulator101.com/io-and-special-group.html
    }

    // Enable Interrupts
    // Sets the interrupt flag
    fn ei(&mut self) {
        self.interrupts_enabled = true;
    }

    // Disable Interrupts
    // Clears the interrupt flag
    fn di(&mut self) {
        self.interrupts_enabled = false;
    }

    // No Operation
    // Execution proceeds with the next sequential instruction
    fn nop(&self) {
    }

    // Load SP From H and L
    fn sphl(&mut self) {
        self.sp = self.registers.get_hl();
    }

    // Exchange Stack
    fn xthl(&mut self) {
        let tmp_h = self.registers.h;
        let tmp_l = self.registers.l;

        self.registers.h = self.memory[self.sp as usize + 1];
        self.registers.l = self.memory[self.sp as usize];
        self.memory[self.sp as usize] = tmp_l;
        self.memory[self.sp as usize + 1] = tmp_h;
    }

    //fn rim(&self) {
    //  not used for Space Invaders
    //}

    //fn sim(&self) {
    //  not used for Space Invaders
    //}

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
        self.condition_codes.carry = (self.registers.a & 0x1) > 0;
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
        let low_bit = self.registers.a & 0x1;
        self.registers.a = (self.registers.a >> 1) | (carry_bit << 7);
        self.condition_codes.carry = low_bit == 0x1;
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
            self.registers.a = self.registers.a.wrapping_add(0x6);
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
        self.condition_codes.set_aux_carry((res & 0xF) == 0xF);
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

    // One byte of data is moved from the register specified by src (the source
    // register) to the register specified by dst (the destination register).
    // The data re- places the contents of the destination register; the source
    // remains unchanged.
    // Condition bits affected: None
    fn mov(&mut self, dest: Operand, src: Operand) {
        let src = match src {
            Operand::A => self.registers.a,
            Operand::B => self.registers.b,
            Operand::C => self.registers.c,
            Operand::D => self.registers.d,
            Operand::E => self.registers.e,
            Operand::H => self.registers.h,
            Operand::L => self.registers.l,
            Operand::M => self.memory[self.registers.get_hl() as usize],
            _ => panic!("MOV only accepts registers or a memory location",),
        };

        match dest {
            Operand::A => self.registers.a = src,
            Operand::B => self.registers.b = src,
            Operand::C => self.registers.c = src,
            Operand::D => self.registers.d = src,
            Operand::E => self.registers.e = src,
            Operand::H => self.registers.h = src,
            Operand::L => self.registers.l = src,
            Operand::M => self.memory[self.registers.get_hl() as usize] = src,
            _ => panic!("MOV only accepts registers or a memory location",),
        }
    }

    // The byte of immediate data is stored in the specified register or memory
    // byte.
    // Condition bits affected: None
    fn mvi(&mut self, dest: Operand, val: u8) {
        match dest {
            Operand::A => self.registers.a = val,
            Operand::B => self.registers.b = val,
            Operand::C => self.registers.c = val,
            Operand::D => self.registers.d = val,
            Operand::E => self.registers.e = val,
            Operand::H => self.registers.h = val,
            Operand::L => self.registers.l = val,
            Operand::M => self.memory[self.registers.get_hl() as usize] = val,
            _ => panic!("MVI only accepts registers or a memory location",),
        }
    }

    // The third byte of the instruction (the most significant 8 bits of the
    // 16-bit immediate data) is loaded into the first register of the
    // specified pair, while the second byte of the instruction (the least
    // significant 8 bits of the 16-bit immediate data) is loaded into the
    // second register of the specified pair. If SP is specified as the
    // register pair, the second byte of the instruction replaces the least
    // significant 8 bits of the stack pointer, while the third byte of the
    // instruction replaces the most significant 8 bits of the stack pointer.
    // Condition bits affected: None
    fn lxi(&mut self, dest: Operand, val: u16) {
        match dest {
            Operand::B => self.registers.set_bc(val),
            Operand::D => self.registers.set_de(val),
            Operand::H => self.registers.set_hl(val),
            Operand::SP => self.sp = val,
            _ => panic!("LXI only accepts B, D, H, or SP as destinations",),
        }
    }

    // The contents of the accumulator are stored in the memory location
    // addressed by registers B and C, or by registers D and E.
    // Condition bits affected: None
    fn stax(&mut self, reg: Operand) {
        match reg {
            Operand::B => self.memory[self.registers.get_bc() as usize] = self.registers.a,
            Operand::D => self.memory[self.registers.get_de() as usize] = self.registers.a,
            _ => panic!("STAX only accepts B and D as operands",),
        }
    }

    // The contents of the memory location addressed by registers B and C, or
    // by registers D and E, replace the contents of the accumulator.
    // Condition bits affected: None
    fn ldax(&mut self, reg: Operand) {
        match reg {
            Operand::B => self.registers.a = self.memory[self.registers.get_bc() as usize],
            Operand::D => self.registers.a = self.memory[self.registers.get_de() as usize],
            _ => panic!("LDAX only accepts B and D as operands",),
        }
    }

    // The contents of the accumulator replace the byte at the memory address given
    // Condition bits affected: None
    fn sta(&mut self, addr: u16) {
        self.memory[addr as usize] = self.registers.a;
    }

    // The contents at the memory address given replaces the contents of the accumulator
    // Condition bits affected: None
    fn lda(&mut self, addr: u16) {
        self.registers.a = self.memory[addr as usize];
    }

    // The contents of the L register are stored at the memory address given and the
    // contents of the H register are stored at the next higher memory address.
    // Condition bits affected: None
    fn shld(&mut self, addr: u16) {
        self.memory[addr as usize] = self.registers.l;
        self.memory[(addr as usize).wrapping_add(1)] = self.registers.h;
    }

    // The byte at the memory address formed replaces the contents of the L register.
    // The byte at the next higher memory address replaces the contents of the H register.
    // Condition bits affected: None
    fn lhld(&mut self, addr: u16) {
        self.registers.l = self.memory[addr as usize];
        self.registers.h = self.memory[(addr as usize).wrapping_add(1)];
    }

    // The 16 bits of data held in the H and L registers are exchanged with the 16 bits
    // of data held in the D and E registers.
    // Condition bits affected: None
    fn xchg(&mut self) {
        let temp = self.registers.get_hl();
        self.registers.set_hl(self.registers.get_de());
        self.registers.set_de(temp);
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

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x38;
        cpu.registers.e = 0xFF;
        cpu.execute(&Instruction::INX(Operand::D));
        assert_eq!(cpu.registers.d, 0x39);
        assert_eq!(cpu.registers.e, 0x00);
        cpu.sp = 0xFFFF;
        cpu.execute(&Instruction::INX(Operand::SP));
        assert_eq!(cpu.sp, 0x0000);
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
        cpu.registers.b = 0x33;
        cpu.registers.c = 0x9F;
        cpu.registers.h = 0xA1;
        cpu.registers.l = 0x7B;
        cpu.condition_codes.carry = true;
        cpu.execute(&Instruction::DAD(Operand::B));
        assert_eq!(cpu.registers.h, 0xD5);
        assert_eq!(cpu.registers.l, 0x1A);
        assert_eq!(cpu.condition_codes.carry, false);
    }

    #[test]
    fn test_push() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x8F;
        cpu.registers.e = 0x9D;
        cpu.sp = 0x3A2C;
        cpu.execute(&Instruction::PUSH(Operand::D));
        assert_eq!(cpu.memory[0x3A2B], 0x8F);
        assert_eq!(cpu.memory[0x3A2A], 0x9D);
        assert_eq!(cpu.sp, 0x3A2A);

        //PUSH PSW
        cpu.registers.a = 0x1F;
        cpu.sp = 0x502A;
        cpu.condition_codes.carry = true;
        cpu.condition_codes.zero = true;
        cpu.condition_codes.parity = true;
        cpu.condition_codes.sign = false;
        cpu.condition_codes.aux_carry = false;
     
        cpu.execute(&Instruction::PUSH(Operand::PSW));
        assert_eq!(cpu.memory[0x5029], 0x1F);
        assert_eq!(cpu.memory[0x5028], 0x47);
        assert_eq!(cpu.sp, 0x5028);
    }

    #[test]
    fn test_pop() {
        let mut cpu = Cpu::new();
        cpu.memory[0x1239] = 0x3D;
        cpu.memory[0x123A] = 0x93;
        cpu.sp = 0x1239;
        cpu.execute(&Instruction::POP(Operand::H));
        assert_eq!(cpu.registers.l, 0x3D);
        assert_eq!(cpu.registers.h, 0x93);
        assert_eq!(cpu.sp, 0x123B);

        //POP PSW
        cpu.memory[0x2C00] = 0xC3;
        cpu.memory[0x2C01] = 0xFF;
        cpu.sp = 0x2C00;
        cpu.execute(&Instruction::POP(Operand::PSW));
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.condition_codes.carry, true);
        assert_eq!(cpu.condition_codes.zero, true);
        assert_eq!(cpu.condition_codes.aux_carry, false);
        assert_eq!(cpu.condition_codes.sign, true);
        assert_eq!(cpu.condition_codes.parity, false);
    }

    #[test]
    fn test_ei() {
        let mut cpu = Cpu::new();
        cpu.interrupts_enabled = false;
        cpu.ei();
        assert_eq!(cpu.interrupts_enabled, true);
    }

    #[test]
    fn test_di() {
        let mut cpu = Cpu::new();
        cpu.interrupts_enabled = true;
        cpu.di();
        assert_eq!(cpu.interrupts_enabled, false);
    }

    #[test]
    fn test_sphl() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x50;
        cpu.registers.l = 0x6C;
        cpu.execute(&Instruction::SPHL);
        assert_eq!(cpu.sp, 0x506C);
    }

    #[test]
    fn test_xthl() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x10AD;
        cpu.registers.h = 0x0B;
        cpu.registers.l = 0x3C;
        cpu.memory[0x10AD] = 0xF0;
        cpu.memory[0x10AE] = 0x0D;
        cpu.execute(&Instruction::XTHL);
        assert_eq!(cpu.registers.h, 0x0D);
        assert_eq!(cpu.registers.l, 0xF0);
        assert_eq!(cpu.memory[0x10AD], 0x3C);
        assert_eq!(cpu.memory[0x10AE], 0x0B);
    }

    //TODO: main function not yet implemented
    //#[test]
    //fn test_input() { //IN opcode ('in' is a reserved keyword)
    
    //}

    //TODO: main function not yet implemented
    //#[test]
    //fn test_output() { //OUT opcode

    //}

    //#[test]
    //fn test_rim() {
    //not used in Space Invaders
    //}

    //#[test]
    //fn test_sim() {
    //not used in Space Invaders
    //}

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

    #[test]
    fn test_mov() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0;
        cpu.registers.e = 0x2B;
        cpu.execute(&Instruction::MOV(Operand::A, Operand::E));
        assert_eq!(cpu.registers.a, 0x2B);
        assert_eq!(cpu.registers.e, 0x2B);

        cpu.registers.a = 0x5A;
        cpu.registers.h = 0x2B;
        cpu.registers.l = 0xE9;
        cpu.execute(&Instruction::MOV(Operand::M, Operand::A));
        assert_eq!(cpu.memory[0x2BE9], 0x5A);
        assert_eq!(cpu.registers.a, 0x5A);
        assert_eq!(cpu.registers.h, 0x2B);
        assert_eq!(cpu.registers.l, 0xE9);
    }

    #[test]
    fn test_mvi() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.registers.b, 0);
        cpu.execute(&Instruction::MVI(Operand::B, 0x3C));
        assert_eq!(cpu.registers.b, 0x3C);
    }

    #[test]
    fn test_lxi() {
        let mut cpu = Cpu::new();
        cpu.execute(&Instruction::LXI(Operand::H, 0x103));
        assert_eq!(cpu.registers.h, 0x1);
        assert_eq!(cpu.registers.l, 0x3);
    }

    #[test]
    fn test_stax() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x5C;
        cpu.registers.b = 0x3F;
        cpu.registers.c = 0x16;
        cpu.execute(&Instruction::STAX(Operand::B));
        assert_eq!(cpu.memory[0x3F16], 0x5C);
    }

    #[test]
    fn test_ldax() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x93;
        cpu.registers.e = 0x8B;
        cpu.memory[0x938B] = 0x5C;
        cpu.execute(&Instruction::LDAX(Operand::D));
        assert_eq!(cpu.registers.a, 0x5C);
    }

    #[test]
    fn test_sta() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.execute(&Instruction::STA(0x5B3));
        assert_eq!(cpu.memory[0x5b3], 0xFF);
    }

    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new();
        cpu.memory[0x300] = 0xB;
        cpu.execute(&Instruction::LDA(0x300));
        assert_eq!(cpu.registers.a, 0xB);
    }

    #[test]
    fn test_shld() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0xAE;
        cpu.registers.l = 0x29;
        cpu.execute(&Instruction::SHLD(0x10A));
        assert_eq!(cpu.memory[0x10A], 0x29);
        assert_eq!(cpu.memory[0x10B], 0xAE);
    }

    #[test]
    fn test_lhld() {
        let mut cpu = Cpu::new();
        cpu.memory[0x25B] = 0xFF;
        cpu.memory[0x25C] = 0x3;
        cpu.execute(&Instruction::LHLD(0x25B));
        assert_eq!(cpu.registers.l, 0xFF);
        assert_eq!(cpu.registers.h, 0x3);
    }

    #[test]
    fn test_xchg() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x33;
        cpu.registers.e = 0x55;
        cpu.registers.h = 0x0;
        cpu.registers.l = 0xFF;
        cpu.execute(&Instruction::XCHG);
        assert_eq!(cpu.registers.d, 0x0);
        assert_eq!(cpu.registers.e, 0xFF);
        assert_eq!(cpu.registers.h, 0x33);
        assert_eq!(cpu.registers.l, 0x55);
    }
}
