use std::fmt;

// source: https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf
#[derive(Debug)]
enum Operation {
    STC,
    CMC,
    INR,
    DCR,
    CMA,
    DAA,
    MOV,
    STAX,
    LDAX,
    NOP,
    ADD,
    ADC,
    SUB,
    SBB,
    ANA,
    XRA,
    ORA,
    CMP,
    RLC,
    RRC,
    RAL,
    RAR,
    PUSH,
    POP,
    DAD,
    INX,
    DCX,
    XCHG,
    XTHL,
    SPHL,
    LXI,
    MVI,
    ADI,
    ACI,
    SUI,
    SBI,
    ANI,
    XRI,
    ORI,
    CPI,
    STA,
    LDA,
    SHLD,
    LHLD,
    PCHL,
    JMP,
    JC,
    JNC,
    JZ,
    JNZ,
    JM,
    JP,
    JPE,
    JPO,
    CALL,
    CC,
    CNC,
    CZ,
    CNZ,
    CM,
    CP,
    CPE,
    CPO,
    RET,
    RN,
    RNC,
    RZ,
    RNZ,
    RM,
    RP,
    RPE,
    RPO,
    EI,
    DI,
    IN,
    OUT
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Debug)]
enum Operand {
    Reg(Register)
}

pub struct Instruction {
    op: Operation,
    target: Option<Operand>,
    source: Option<Operand>,
    size: u8,
    cycles: u8
}

impl Instruction {
    fn decode_op(bytes: &[u8]) -> Result<Instruction, ()> {
        let opcode = bytes[0];

        let instruction = match opcode {
            0x00 => Instruction {
                op: Operation::NOP,
                target: None,
                source: None,
                size: 1,
                cycles: 4
            },
            0x01 => Instruction {
                op: Operation::LXI,
                target: Some(Operand::Reg(Register::B)),
                source: None,  // TODO
                size: 3,
                cycles: 10
            },
            0x3E => Instruction {
                op: Operation::MVI,
                target: Some(Operand::Reg(Register::A)),
                source: None,  // TODO
                size: 3,
                cycles: 10
            },
            0xC3 => Instruction {
                op: Operation::JMP,
                target: None, // TODO 
                source: None, // TODO
                size: 3,
                cycles: 10
            },
            0xC5 => Instruction {
                op: Operation::PUSH,
                target: None, // TODO 
                source: None, // TODO
                size: 1,
                cycles: 11
            },
            _ => unimplemented!("instruction {:#x?} has not yet been implemented", opcode)
        };

        Ok(instruction)
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

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "{:?}", self.op);

        if let Some(target) = &self.target {
            write!(f, " {:?}", target)?;
        }

        if let Some(source) = &self.source {
            write!(f, " {:?}", source)?;
        }
        
        res
    }
}

