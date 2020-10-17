use std::fmt;

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
//
// EACH OPERATION ADDED HERE WILL NEED TO BE ADDED TO THE MATCH IN THE CORRESPONDING
// fmt FUNCTION. 
enum Operation {
    NOP,
    JMP(Operand),
    PUSH(Register),
    MVI(Register, Operand), //FIX ME
    STA(Operand),
    LXI(Register, Operand), // FIX ME
    STAX(Register),
    INX(Register),
    INR(Register),
    DCR(Register),
    RLC,
    DAD(Register),
    LDAX(Register), 
    DCX(Register), 
    RRC,
    RAL, 
    RAR,
    SHLD(Operand),
    DAA, 
    LHLD(Operand),
    CMA, 
    STC,
    LDA(Operand),
    CMC,
    MOV(Register, Register),
    ACI(Operand),
    ADI(Operand),
    ANI(Operand),
    CALL(Operand),
    CC(Operand),
    CM(Operand),
    CMP(Register),
    CNC(Operand),
    CP(Operand),
    CPE(Operand),
    CPI(Operand),
    CPO(Operand),
    CNZ(Operand),
    CZ(Operand),
    DI,
    EI,
    IN(Operand),
    JC(Operand),
    JM(Operand),
    JNC(Operand),
    JNZ(Operand),
    JP(Operand),
    JPE(Operand),
    JPO(Operand),
    JZ(Operand),
    ORA(Register),
    ORI(Operand),
    OUT(Operand),
    PCHL,
    POP(Register),
    RC,
    RET,
    RM,
    RNC,
    RNZ,
    RP,
    RPE,
    RPO,
    RST(Operand),
    RZ,
    SBI(Operand),
    SPHL,
    SUI(Operand),
    XCHG,
    XRI(Operand),
    XTHL
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    SP,
    PSW
}

enum Operand {
    A16(u16),
    D8(u8),
    D16(u16)
}

pub struct Instruction {
    op: Operation,
    size: u8,
    cycles: u8
}

