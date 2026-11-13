use super::error::EncodeError;
use super::instruction::add::ADD;
use super::instruction::Instruction;
//encode arm64 instruction to bytes
pub struct Encoder;
impl Encoder{
    //encodes one add instuctions into four bytes 
    pub fn encode_add(instruction: &ADD) -> Result<[u8; 4], EncodeError> {
        let encoded = instruction.encode()?;
        Ok(encoded.to_le_bytes())
    }
}

///tests for the encoder
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm64::instruction::SharedInstruction;

    #[test]
    fn test_encode_add() {
        let instruction = ADD(SharedInstruction {
            rd: 0,
            rn: 1,
            imm12: 5,
        });

        let bytes = Encoder::encode_add(&instruction).unwrap();

        assert_eq!(bytes, [0x20, 0x14, 0x00, 0x91]);
    }
}