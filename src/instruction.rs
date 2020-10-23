use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Operand {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    SP,
    PSW,
}

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
pub enum Instruction {
    NOP,
    JMP(u16),
    PUSH(Operand),
    MVI(Operand, u8),
    STA(u16),
    LXI(Operand, u16),
    STAX(Operand),
    INX(Operand),
    INR(Operand),
    DCR(Operand),
    RLC,
    DAD(Operand),
    LDAX(Operand),
    DCX(Operand),
    RRC,
    RAL,
    RAR,
    SHLD(u16),
    DAA,
    LHLD(u16),
    CMA,
    STC,
    LDA(u16),
    CMC,
    MOV(Operand, Operand),
    HLT,
    ADD(Operand),
    ANA(Operand),
    ADC(Operand),
    SUB(Operand),
    SBB(Operand),
    XRA(Operand),
    ACI(u8),
    ADI(u8),
    ANI(u8),
    CALL(u16),
    CC(u16),
    CM(u16),
    CMP(Operand),
    CNC(u16),
    CP(u16),
    CPE(u16),
    CPI(u8),
    CPO(u16),
    CNZ(u16),
    CZ(u16),
    DI,
    EI,
    IN(u8),
    JC(u16),
    JM(u16),
    JNC(u16),
    JNZ(u16),
    JP(u16),
    JPE(u16),
    JPO(u16),
    JZ(u16),
    ORA(Operand),
    ORI(u8),
    OUT(u8),
    PCHL,
    POP(Operand),
    RC,
    RET,
    RM,
    RNC,
    RNZ,
    RP,
    RPE,
    RPO,
    RST(u8),
    RZ,
    SBI(u8),
    SPHL,
    SUI(u8),
    XCHG,
    XRI(u8),
    XTHL,
}