impl Instruction {
    fn decode_op(bytes: &[u8]) -> Result<Instruction, ()> {
        let opcode = bytes[0];

        let instruction = match opcode {
            0x00 | 0x10 | 0x20 | 0x30 | 
            0x08 | 0x18 | 0x28 | 0x38  => Instruction {
                op: Operation::NOP,
                size: 1,
                cycles: 4
            },
            0x01 => Instruction {
                op: Operation::LXI(Register::B, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x02 => Instruction {
                op: Operation::STAX(Register::B),
                size: 1,
                cycles: 7
            },
            0x03 => Instruction {
                op: Operation::INX(Register::B),
                size: 1,
                cycles: 5
            },
            0x04 => Instruction {
                op: Operation::INR(Register::B),
                size: 1,
                cycles: 5
            },
            0x05 => Instruction {
                op: Operation::DCR(Register::B),
                size: 1,
                cycles: 5
            },
            0x06 => Instruction {
                op: Operation::MVI(Register::B, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x07 => Instruction {
                op: Operation::RLC,
                size: 1,
                cycles: 4
            },
            0x09 => Instruction {
                op: Operation::DAD(Register::B),
                size: 1,
                cycles: 10
            },
            0x0a => Instruction {
                op: Operation::LDAX(Register::B),
                size: 1,
                cycles: 7
            },
            0x0b => Instruction {
                op: Operation::DCX(Register::B),
                size: 1,
                cycles: 5
            },
            0x0c => Instruction {
                op: Operation::INR(Register::C),
                size: 1,
                cycles: 5
            },
            0x0d => Instruction {
                op: Operation::DCR(Register::C),
                size: 1,
                cycles: 5
            },
            0x0e => Instruction {
                op: Operation::MVI(Register::C, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x0f => Instruction {
                op: Operation::RRC,
                size: 1,
                cycles: 4
            },
            0x11 => Instruction {
                op: Operation::LXI(Register::D, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x12 => Instruction {
                op: Operation::STAX(Register::D),
                size: 1,
                cycles: 7
            },
            0x13 => Instruction {
                op: Operation::INX(Register::D),
                size: 1,
                cycles: 5
            },
            0x14 => Instruction {
                op: Operation::INR(Register::D),
                size: 1,
                cycles: 5
            },
            0x15 => Instruction {
                op: Operation::DCR(Register::D),
                size: 1,
                cycles: 5
            },
            0x16 => Instruction {
                op: Operation::MVI(Register::D, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x17 => Instruction {
                op: Operation::RAL,
                size: 1,
                cycles: 4
            },
            0x19 => Instruction {
                op: Operation::DAD(Register::D),
                size: 1,
                cycles: 10
            },
            0x1a => Instruction {
                op: Operation::LDAX(Register::D),
                size: 1,
                cycles: 7
            },
            0x1b => Instruction {
                op: Operation::DCX(Register::D),
                size: 1,
                cycles: 5
            },
            0x1c => Instruction {
                op: Operation::INR(Register::E),
                size: 1,
                cycles: 5
            },
            0x1d => Instruction {
                op: Operation::DCR(Register::E),
                size: 1,
                cycles: 5
            },
            0x1e => Instruction {
                op: Operation::MVI(Register::E, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x1f => Instruction {
                op: Operation::RAR,
                size: 1,
                cycles: 4
            },
            0x21 => Instruction {
                op: Operation::LXI(Register::H, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x22 => Instruction {
                op: Operation::SHLD(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 16
            },
            0x23 => Instruction {
                op: Operation::INX(Register::H),
                size: 1,
                cycles: 5
            },
            0x24 => Instruction {
                op: Operation::INR(Register::H),
                size: 1,
                cycles: 5
            },
            0x25 => Instruction {
                op: Operation::DCR(Register::H),
                size: 1,
                cycles: 5
            },
            0x26 => Instruction {
                op: Operation::MVI(Register::H, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x27 => Instruction {
                op: Operation::DAA,
                size: 1,
                cycles: 4
            },
            0x29 => Instruction {
                op: Operation::DAD(Register::H),
                size: 1,
                cycles: 10
            },
            0x2a => Instruction {
                op: Operation::LHLD(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 16
            },
            0x2b => Instruction {
                op: Operation::DCX(Register::H),
                size: 1,
                cycles: 5
            },
            0x2c => Instruction {
                op: Operation::INR(Register::L),
                size: 1,
                cycles: 5
            },
            0x2d => Instruction {
                op: Operation::DCR(Register::L),
                size: 1,
                cycles: 5
            },
            0x2e => Instruction {
                op: Operation::MVI(Register::L, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x2f => Instruction {
                op: Operation::CMA,
                size: 1,
                cycles: 4
            },
            0x31 => Instruction {
                op: Operation::LXI(Register::SP, Operand::D16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0x32 => Instruction {
                op: Operation::STA(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 13
            },
            0x33 => Instruction {
                op: Operation::INX(Register::SP),
                size: 1,
                cycles: 5
            },
            0x34 => Instruction {
                op: Operation::INR(Register::M),
                size: 1,
                cycles: 10
            },
            0x35 => Instruction {
                op: Operation::DCR(Register::M),
                size: 1,
                cycles: 10
            },
            0x36 => Instruction {
                op: Operation::MVI(Register::M, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 10
            },
            0x37 => Instruction {
                op: Operation::STC,
                size: 1,
                cycles: 4
            },
            0x39 => Instruction {
                op: Operation::DAD(Register::SP),
                size: 1,
                cycles: 10
            },
            0x3a => Instruction {
                op: Operation::LDA(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 13
            },
            0x3b => Instruction {
                op: Operation::DCX(Register::SP),
                size: 1,
                cycles: 5
            },
            0x3c => Instruction {
                op: Operation::INR(Register::A),
                size: 1,
                cycles: 5
            },
            0x3d => Instruction {
                op: Operation::DCR(Register::A),
                size: 1,
                cycles: 5
            },
            0x3e => Instruction {
                op: Operation::MVI(Register::A, Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0x3f => Instruction {
                op: Operation::CMC,
                size: 1,
                cycles: 4
            },
            0x40 =>Instruction {
                op: Operation::MOV(Register::B, Register::B),
                size: 1,
                cycles: 5
            },
            0x41 =>Instruction {
                op: Operation::MOV(Register::B, Register::C),
                size: 1,
                cycles: 5
            },
            0x42 =>Instruction {
                op: Operation::MOV(Register::B, Register::D),
                size: 1,
                cycles: 5
            },
            0x43 =>Instruction {
                op: Operation::MOV(Register::B, Register::E),
                size: 1,
                cycles: 5
            },
            0x44 =>Instruction {
                op: Operation::MOV(Register::B, Register::H),
                size: 1,
                cycles: 5
            },
            0x45 =>Instruction {
                op: Operation::MOV(Register::B, Register::L),
                size: 1,
                cycles: 5
            },
            0x46 =>Instruction {
                op: Operation::MOV(Register::B, Register::M),
                size: 1,
                cycles: 7
            },
            0x47 =>Instruction {
                op: Operation::MOV(Register::B, Register::A),
                size: 1,
                cycles: 5
            },
            0x48 =>Instruction {
                op: Operation::MOV(Register::C, Register::B),
                size: 1,
                cycles: 5
            },
            0x49 =>Instruction {
                op: Operation::MOV(Register::C, Register::C),
                size: 1,
                cycles: 5
            },
            0x4a =>Instruction {
                op: Operation::MOV(Register::C, Register::D),
                size: 1,
                cycles: 5
            },
            0x4b =>Instruction {
                op: Operation::MOV(Register::C, Register::E),
                size: 1,
                cycles: 5
            },
            0x4c =>Instruction {
                op: Operation::MOV(Register::C, Register::H),
                size: 1,
                cycles: 5
            },
            0x4d =>Instruction {
                op: Operation::MOV(Register::C, Register::L),
                size: 1,
                cycles: 5
            },
            0x4e =>Instruction {
                op: Operation::MOV(Register::C, Register::M),
                size: 1,
                cycles: 7
            },
            0x4f =>Instruction {
                op: Operation::MOV(Register::C, Register::A),
                size: 1,
                cycles: 5
            },

            0x50 =>Instruction {
                op: Operation::MOV(Register::D, Register::B),
                size: 1,
                cycles: 5
            },
            0x51 =>Instruction {
                op: Operation::MOV(Register::D, Register::C),
                size: 1,
                cycles: 5
            },
            0x52 =>Instruction {
                op: Operation::MOV(Register::D, Register::D),
                size: 1,
                cycles: 5
            },
            0x53 =>Instruction {
                op: Operation::MOV(Register::D, Register::E),
                size: 1,
                cycles: 5
            },
            0x54 =>Instruction {
                op: Operation::MOV(Register::D, Register::H),
                size: 1,
                cycles: 5
            },
            0x55 =>Instruction {
                op: Operation::MOV(Register::D, Register::L),
                size: 1,
                cycles: 5
            },
            0x56 =>Instruction {
                op: Operation::MOV(Register::D, Register::M),
                size: 1,
                cycles: 7
            },
            0x57 =>Instruction {
                op: Operation::MOV(Register::D, Register::A),
                size: 1,
                cycles: 5
            },
            0x58 =>Instruction {
                op: Operation::MOV(Register::E, Register::B),
                size: 1,
                cycles: 5
            },
            0x59 =>Instruction {
                op: Operation::MOV(Register::E, Register::C),
                size: 1,
                cycles: 5
            },
            0x5a =>Instruction {
                op: Operation::MOV(Register::E, Register::D),
                size: 1,
                cycles: 5
            },
            0x5b =>Instruction {
                op: Operation::MOV(Register::E, Register::E),
                size: 1,
                cycles: 5
            },
            0x5c =>Instruction {
                op: Operation::MOV(Register::E, Register::H),
                size: 1,
                cycles: 5
            },
            0x5d =>Instruction {
                op: Operation::MOV(Register::E, Register::L),
                size: 1,
                cycles: 5
            },
            0x5e =>Instruction {
                op: Operation::MOV(Register::E, Register::M),
                size: 1,
                cycles: 7
            },
            0x5f =>Instruction {
                op: Operation::MOV(Register::E, Register::A),
                size: 1,
                cycles: 5
            },
            //0xb0 through 0xff - DS
            0xb0 => Instruction {
                op: Operation::ORA(Register::B),
                size: 1,
                cycles: 4
            },
            0xb1 => Instruction {
                op: Operation::ORA(Register::C),
                size: 1,
                cycles: 4
            },
            0xb2 => Instruction {
                op: Operation::ORA(Register::D),
                size: 1,
                cycles: 4
            },
            0xb3 => Instruction {
                op: Operation::ORA(Register::E),
                size: 1,
                cycles: 4
            }
            0xb4 => Instruction {
                op: Operation::ORA(Register::H),
                size: 1,
                cycles: 4
            },
            0xb5 => Instruction {
                op: Operation::ORA(Register::L),
                size: 1,
                cycles: 4
            },
            0xb6 => Instruction {
                op: Operation::ORA(Register::M),
                size: 1,
                cycles: 7
            },
            0xb7 => Instruction {
                op: Operation::ORA(Register::A),
                size: 1,
                cycles: 4
            },
            0xb8 => Instruction {
                op: Operation::CMP(Register::B),
                size: 1,
                cycles: 4
            },
            0xb9 => Instruction {
                op: Operation::CMP(Register::C),
                size: 1,
                cycles: 4
            },
            0xba => Instruction {
                op: Operation::CMP(Register::D),
                size: 1,
                cycles: 4
            },
            0xbb => Instruction {
                op: Operation::CMP(Register::E),
                size: 1,
                cycles: 4
            },
            0xbc => Instruction {
                op: Operation::CMP(Register::H),
                size: 1,
                cycles: 4
            },
            0xbd => Instruction {
                op: Operation::CMP(Register::L),
                size: 1,
                cycles: 4
            },
            0xbe => Instruction {
                op: Operation::CMP(Register::M),
                size: 1,
                cycles: 7
            },
            0xbf => Instruction {
                op: Operation::CMP(Register::A),
                size: 1,
                cycles: 4
            },
            //note: listed both cycle numbers (action taken/action not-taken) 
            //next to cycles where applicable.
            //https://pastraiser.com/cpu/i8080/i8080_opcodes.html
            0xc0 => Instruction {
                op: Operation::RNZ,
                size: 1,
                cycles: 11 //11/5
            },
            0xc1 => Instruction {
                op: Operation::POP(Register::B),
                size: 1,
                cycles: 10
            },
            0xc2 => Instruction {
                op: Operation::JNZ(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xc3 => Instruction {
                op: Operation::JMP(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xc4 => Instruction {
                op: Operation::CNZ(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xc5 => Instruction {
                op: Operation::PUSH(Register::B),
                size: 1,
                cycles: 11
            },
            0xc6 => Instruction {
                op: Operation::ADI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xc7 => Instruction {
                op: Operation::RST(Operand::D8(0x00)),
                size: 1,
                cycles: 11
            },
            0xc8 => Instruction {
                op: Operation::RZ,
                size: 1,
                cycles: 11 //11/5
            },
            0xc9 => Instruction {
                op: Operation::RET,
                size: 1,
                cycles: 10
            },
            0xca => Instruction {
                op: Operation::JZ(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xcb => Instruction {
                op: Operation::JMP(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xcc => Instruction {
                op: Operation::CZ(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xcd => Instruction {
                op: Operation::CALL(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17
            },
            0xce => Instruction {
                op: Operation::ACI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xcf => Instruction {
                op: Operation::RST(Operand::D8(0x08)),
                size: 1,
                cycles: 11
            },
            0xd0 => Instruction {
                op: Operation::RNC,
                size: 1,
                cycles: 11 //11/5
            },
            0xd1 => Instruction {
                op: Operation::POP(Register::D),
                size: 1,
                cycles: 10
            },
            0xd2 => Instruction {
                op: Operation::JNC(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xd3 => Instruction {
                op: Operation::OUT(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 10
            },
            0xd4 => Instruction {
                op: Operation::CNC(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xd5 => Instruction {
                op: Operation::PUSH(Register::D),
                size: 1,
                cycles: 11
            },
            0xd6 => Instruction {
                op: Operation::SUI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xd7 => Instruction {
                op: Operation::RST(Operand::D8(0x10)),
                size: 1,
                cycles: 11
            },
            0xd8 => Instruction {
                op: Operation::RC,
                size: 1,
                cycles: 11 //11/5
            },
            0xd9 => Instruction {
                op: Operation::RET,
                size: 1,
                cycles:10
            },
            0xda => Instruction {
                op: Operation::JC(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xdb => Instruction {
                op: Operation::IN(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 10
            },
            0xdc => Instruction {
                op: Operation::CC(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xdd => Instruction {
                op: Operation::CALL(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17
            },
            0xde => Instruction {
                op: Operation::SBI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xdf => Instruction {
                op: Operation::RST(Operand::D8(0x18)),
                size: 1,
                cycles: 11
            },
            0xe0 => Instruction {
                op: Operation::RPO,
                size: 1,
                cycles: 11 //11/5
            },
            0xe1 => Instruction {
                op: Operation::POP(Register::H),
                size: 1,
                cycles: 10
            },
            0xe2 => Instruction {
                op: Operation::JPO(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xe3 => Instruction {
                op: Operation::XTHL,
                size: 1,
                cycles: 18
            },
            0xe4 => Instruction {
                op: Operation::CPO(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xe5 => Instruction {
                op: Operation::PUSH(Register::H),
                size: 1,
                cycles: 11
            },
            0xe6 => Instruction {
                op: Operation::ANI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xe7 => Instruction {
                op: Operation::RST(Operand::D8(0x20)),
                size: 1,
                cycles: 11
            },          
            0xe8 => Instruction {
                op: Operation::RPE,
                size: 1,
                cycles: 11 //11/5
            },
            0xe9 => Instruction {
                op: Operation::PCHL,
                size: 1,
                cycles: 5
            },
            0xea => Instruction {
                op: Operation::JPE(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xeb => Instruction {
                op: Operation::XCHG,
                size: 1,
                cycles: 5
            },
            0xec => Instruction {
                op: Operation::CPE(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xed => Instruction {
                op: Operation::CALL(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17
            },
            0xee => Instruction {
                op: Operation::XRI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xef => Instruction {
                op: Operation::RST(Operand::D8(0x28)),
                size: 1,
                cycles: 11
            },
            0xf0 => Instruction {
                op: Operation::RP,
                size: 1,
                cycles: 11 //11/5
            },
            0xf1 => Instruction {
                op: Operation::POP(Register::PSW),
                size: 1,
                cycles: 10
            },
            0xf2 => Instruction {
                op: Operation::JP(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xf3 => Instruction {
                op: Operation::DI,
                size: 1,
                cycles: 4
            },
            0xf4 => Instruction {
                op: Operation::CP(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xf5 => Instruction {
                op: Operation::PUSH(Register::PSW),
                size: 1,
                cycles: 11
            },
            0xf6 => Instruction {
                op: Operation::ORI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xf7 => Instruction {
                op: Operation::RST(Operand::D8(0x30)),
                size: 1,
                cycles: 11
            },
            0xf8 => Instruction {
                op: Operation::RM,
                size: 1,
                cycles: 11 //11/5
            },
            0xf9 => Instruction {
                op: Operation::SPHL,
                size: 1,
                cycles: 5
            },
            0xfa => Instruction {
                op: Operation::JM(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 10
            },
            0xfb => Instruction {
                op: Operation::EI,
                size: 1,
                cycles: 4
            },
            0xfc => Instruction {
                op: Operation::CM(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17 //17/11
            },
            0xfd => Instruction {
                op: Operation::CALL(Operand::A16(Instruction::read_imm16(bytes))),
                size: 3,
                cycles: 17
            },
            0xfe => Instruction {
                op: Operation::CPI(Operand::D8(Instruction::read_imm8(bytes))),
                size: 2,
                cycles: 7
            },
            0xff => Instruction {
                op: Operation::RST(Operand::D8(0x38)),
                size: 1,
                cycles: 11
            },
            _ => unimplemented!("instruction {:#x?} has not yet been implemented", opcode)
        };

        Ok(instruction)
    }

    fn read_imm8(bytes: &[u8]) -> u8 {
        u8::from_le_bytes([bytes[1]])
    }

    fn read_imm16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[1], bytes[2]])
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    fn cycles(&self) -> u8 {
        self.cycles
    }
}

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Instruction {
        Instruction::decode_op(bytes).unwrap()
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::NOP => write!(f, "NOP"),
            Operation::PUSH(val) => write!(f, "PUSH\t{:#x?}", val),
            Operation::JMP(val) => write!(f, "JMP\t{:#x?}", val),
            Operation::MVI(lhs, rhs) => write!(f, "MVI\t{:#x?}, {:#x?}", lhs, rhs),
            Operation::STA(val) => write!(f, "STA\t{:#x?}", val),
            Operation::LXI(lhs, rhs) => write!(f, "LXI\t{:#x?}, {:#x?}", lhs, rhs), 
            Operation::STAX(val) => write!(f, "STAX\t{:#x?}", val),
            Operation::INX(val) => write!(f, "INX\t{:#x?}", val),
            Operation::INR(val) => write!(f, "INR\t{:#x?}", val),
            Operation::DCR(val) => write!(f, "DCR\t{:#x?}", val),
            Operation::RLC => write!(f, "RLC"),
            Operation::DAD(val) => write!(f, "DAD\t{:#x?}", val),
            Operation::LDAX(val) => write!(f, "LDAX\t{:#x?}", val), 
            Operation::DCX(val) => write!(f, "DCX\t{:#x?}", val), 
            Operation::RRC => write!(f, "RLC"),
            Operation::RAL => write!(f, "RLC"), 
            Operation::RAR => write!(f, "RLC"),
            Operation::SHLD(val) => write!(f, "SHLD\t{:#x?}", val),
            Operation::DAA => write!(f, "RLC"), 
            Operation::LHLD(val) => write!(f, "LHLD\t{:#x?}", val),
            Operation::CMA => write!(f, "RLC"), 
            Operation::STC => write!(f, "RLC"),
            Operation::LDA(val) => write!(f, "LDA\t{:#x?}", val),
            Operation::CMC => write!(f, "RLC"),
            Operation::MOV(lhs, rhs) => write!(f, "MOV\t{:#x?}, {:#x?}", lhs, rhs),
            _ => unimplemented!("Operation has not yet been implemented for fmt::Debug")
        }
    }
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::D8(val) => write!(f, "{:#x?}", val),
            Operand::A16(val) | Operand::D16(val) => write!(f, "{:#x?}", val),
            _ => write!(f, "Debug printing is not implemented for {:#x?}", self)
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self.op);
    }
}

