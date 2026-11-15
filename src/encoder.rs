#[cfg(test)]
mod tests {
    use crate::instruction::{
        IInstruction, Immediate12, RegisterNumber, SharedInstruction, add::Add,
    };

    #[test]
    fn test_encode_add() {
        let instruction = Add(SharedInstruction {
            rd: RegisterNumber::new(0).unwrap(),
            rn: RegisterNumber::new(1).unwrap(),
            imm12: Immediate12::new(5).unwrap(),
        });

        let bytes = instruction.encode().to_le_bytes();
        assert_eq!(bytes, [0x20, 0x14, 0x00, 0x91]);
    }
}