impl Instruction {
    fn decode_op(bytes: &[u8]) -> Result<Instruction, ()> {
        let opcode = bytes[0];

        let instruction = match opcode {
            0x00 | 0x10 | 0x20 | 0x30 | 0x08 | 0x18 | 0x28 | 0x38 => Instruction::NOP,
            0x01 => Instruction::LXI(Operand::B, Instruction::read_imm16(bytes)),
            0x02 => Instruction::STAX(Operand::B),
            0x03 => Instruction::INX(Operand::B),
            0x04 => Instruction::INR(Operand::B),
            0x05 => Instruction::DCR(Operand::B),
            0x06 => Instruction::MVI(Operand::B, Instruction::read_imm8(bytes)),
            0x07 => Instruction::RLC,
            0x09 => Instruction::DAD(Operand::B),
            0x0a => Instruction::LDAX(Operand::B),
            0x0b => Instruction::DCX(Operand::B),
            0x0c => Instruction::INR(Operand::C),
            0x0d => Instruction::DCR(Operand::C),
            0x0e => Instruction::MVI(Operand::C, Instruction::read_imm8(bytes)),
            0x0f => Instruction::RRC,
            0x11 => Instruction::LXI(Operand::D, Instruction::read_imm16(bytes)),
            0x12 => Instruction::STAX(Operand::D),
            0x13 => Instruction::INX(Operand::D),
            0x14 => Instruction::INR(Operand::D),
            0x15 => Instruction::DCR(Operand::D),
            0x16 => Instruction::MVI(Operand::D, Instruction::read_imm8(bytes)),
            0x17 => Instruction::RAL,
            0x19 => Instruction::DAD(Operand::D),
            0x1a => Instruction::LDAX(Operand::D),
            0x1b => Instruction::DCX(Operand::D),
            0x1c => Instruction::INR(Operand::E),
            0x1d => Instruction::DCR(Operand::E),
            0x1e => Instruction::MVI(Operand::E, Instruction::read_imm8(bytes)),
            0x1f => Instruction::RAR,
            0x21 => Instruction::LXI(Operand::H, Instruction::read_imm16(bytes)),
            0x22 => Instruction::SHLD(Instruction::read_imm16(bytes)),
            0x23 => Instruction::INX(Operand::H),
            0x24 => Instruction::INR(Operand::H),
            0x25 => Instruction::DCR(Operand::H),
            0x26 => Instruction::MVI(Operand::H, Instruction::read_imm8(bytes)),
            0x27 => Instruction::DAA,
            0x29 => Instruction::DAD(Operand::H),
            0x2a => Instruction::LHLD(Instruction::read_imm16(bytes)),
            0x2b => Instruction::DCX(Operand::H),
            0x2c => Instruction::INR(Operand::L),
            0x2d => Instruction::DCR(Operand::L),
            0x2e => Instruction::MVI(Operand::L, Instruction::read_imm8(bytes)),
            0x2f => Instruction::CMA,
            0x31 => Instruction::LXI(Operand::SP, Instruction::read_imm16(bytes)),
            0x32 => Instruction::STA(Instruction::read_imm16(bytes)),
            0x33 => Instruction::INX(Operand::SP),
            0x34 => Instruction::INR(Operand::M),
            0x35 => Instruction::DCR(Operand::M),
            0x36 => Instruction::MVI(Operand::M, Instruction::read_imm8(bytes)),
            0x37 => Instruction::STC,
            0x39 => Instruction::DAD(Operand::SP),
            0x3a => Instruction::LDA(Instruction::read_imm16(bytes)),
            0x3b => Instruction::DCX(Operand::SP),
            0x3c => Instruction::INR(Operand::A),
            0x3d => Instruction::DCR(Operand::A),
            0x3e => Instruction::MVI(Operand::A, Instruction::read_imm8(bytes)),
            0x3f => Instruction::CMC,
            0x40 => Instruction::MOV(Operand::B, Operand::B),
            0x41 => Instruction::MOV(Operand::B, Operand::C),
            0x42 => Instruction::MOV(Operand::B, Operand::D),
            0x43 => Instruction::MOV(Operand::B, Operand::E),
            0x44 => Instruction::MOV(Operand::B, Operand::H),
            0x45 => Instruction::MOV(Operand::B, Operand::L),
            0x46 => Instruction::MOV(Operand::B, Operand::M),
            0x47 => Instruction::MOV(Operand::B, Operand::A),
            0x48 => Instruction::MOV(Operand::C, Operand::B),
            0x49 => Instruction::MOV(Operand::C, Operand::C),
            0x4a => Instruction::MOV(Operand::C, Operand::D),
            0x4b => Instruction::MOV(Operand::C, Operand::E),
            0x4c => Instruction::MOV(Operand::C, Operand::H),
            0x4d => Instruction::MOV(Operand::C, Operand::L),
            0x4e => Instruction::MOV(Operand::C, Operand::M),
            0x4f => Instruction::MOV(Operand::C, Operand::A),
            0x50 => Instruction::MOV(Operand::D, Operand::B),
            0x51 => Instruction::MOV(Operand::D, Operand::C),
            0x52 => Instruction::MOV(Operand::D, Operand::D),
            0x53 => Instruction::MOV(Operand::D, Operand::E),
            0x54 => Instruction::MOV(Operand::D, Operand::H),
            0x55 => Instruction::MOV(Operand::D, Operand::L),
            0x56 => Instruction::MOV(Operand::D, Operand::M),
            0x57 => Instruction::MOV(Operand::D, Operand::A),
            0x58 => Instruction::MOV(Operand::E, Operand::B),
            0x59 => Instruction::MOV(Operand::E, Operand::C),
            0x5a => Instruction::MOV(Operand::E, Operand::D),
            0x5b => Instruction::MOV(Operand::E, Operand::E),
            0x5c => Instruction::MOV(Operand::E, Operand::H),
            0x5d => Instruction::MOV(Operand::E, Operand::L),
            0x5e => Instruction::MOV(Operand::E, Operand::M),
            0x5f => Instruction::MOV(Operand::E, Operand::A),
            0x60 => Instruction::MOV(Operand::H, Operand::B),
            0x61 => Instruction::MOV(Operand::H, Operand::C),
            0x62 => Instruction::MOV(Operand::H, Operand::D),
            0x63 => Instruction::MOV(Operand::H, Operand::E),
            0x64 => Instruction::MOV(Operand::H, Operand::H),
            0x65 => Instruction::MOV(Operand::H, Operand::L),
            0x66 => Instruction::MOV(Operand::H, Operand::M),
            0x67 => Instruction::MOV(Operand::H, Operand::A),
            0x68 => Instruction::MOV(Operand::L, Operand::B),
            0x69 => Instruction::MOV(Operand::L, Operand::C),
            0x6a => Instruction::MOV(Operand::L, Operand::D),
            0x6b => Instruction::MOV(Operand::L, Operand::E),
            0x6c => Instruction::MOV(Operand::L, Operand::H),
            0x6d => Instruction::MOV(Operand::L, Operand::L),
            0x6e => Instruction::MOV(Operand::L, Operand::M),
            0x6f => Instruction::MOV(Operand::L, Operand::A),
            0x70 => Instruction::MOV(Operand::M, Operand::B),
            0x71 => Instruction::MOV(Operand::M, Operand::C),
            0x72 => Instruction::MOV(Operand::M, Operand::D),
            0x73 => Instruction::MOV(Operand::M, Operand::E),
            0x74 => Instruction::MOV(Operand::M, Operand::H),
            0x75 => Instruction::MOV(Operand::M, Operand::L),
            0x76 => Instruction::HLT,
            0x77 => Instruction::MOV(Operand::M, Operand::A),
            0x78 => Instruction::MOV(Operand::A, Operand::B),
            0x79 => Instruction::MOV(Operand::A, Operand::C),
            0x7a => Instruction::MOV(Operand::A, Operand::D),
            0x7b => Instruction::MOV(Operand::A, Operand::E),
            0x7c => Instruction::MOV(Operand::A, Operand::H),
            0x7d => Instruction::MOV(Operand::A, Operand::L),
            0x7e => Instruction::MOV(Operand::A, Operand::M),
            0x7f => Instruction::MOV(Operand::A, Operand::A),
            0x80 => Instruction::ADD(Operand::B),
            0x81 => Instruction::ADD(Operand::C),
            0x82 => Instruction::ADD(Operand::D),
            0x83 => Instruction::ADD(Operand::E),
            0x84 => Instruction::ADD(Operand::H),
            0x85 => Instruction::ADD(Operand::L),
            0x86 => Instruction::ADD(Operand::M),
            0x87 => Instruction::ADD(Operand::A),
            0x88 => Instruction::ADC(Operand::B),
            0x89 => Instruction::ADC(Operand::C),
            0x8a => Instruction::ADC(Operand::D),
            0x8b => Instruction::ADC(Operand::E),
            0x8c => Instruction::ADC(Operand::H),
            0x8d => Instruction::ADC(Operand::L),
            0x8e => Instruction::ADC(Operand::M),
            0x8f => Instruction::ADC(Operand::A),
            0x90 => Instruction::SUB(Operand::B),
            0x91 => Instruction::SUB(Operand::C),
            0x92 => Instruction::SUB(Operand::D),
            0x93 => Instruction::SUB(Operand::E),
            0x94 => Instruction::SUB(Operand::H),
            0x95 => Instruction::SUB(Operand::L),
            0x96 => Instruction::SUB(Operand::M),
            0x97 => Instruction::SUB(Operand::A),
            0x98 => Instruction::SBB(Operand::B),
            0x99 => Instruction::SBB(Operand::C),
            0x9a => Instruction::SBB(Operand::D),
            0x9b => Instruction::SBB(Operand::E),
            0x9c => Instruction::SBB(Operand::H),
            0x9d => Instruction::SBB(Operand::L),
            0x9e => Instruction::SBB(Operand::M),
            0x9f => Instruction::SBB(Operand::A),
            0xa0 => Instruction::ANA(Operand::B),
            0xa1 => Instruction::ANA(Operand::C),
            0xa2 => Instruction::ANA(Operand::D),
            0xa3 => Instruction::ANA(Operand::E),
            0xa4 => Instruction::ANA(Operand::H),
            0xa5 => Instruction::ANA(Operand::L),
            0xa6 => Instruction::ANA(Operand::M),
            0xa7 => Instruction::ANA(Operand::A),
            0xa8 => Instruction::XRA(Operand::B),
            0xa9 => Instruction::XRA(Operand::C),
            0xaa => Instruction::XRA(Operand::D),
            0xab => Instruction::XRA(Operand::E),
            0xac => Instruction::XRA(Operand::H),
            0xad => Instruction::XRA(Operand::L),
            0xae => Instruction::XRA(Operand::M),
            0xaf => Instruction::XRA(Operand::A),
            0xb0 => Instruction::ORA(Operand::B),
            0xb1 => Instruction::ORA(Operand::C),
            0xb2 => Instruction::ORA(Operand::D),
            0xb3 => Instruction::ORA(Operand::E),
            0xb4 => Instruction::ORA(Operand::H),
            0xb5 => Instruction::ORA(Operand::L),
            0xb6 => Instruction::ORA(Operand::M),
            0xb7 => Instruction::ORA(Operand::A),
            0xb8 => Instruction::CMP(Operand::B),
            0xb9 => Instruction::CMP(Operand::C),
            0xba => Instruction::CMP(Operand::D),
            0xbb => Instruction::CMP(Operand::E),
            0xbc => Instruction::CMP(Operand::H),
            0xbd => Instruction::CMP(Operand::L),
            0xbe => Instruction::CMP(Operand::M),
            0xbf => Instruction::CMP(Operand::A),
            //note: listed both cycle numbers (action taken/action not-taken)
            //next to cycles where applicable.
            //https://pastraiser.com/cpu/i8080/i8080_opcodes.html
            0xc0 => Instruction::RNZ,
            0xc1 => Instruction::POP(Operand::B),
            0xc2 => Instruction::JNZ(Instruction::read_imm16(bytes)),
            0xc3 | 0xcb => Instruction::JMP(Instruction::read_imm16(bytes)),
            0xc4 => Instruction::CNZ(Instruction::read_imm16(bytes)),
            0xc5 => Instruction::PUSH(Operand::B),
            0xc6 => Instruction::ADI(Instruction::read_imm8(bytes)),
            0xc7 => Instruction::RST(0),
            0xc8 => Instruction::RZ,
            0xc9 | 0xd9 => Instruction::RET,
            0xca => Instruction::JZ(Instruction::read_imm16(bytes)),
            0xcc => Instruction::CZ(Instruction::read_imm16(bytes)),
            0xcd | 0xdd | 0xed | 0xfd => Instruction::CALL(Instruction::read_imm16(bytes)),
            0xce => Instruction::ACI(Instruction::read_imm8(bytes)),
            0xcf => Instruction::RST(1),
            0xd0 => Instruction::RNC,
            0xd1 => Instruction::POP(Operand::D),
            0xd2 => Instruction::JNC(Instruction::read_imm16(bytes)),
            0xd3 => Instruction::OUT(Instruction::read_imm8(bytes)),
            0xd4 => Instruction::CNC(Instruction::read_imm16(bytes)),
            0xd5 => Instruction::PUSH(Operand::D),
            0xd6 => Instruction::SUI(Instruction::read_imm8(bytes)),
            0xd7 => Instruction::RST(2),
            0xd8 => Instruction::RC,
            0xda => Instruction::JC(Instruction::read_imm16(bytes)),
            0xdb => Instruction::IN(Instruction::read_imm8(bytes)),
            0xdc => Instruction::CC(Instruction::read_imm16(bytes)),
            0xde => Instruction::SBI(Instruction::read_imm8(bytes)),
            0xdf => Instruction::RST(3),
            0xe0 => Instruction::RPO,
            0xe1 => Instruction::POP(Operand::H),
            0xe2 => Instruction::JPO(Instruction::read_imm16(bytes)),
            0xe3 => Instruction::XTHL,
            0xe4 => Instruction::CPO(Instruction::read_imm16(bytes)),
            0xe5 => Instruction::PUSH(Operand::H),
            0xe6 => Instruction::ANI(Instruction::read_imm8(bytes)),
            0xe7 => Instruction::RST(4),
            0xe8 => Instruction::RPE,
            0xe9 => Instruction::PCHL,
            0xea => Instruction::JPE(Instruction::read_imm16(bytes)),
            0xeb => Instruction::XCHG,
            0xec => Instruction::CPE(Instruction::read_imm16(bytes)),
            0xee => Instruction::XRI(Instruction::read_imm8(bytes)),
            0xef => Instruction::RST(5),
            0xf0 => Instruction::RP,
            0xf1 => Instruction::POP(Operand::PSW),
            0xf2 => Instruction::JP(Instruction::read_imm16(bytes)),
            0xf3 => Instruction::DI,
            0xf4 => Instruction::CP(Instruction::read_imm16(bytes)),
            0xf5 => Instruction::PUSH(Operand::PSW),
            0xf6 => Instruction::ORI(Instruction::read_imm8(bytes)),
            0xf7 => Instruction::RST(6),
            0xf8 => Instruction::RM,
            0xf9 => Instruction::SPHL,
            0xfa => Instruction::JM(Instruction::read_imm16(bytes)),
            0xfb => Instruction::EI,
            0xfc => Instruction::CM(Instruction::read_imm16(bytes)),
            0xfe => Instruction::CPI(Instruction::read_imm8(bytes)),
            0xff => Instruction::RST(7),
        };

        Ok(instruction)
    }

