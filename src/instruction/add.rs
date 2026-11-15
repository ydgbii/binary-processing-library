use super::{IInstruction, Immediate12, RegisterNumber, SharedInstruction};
use crate::error::DecodeError;

/// ADD immediate instruction for ARM64.
pub struct Add(pub SharedInstruction);

impl IInstruction for Add {
    const MASK: u32 = 0xFFC00000;
    const OP_CODE: u32 = 0x91000000;

    fn encode(&self) -> u32 {
        Self::OP_CODE
            | ((self.0.imm12.value() as u32) << 10)
            | ((self.0.rn.value() as u32) << 5)
            | (self.0.rd.value() as u32)
    }

    fn decode(i: u32) -> Result<Self, DecodeError> {
        if !Self::matches_opcode(i) {
            return Err(DecodeError::UnknownInstruction);
        }

        let rd =
            RegisterNumber::new((i & 0b11111) as u8).ok_or(DecodeError::InvalidRegisterNumber)?;

        let rn = RegisterNumber::new(((i >> 5) & 0b11111) as u8)
            .ok_or(DecodeError::InvalidRegisterNumber)?;

        let imm12 = Immediate12::new(((i >> 10) & 0xFFF) as u16)
            .expect("A 12-bit field always fits in Immediate12");

        Ok(Add(SharedInstruction { rd, rn, imm12 }))
    }
}

#[test]
fn test_add_encode() {
    let instruction = Add(SharedInstruction {
        rd: RegisterNumber::new(0).unwrap(),
        rn: RegisterNumber::new(1).unwrap(),
        imm12: Immediate12::new(5).unwrap(),
    });
    assert_eq!(instruction.encode(), 0x91001420);
}

#[test]
fn test_add_decode() {
    let instruction = Add::decode(0x91001420).unwrap();

    assert_eq!(instruction.0.rd.value(), 0);
    assert_eq!(instruction.0.rn.value(), 1);
    assert_eq!(instruction.0.imm12.value(), 5);
}

#[test]
fn test_add_round_trip() {
    let original = Add(SharedInstruction {
        rd: RegisterNumber::new(0).unwrap(),
        rn: RegisterNumber::new(1).unwrap(),
        imm12: Immediate12::new(5).unwrap(),
    });

    let encoded = original.encode();
    let decoded = Add::decode(encoded).unwrap();

    assert_eq!(original.0.rd.value(), decoded.0.rd.value());
    assert_eq!(original.0.rn.value(), decoded.0.rn.value());
    assert_eq!(original.0.imm12.value(), decoded.0.imm12.value());
}

#[test]
fn test_add_invalid_register() {
    assert!(RegisterNumber::new(40).is_none());
    assert!(RegisterNumber::new(31).is_some());
}

#[test]
fn test_add_invalid_immediate() {
    assert!(Immediate12::new(5000).is_none());
    assert!(Immediate12::new(4095).is_some());
}
