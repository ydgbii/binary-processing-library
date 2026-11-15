pub mod add;
pub mod sub;

use super::error::{DecodeError, EncodeError};

pub struct RegisterNumber(u8);

impl RegisterNumber {
    pub fn new(v: u8) -> Option<Self> {
        if v > 31 { None } else { Some(Self(v)) }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

pub struct Immediate12(u16);

impl Immediate12 {
    pub fn new(v: u16) -> Option<Self> {
        if v > 4095 { None } else { Some(Self(v)) }
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

pub struct SharedInstruction {
    pub rd: RegisterNumber,
    pub rn: RegisterNumber,
    pub imm12: Immediate12,
}

pub enum Instruction {
    Add(add::Add),
    Sub(sub::SUB),
}

impl Instruction {
    pub fn decode(v: u32) -> Result<Self, DecodeError> {
        if add::Add::matches_opcode(v) {
            return Ok(Instruction::Add(add::Add::decode(v)?));
        }

        if sub::SUB::matches_opcode(v) {
            return Ok(Instruction::Sub(sub::SUB::decode(v)?));
        }

        Err(DecodeError::UnknownInstruction)
    }

    pub fn encode(&self) -> Result<u32, EncodeError> {
        match self {
            Instruction::Add(add) => Ok(add.encode()),
            Instruction::Sub(sub) => Ok(sub.encode()),
        }
    }
}
pub trait IInstruction: Sized {
    const MASK: u32;
    const OP_CODE: u32;
    fn encode(&self) -> u32;
    fn decode(i: u32) -> Result<Self, DecodeError>;
    fn matches_opcode(i: u32) -> bool {
        (i & Self::MASK) == Self::OP_CODE
    }
}