    pub fn size(&self) -> u16 {
        match *self {
            Instruction::NOP => 1,
            Instruction::JMP(_) => 3,
            Instruction::PUSH(_) => 1,
            Instruction::MVI(_, _) => 2,
            Instruction::STA(_) => 3,
            Instruction::LXI(_, _) => 3,
            Instruction::STAX(_) => 1,
            Instruction::INX(_) => 1,
            Instruction::INR(_) => 1,
            Instruction::DCR(_) => 1,
            Instruction::RLC => 1,
            Instruction::DAD(_) => 1,
            Instruction::LDAX(_) => 1,
            Instruction::DCX(_) => 1,
            Instruction::RRC => 1,
            Instruction::RAL => 1,
            Instruction::RAR => 1,
            Instruction::SHLD(_) => 3,
            Instruction::DAA => 1,
            Instruction::LHLD(_) => 3,
            Instruction::CMA => 1,
            Instruction::STC => 1,
            Instruction::LDA(_) => 3,
            Instruction::CMC => 1,
            Instruction::MOV(_, _) => 1,
            Instruction::HLT => 1,
            Instruction::ADD(_) => 1,
            Instruction::ANA(_) => 1,
            Instruction::ADC(_) => 1,
            Instruction::SUB(_) => 1,
            Instruction::SBB(_) => 1,
            Instruction::XRA(_) => 1,
            Instruction::ORA(_) => 1,
            Instruction::CMP(_) => 1,
            Instruction::RNZ => 1,
            Instruction::POP(_) => 1,
            Instruction::JNZ(_) => 3,
            Instruction::CNZ(_) => 3,
            Instruction::ADI(_) => 2,
            Instruction::RST(_) => 1,
            Instruction::RZ => 1,
            Instruction::RET => 1,
            Instruction::JZ(_) => 3,
            Instruction::CZ(_) => 3,
            Instruction::CALL(_) => 3,
            Instruction::ACI(_) => 2,
            Instruction::RNC => 1,
            Instruction::JNC(_) => 3,
            Instruction::OUT(_) => 2,
            Instruction::CNC(_) => 3,
            Instruction::SUI(_) => 2,
            Instruction::RC => 1,
            Instruction::JC(_) => 3,
            Instruction::IN(_) => 2,
            Instruction::CC(_) => 3,
            Instruction::SBI(_) => 3,
            Instruction::RPO => 1,
            Instruction::JPO(_) => 3,
            Instruction::XTHL => 1,
            Instruction::CPO(_) => 3,
            Instruction::ANI(_) => 2,
            Instruction::RPE => 1,
            Instruction::PCHL => 1,
            Instruction::JPE(_) => 3,
            Instruction::XCHG => 1,
            Instruction::CPE(_) => 3,
            Instruction::XRI(_) => 2,
            Instruction::RP => 1,
            Instruction::JP(_) => 3,
            Instruction::DI => 1,
            Instruction::CP(_) => 3,
            Instruction::ORI(_) => 2,
            Instruction::RM => 1,
            Instruction::SPHL => 1,
            Instruction::JM(_) => 3,
            Instruction::EI => 1,
            Instruction::CM(_) => 3,
            Instruction::CPI(_) => 2,
        }
    }

