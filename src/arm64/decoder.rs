use super::error::DecodeError;
use super::instruction::add::ADD;
use super::instruction::Instruction;

/// Decodes ARM64 machine-code bytes.
pub struct Decoder;

impl Decoder {
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
}