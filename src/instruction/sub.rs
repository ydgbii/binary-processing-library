use super::{IInstruction, Immediate12, RegisterNumber, SharedInstruction};
use crate::error::DecodeError;

/// SUB immediate instruction for ARM64.
pub struct SUB(pub SharedInstruction);

impl IInstruction for SUB {
    const MASK: u32 = 0xFFC00000;
    const OP_CODE: u32 = 0xD1000000;

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

        let rd = RegisterNumber::new((i & 0x1F) as u8).ok_or(DecodeError::InvalidRegisterNumber)?;

        let rn = RegisterNumber::new(((i >> 5) & 0x1F) as u8)
            .ok_or(DecodeError::InvalidRegisterNumber)?;

        let imm12 = Immediate12::new(((i >> 10) & 0xFFF) as u16)
            .expect("A 12-bit field always fits in Immediate12");

        Ok(SUB(SharedInstruction { rd, rn, imm12 }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Test to ensure that encoding and then decoding a SUB instruction preserves all operands.

    #[test]
    fn test_sub_encode() {
        let instruction = SUB(SharedInstruction {
            rd: RegisterNumber::new(0).unwrap(),
            rn: RegisterNumber::new(1).unwrap(),
            imm12: Immediate12::new(4).unwrap(),
        });

        let encoded = instruction.encode();

        assert_eq!(encoded, 0xD1001020);
    }

    #[test]
    fn test_sub_decode() {
        let encoded = 0xD1001020;

        let instruction = SUB::decode(encoded).unwrap();

        assert_eq!(instruction.0.rd.value(), 0);
        assert_eq!(instruction.0.rn.value(), 1);
        assert_eq!(instruction.0.imm12.value(), 4);
    }

    #[test]
    fn test_sub_round_trip() {
        let original = SUB(SharedInstruction {
            rd: RegisterNumber::new(2).unwrap(),
            rn: RegisterNumber::new(3).unwrap(),
            imm12: Immediate12::new(10).unwrap(),
        });

        let encoded = original.encode();
        let decoded = SUB::decode(encoded).unwrap();

        assert_eq!(decoded.0.rd.value(), original.0.rd.value());
        assert_eq!(decoded.0.rn.value(), original.0.rn.value());
        assert_eq!(decoded.0.imm12.value(), original.0.imm12.value());
    }
}