    pub fn cycles(&self) -> u8 {
        match *self {
            Instruction::NOP => 4,
            Instruction::JMP(_) => 10,
            Instruction::PUSH(_) => 11,
            Instruction::MVI(target, _) => match target {
                Operand::M => 10,
                _ => 7,
            },
            Instruction::STA(_) => 13,
            Instruction::LXI(_, _) => 10,
            Instruction::STAX(_) => 7,
            Instruction::INX(_) => 5,
            Instruction::INR(target) => match target {
                Operand::M => 10,
                _ => 5,
            },
            Instruction::DCR(target) => match target {
                Operand::M => 10,
                _ => 5,
            },
            Instruction::RLC => 4,
            Instruction::DAD(_) => 10,
            Instruction::LDAX(_) => 7,
            Instruction::DCX(_) => 5,
            Instruction::RRC => 4,
            Instruction::RAL => 4,
            Instruction::RAR => 4,
            Instruction::SHLD(_) => 16,
            Instruction::DAA => 4,
            Instruction::LHLD(_) => 16,
            Instruction::CMA => 4,
            Instruction::STC => 4,
            Instruction::LDA(_) => 13,
            Instruction::CMC => 4,
            Instruction::MOV(target, source) => match (target, source) {
                (Operand::M, _) => 7,
                (_, Operand::M) => 7,
                _ => 5,
            },
            Instruction::HLT => 7,
            Instruction::ADD(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::ANA(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::ADC(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::SUB(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::SBB(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::XRA(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::ORA(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::CMP(target) => match target {
                Operand::M => 7,
                _ => 4,
            },
            Instruction::RNZ => 5,
            Instruction::POP(_) => 10,
            Instruction::JNZ(_) => 10,
            Instruction::CNZ(_) => 11,
            Instruction::ADI(_) => 7,
            Instruction::RST(_) => 11,
            Instruction::RZ => 5,
            Instruction::RET => 10,
            Instruction::JZ(_) => 10,
            Instruction::CZ(_) => 11,
            Instruction::CALL(_) => 17,
            Instruction::ACI(_) => 7,
            Instruction::RNC => 5,
            Instruction::JNC(_) => 10,
            Instruction::OUT(_) => 10,
            Instruction::CNC(_) => 11,
            Instruction::SUI(_) => 7,
            Instruction::RC => 5,
            Instruction::JC(_) => 10,
            Instruction::IN(_) => 10,
            Instruction::CC(_) => 11,
            Instruction::SBI(_) => 7,
            Instruction::RPO => 5,
            Instruction::JPO(_) => 10,
            Instruction::XTHL => 18,
            Instruction::CPO(_) => 11,
            Instruction::ANI(_) => 7,
            Instruction::RPE => 5,
            Instruction::PCHL => 5,
            Instruction::JPE(_) => 10,
            Instruction::XCHG => 5,
            Instruction::CPE(_) => 11,
            Instruction::XRI(_) => 7,
            Instruction::RP => 5,
            Instruction::JP(_) => 10,
            Instruction::DI => 4,
            Instruction::CP(_) => 11,
            Instruction::ORI(_) => 7,
            Instruction::RM => 5,
            Instruction::SPHL => 5,
            Instruction::JM(_) => 10,
            Instruction::EI => 4,
            Instruction::CM(_) => 11,
            Instruction::CPI(_) => 7,
        }
    }

    fn read_imm8(bytes: &[u8]) -> u8 {
        u8::from_le_bytes([bytes[1]])
    }

    fn read_imm16(bytes: &[u8]) -> u16 {
        u16::from_le_bytes([bytes[1], bytes[2]])
    }
}

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Instruction {
        Instruction::decode_op(bytes).unwrap()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::PUSH(val) => write!(f, "PUSH\t{:#x?}", val),
            Instruction::JMP(val) => write!(f, "JMP\t{:#x?}", val),
            Instruction::MVI(lhs, rhs) => write!(f, "MVI\t{:#x?}, {:#x?}", lhs, rhs),
            Instruction::STA(val) => write!(f, "STA\t{:#x?}", val),
            Instruction::LXI(lhs, rhs) => write!(f, "LXI\t{:#x?}, {:#x?}", lhs, rhs),
            Instruction::STAX(val) => write!(f, "STAX\t{:#x?}", val),
            Instruction::INX(val) => write!(f, "INX\t{:#x?}", val),
            Instruction::INR(val) => write!(f, "INR\t{:#x?}", val),
            Instruction::DCR(val) => write!(f, "DCR\t{:#x?}", val),
            Instruction::RLC => write!(f, "RLC"),
            Instruction::DAD(val) => write!(f, "DAD\t{:#x?}", val),
            Instruction::LDAX(val) => write!(f, "LDAX\t{:#x?}", val),
            Instruction::DCX(val) => write!(f, "DCX\t{:#x?}", val),
            Instruction::RRC => write!(f, "RLC"),
            Instruction::RAL => write!(f, "RLC"),
            Instruction::RAR => write!(f, "RLC"),
            Instruction::SHLD(val) => write!(f, "SHLD\t{:#x?}", val),
            Instruction::DAA => write!(f, "RLC"),
            Instruction::LHLD(val) => write!(f, "LHLD\t{:#x?}", val),
            Instruction::CMA => write!(f, "RLC"),
            Instruction::STC => write!(f, "RLC"),
            Instruction::LDA(val) => write!(f, "LDA\t{:#x?}", val),
            Instruction::CMC => write!(f, "RLC"),
            Instruction::MOV(lhs, rhs) => write!(f, "MOV\t{:#x?}, {:#x?}", lhs, rhs),
            Instruction::HLT => write!(f, "HLT"),
            Instruction::ADD(val) => write!(f, "ADD\t{:#x?}", val),
            Instruction::ANA(val) => write!(f, "ANA\t{:#x?}", val),
            Instruction::ADC(val) => write!(f, "ADC\t{:#x?}", val),
            Instruction::SUB(val) => write!(f, "SUB\t{:#x?}", val),
            Instruction::SBB(val) => write!(f, "SBB\t{:#x?}", val),
            Instruction::XRA(val) => write!(f, "XRA\t{:#x?}", val),
            Instruction::ACI(val) => write!(f, "ACI\t{:#x?}", val),
            Instruction::ADI(val) => write!(f, "ADI\t{:#x?}", val),
            Instruction::ANI(val) => write!(f, "ANI\t{:#x?}", val),
            Instruction::CALL(val) => write!(f, "CALL\t{:#x?}", val),
            Instruction::CC(val) => write!(f, "CC\t{:#x?}", val),
            Instruction::CM(val) => write!(f, "CM\t{:#x?}", val),
            Instruction::CMP(val) => write!(f, "CMP\t{:#x?}", val),
            Instruction::CNC(val) => write!(f, "CNC\t{:#x?}", val),
            Instruction::CP(val) => write!(f, "CP\t{:#x?}", val),
            Instruction::CPE(val) => write!(f, "CPE\t{:#x?}", val),
            Instruction::CPI(val) => write!(f, "CPI\t{:#x?}", val),
            Instruction::CPO(val) => write!(f, "CPO\t{:#x?}", val),
            Instruction::CNZ(val) => write!(f, "CNZ\t{:#x?}", val),
            Instruction::CZ(val) => write!(f, "CZ\t{:#x?}", val),
            Instruction::DI => write!(f, "DI"),
            Instruction::EI => write!(f, "EI"),
            Instruction::IN(val) => write!(f, "IN\t{:#x?}", val),
            Instruction::JC(val) => write!(f, "JC\t{:#x?}", val),
            Instruction::JM(val) => write!(f, "JM\t{:#x?}", val),
            Instruction::JNC(val) => write!(f, "JNC\t{:#x?}", val),
            Instruction::JNZ(val) => write!(f, "JNZ\t{:#x?}", val),
            Instruction::JP(val) => write!(f, "JP\t{:#x?}", val),
            Instruction::JPE(val) => write!(f, "JPE\t{:#x?}", val),
            Instruction::JPO(val) => write!(f, "JPO\t{:#x?}", val),
            Instruction::JZ(val) => write!(f, "JZ\t{:#x?}", val),
            Instruction::ORA(val) => write!(f, "ORA\t{:#x?}", val),
            Instruction::ORI(val) => write!(f, "ORI\t{:#x?}", val),
            Instruction::OUT(val) => write!(f, "OUT\t{:#x?}", val),
            Instruction::PCHL => write!(f, "PCHL"),
            Instruction::POP(val) => write!(f, "POP\t{:#x?}", val),
            Instruction::RC => write!(f, "RC"),
            Instruction::RET => write!(f, "RET"),
            Instruction::RM => write!(f, "RM"),
            Instruction::RNC => write!(f, "RNC"),
            Instruction::RNZ => write!(f, "RNZ"),
            Instruction::RP => write!(f, "RP"),
            Instruction::RPE => write!(f, "RPE"),
            Instruction::RPO => write!(f, "RPO"),
            Instruction::RST(val) => write!(f, "RST\t{:#x?}", val),
            Instruction::RZ => write!(f, "RZ"),
            Instruction::SBI(val) => write!(f, "SBI\t{:#x?}", val),
            Instruction::SPHL => write!(f, "SPHL"),
            Instruction::SUI(val) => write!(f, "SUI\t{:#x?}", val),
            Instruction::XCHG => write!(f, "XCHG"),
            Instruction::XRI(val) => write!(f, "XRI\t{:#x?}", val),
            Instruction::XTHL => write!(f, "XTHL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycles_m_target() {
        assert_eq!(Instruction::MOV(Operand::M, Operand::B).cycles(), 7);
    }

    #[test]
    fn test_cycles_m_source() {
        assert_eq!(Instruction::MOV(Operand::B, Operand::M).cycles(), 7);
    }

    #[test]
    fn test_cycles_m_neither() {
        assert_eq!(Instruction::MOV(Operand::B, Operand::C).cycles(), 5);
    }

    #[test]
    fn test_size() {
        assert_eq!(Instruction::PUSH(Operand::C).size(), 1);
    }
}
