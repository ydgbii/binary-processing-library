use super::instruction::add::ADD;
use super::registers::Registers;

pub fn execute_add(
    instruction: &ADD,
    registers: &mut Registers,
) {
    let rn = instruction.0.rn;
    let rd = instruction.0.rd;
    let immediate = instruction.0.imm12 as u64;

    let value = registers.read(rn);
    let result = value.wrapping_add(immediate);

    registers.write(rd, result);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm64::decoder::Decoder;
    use crate::arm64::encoder::Encoder;
    use crate::arm64::instruction::SharedInstruction;

    #[test]
    fn test_add_execution() {
        let mut registers = Registers::new();

        // X1 = 7
        registers.write(1, 7);

        // ADD X0, X1, #5
        let instruction = ADD(SharedInstruction {
            rd: 0,
            rn: 1,
            imm12: 5,
        });

        // Encode to ARM64 machine code.
        let bytes = Encoder::encode_add(&instruction).unwrap();

        // Decode the machine code.
        let decoded = Decoder::decode_add(&bytes).unwrap();

        // Execute the decoded instruction.
        execute_add(&decoded, &mut registers);

        println!("Machine code: {:02X?}", bytes);
        println!("7 + 5 = {}", registers.read(0));

        assert_eq!(registers.read(0), 12);
    }


    ///to test that the addition of two numbers does not overflow and wraps around correctly
    #[test]
fn test_add_overflow() {
    let mut registers = Registers::new();

    registers.write(1, u64::MAX);

    let instruction = ADD(SharedInstruction {
        rd: 0,
        rn: 1,
        imm12: 1,
    });

    let bytes = Encoder::encode_add(&instruction).unwrap();
    let decoded = Decoder::decode_add(&bytes).unwrap();

    execute_add(&decoded, &mut registers);

    assert_eq!(registers.read(0), 0);
}


/// test function reads original value correctly before recording the result 
#[test]
fn test_add_same_register() {
    let mut registers = Registers::new();

    registers.write(1, 7);

    // ADD X1, X1, #5
    let instruction = ADD(SharedInstruction {
        rd: 1,
        rn: 1,
        imm12: 5,
    });

    let bytes = Encoder::encode_add(&instruction).unwrap();
    let decoded = Decoder::decode_add(&bytes).unwrap();

    execute_add(&decoded, &mut registers);

    assert_eq!(registers.read(1), 12);
}


/// Test to ensure that the stack pointer (SP) can be used correctly in an ADD instruction.
#[test]
fn test_add_stack_pointer() {
    let mut registers = Registers::new();

    registers.write(31, 0x1000);

    // ADD SP, SP, #16
    let instruction = ADD(SharedInstruction {
        rd: 31,
        rn: 31,
        imm12: 16,
    });

    let bytes = Encoder::encode_add(&instruction).unwrap();
    let decoded = Decoder::decode_add(&bytes).unwrap();

    execute_add(&decoded, &mut registers);

    assert_eq!(registers.read(31), 0x1010);
}
}