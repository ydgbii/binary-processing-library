use super::error::DecodeError;
use super::instruction::add::ADD;
use super::instruction::sub::SUB;
use super::instruction::Instruction;
use super::instruction::Arm64Instruction;

/// Decodes ARM64 machine-code bytes.
pub struct Decoder;

impl Decoder {
    /// Decodes an ADD instruction from a byte slice.
    pub fn decode_add(bytes: &[u8]) -> Result<ADD, DecodeError> {
        if bytes.len() != 4 {
            return Err(DecodeError::InvalidInputLength);
        }

        let word = u32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
        ]);

        ADD::decode(word).ok_or(DecodeError::UnknownInstruction)
    }
    /// Decodes a SUB instruction from a byte slice.
    pub fn decode_sub(bytes: &[u8]) -> Result<SUB, DecodeError> {
    if bytes.len() != 4 {
        return Err(DecodeError::InvalidInputLength);
    }

    let word = u32::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
    ]);

    SUB::decode(word).ok_or(DecodeError::UnknownInstruction)
}

pub fn decode_buffer(
    bytes: &[u8],
) -> Result<Vec<Arm64Instruction>, DecodeError> {
    if bytes.len() % 4 != 0 {
        return Err(DecodeError::InvalidInputLength);
    }

    let mut instructions = Vec::new();

    for chunk in bytes.chunks_exact(4) {
        let word = u32::from_le_bytes([
            chunk[0],
            chunk[1],
            chunk[2],
            chunk[3],
        ]);

        if let Some(add) = ADD::decode(word) {
            instructions.push(Arm64Instruction::Add(add));
        } else if let Some(sub) = SUB::decode(word) {
            instructions.push(Arm64Instruction::Sub(sub));
        } else {
            return Err(DecodeError::UnknownInstruction);
        }
    }

    Ok(instructions)
}
}
/// Tests for the decoder
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_add() {
        let bytes = [0x20, 0x14, 0x00, 0x91];

        let instruction = Decoder::decode_add(&bytes).unwrap();

        assert_eq!(instruction.0.rd, 0);
        assert_eq!(instruction.0.rn, 1);
        assert_eq!(instruction.0.imm12, 5);
    }

    /// Test to ensure that encoding and then decoding an instruction preserves all operands.
    #[test]
fn test_encoder_decoder_round_trip() {
    use crate::arm64::encoder::Encoder;
    use crate::arm64::instruction::SharedInstruction;

    let original = ADD(SharedInstruction {
        rd: 0,
        rn: 1,
        imm12: 5,
    });

    // Encode the instruction into bytes.
    let bytes = Encoder::encode_add(&original).unwrap();

    // Decode the bytes back into an instruction.
    let decoded = Decoder::decode_add(&bytes).unwrap();

    // Verify that all operands are preserved.
    assert_eq!(decoded.0.rd, original.0.rd);
    assert_eq!(decoded.0.rn, original.0.rn);
    assert_eq!(decoded.0.imm12, original.0.imm12);
}

/// Test to ensure that decoding an instruction with an invalid input length returns the appropriate error.
#[test]
fn test_invalid_input_length() {
    let bytes = [0x20, 0x14, 0x00];

    let result = Decoder::decode_add(&bytes);

    assert!(matches!(
        result,
        Err(DecodeError::InvalidInputLength)
    ));
}

#[test]
fn test_unknown_instruction() {
    let bytes = [0xFF, 0xFF, 0xFF, 0xFF];

    let result = Decoder::decode_add(&bytes);

    assert!(matches!(
        result,
        Err(DecodeError::UnknownInstruction)
    ));
}

/// Test to ensure that decoding a SUB instruction from bytes works correctly.
#[test]
fn test_sub_encoder_decoder_round_trip() {
    use crate::arm64::encoder::Encoder;
    use crate::arm64::instruction::SharedInstruction;

    let original = SUB(SharedInstruction {
        rd: 0,
        rn: 1,
        imm12: 4,
    });

    let bytes = Encoder::encode_sub(&original).unwrap();

    assert_eq!(bytes, [0x20, 0x10, 0x00, 0xD1]);

    let decoded = Decoder::decode_sub(&bytes).unwrap();

    assert_eq!(decoded.0.rd, 0);
    assert_eq!(decoded.0.rn, 1);
    assert_eq!(decoded.0.imm12, 4);
}

#[test]
fn test_decode_buffer() {
    let bytes = [
        0x20, 0x14, 0x00, 0x91, // ADD X0, X1, #5
        0x20, 0x10, 0x00, 0xD1, // SUB X0, X1, #4
    ];

    let instructions = Decoder::decode_buffer(&bytes).unwrap();

    assert_eq!(instructions.len(), 2);

    assert!(matches!(
        instructions[0],
        Arm64Instruction::Add(_)
    ));

    assert!(matches!(
        instructions[1],
        Arm64Instruction::Sub(_)
    ));
}
}