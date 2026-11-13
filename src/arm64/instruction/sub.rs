use super::{Instruction, SharedInstruction};
use crate::arm64::error::EncodeError;

/// SUB immediate instruction for ARM64.
/// Supports 64-bit SUB with shift = 0.
pub struct SUB(pub SharedInstruction);

impl Instruction for SUB {
    const MASK: u32 = 0xFFC00000;
    const OP_CODE: u32 = 0xD1000000;

    fn encode(&self) -> Result<u32, EncodeError> {
        if self.0.rd > 31 {
            return Err(EncodeError::InvalidRegister {
                field: "rd",
                value: self.0.rd,
            });
        }

        if self.0.rn > 31 {
            return Err(EncodeError::InvalidRegister {
                field: "rn",
                value: self.0.rn,
            });
        }

        if self.0.imm12 > 4095 {
            return Err(EncodeError::InvalidImmediate {
                value: self.0.imm12,
            });
        }

        Ok(Self::OP_CODE
            | ((self.0.imm12 as u32) << 10)
            | ((self.0.rn as u32) << 5)
            | (self.0.rd as u32))
    }

    fn decode(i: u32) -> Option<Self> {
        if !Self::matches_opcode(i) {
            return None;
        }

        Some(SUB(SharedInstruction {
            rd: (i & 0x1F) as u8,
            rn: ((i >> 5) & 0x1F) as u8,
            imm12: ((i >> 10) & 0xFFF) as u16,
        }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    /// Test to ensure that encoding and then decoding a SUB instruction preserves all operands.
    #[test]
    fn test_sub_encode() {
        let instruction = SUB(SharedInstruction {
            rd: 0,
            rn: 1,
            imm12: 4,
        });

        let encoded = instruction.encode().unwrap();

        assert_eq!(encoded, 0xD1001020);
    }

    #[test]
    fn test_sub_decode() {
        let encoded = 0xD1001020;

        let instruction = SUB::decode(encoded).unwrap();

        assert_eq!(instruction.0.rd, 0);
        assert_eq!(instruction.0.rn, 1);
        assert_eq!(instruction.0.imm12, 4);
    }

    #[test]
    fn test_sub_round_trip() {
        let original = SUB(SharedInstruction {
            rd: 2,
            rn: 3,
            imm12: 10,
        });

        let encoded = original.encode().unwrap();
        let decoded = SUB::decode(encoded).unwrap();

        assert_eq!(decoded.0.rd, original.0.rd);
        assert_eq!(decoded.0.rn, original.0.rn);
        assert_eq!(decoded.0.imm12, original.0.imm12);
    }
}