use super::{Instruction, SharedInstruction};
use crate::arm64::error::EncodeError;

/// ADD immediate instruction for ARM64.
pub struct ADD(pub SharedInstruction);

impl Instruction for ADD {
    const MASK: u32 = 0xFFC00000;
    const OP_CODE: u32 = 0x91000000;

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

    Ok(
        Self::OP_CODE
            | ((self.0.imm12 as u32) << 10)
            | ((self.0.rn as u32) << 5)
            | (self.0.rd as u32)
    )
}
    fn decode(i: u32) -> Option<Self> {
        if !Self::matches_opcode(i) {
            return None;
        }

        Some(ADD(SharedInstruction {
            rd: (i & 0b11111) as u8,
            rn: ((i >> 5) & 0b11111) as u8,
            imm12: ((i >> 10) & 0xFFF) as u16,
        }))
    }
}

 #[test]
    fn test_add_encode(){
        let instruction= ADD(SharedInstruction{
            rd:0,
            rn:1,
            imm12: 5,
        });
        assert_eq!(instruction.encode().unwrap(), 0x91001420);
        }

#[test]
    fn test_add_decode(){
        let instruction= ADD::decode(0x91001420).unwrap();
    
        assert_eq!(instruction.0.rd, 0);
        assert_eq!(instruction.0.rn, 1);
        assert_eq!(instruction.0.imm12, 5);
    }
#[test]
    fn test_add_round_trip(){
        let original= ADD(SharedInstruction{
            rd:0,
            rn:1,
            imm12: 5,
        });
        let encoded= original.encode().unwrap();
        let decoded= ADD::decode(encoded).unwrap();
        assert_eq!(original.0.rd, decoded.0.rd);
        assert_eq!(original.0.rn, decoded.0.rn);
        assert_eq!(original.0.imm12, decoded.0.imm12);
    }

    #[test]
    fn test_add_invalid_register(){
        let instruction= ADD(SharedInstruction{
            rd:40,
            rn:1,
            imm12:5,
        });
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidRegister {
            field: "rd",
            value: 40,
        })
    );
    }
    #[test]
    fn test_add_invalid_immediate(){
        let instruction= ADD(SharedInstruction{
        rd:0,
        rn:1,
        imm12:5000,});
        assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidImmediate {
            value: 5000,
        })
     );
    }