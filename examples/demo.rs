
use binary_processing_library::arm64::decoder::Decoder;
use binary_processing_library::arm64::encoder::Encoder;
use binary_processing_library::arm64::execution::{
    execute_add, execute_sub,
};
use binary_processing_library::arm64::instruction::add::ADD;
use binary_processing_library::arm64::instruction::sub::SUB;
use binary_processing_library::arm64::instruction::SharedInstruction;
use binary_processing_library::arm64::registers::Registers;

fn main() {
    let a = 10u64;
    let b = 4u16;

    println!("=== ARM64 ENCODER / DECODER DEMO ===");
    println!("Input: A = {}, B = {}", a, b);

    let add = ADD(SharedInstruction {
        rd: 0,
        rn: 1,
        imm12: b,
    });

    let sub = SUB(SharedInstruction {
        rd: 0,
        rn: 1,
        imm12: b,
    });

    // ADD: encode -> decode -> execute
    let add_bytes = Encoder::encode_add(&add).unwrap();
    let add_word = u32::from_le_bytes(add_bytes);
    let decoded_add = Decoder::decode_add(&add_bytes).unwrap();

    let mut registers = Registers::new();
    registers.write(1, a);
    execute_add(&decoded_add, &mut registers);

    println!("\n--- ADD ---");
    println!("Instruction: ADD X0, X1, #{}", b);
    println!("Machine code: 0x{:08X}", add_word);
    println!("Binary: {:032b}", add_word);
    println!("Bytes (little-endian): {:02X?}", add_bytes);
    println!("Decoded: ADD X{}, X{}, #{}",
        decoded_add.0.rd,
        decoded_add.0.rn,
        decoded_add.0.imm12
    );
    println!("Result: {} + {} = {}", a, b, registers.read(0));

    // SUB: encode -> decode -> execute
    let sub_bytes = Encoder::encode_sub(&sub).unwrap();
    let sub_word = u32::from_le_bytes(sub_bytes);
    let decoded_sub = Decoder::decode_sub(&sub_bytes).unwrap();

    registers.write(1, a);
    execute_sub(&decoded_sub, &mut registers);

    println!("\n--- SUB ---");
    println!("Instruction: SUB X0, X1, #{}", b);
    println!("Machine code: 0x{:08X}", sub_word);
    println!("Binary: {:032b}", sub_word);
    println!("Bytes (little-endian): {:02X?}", sub_bytes);
    println!("Decoded: SUB X{}, X{}, #{}",
        decoded_sub.0.rd,
        decoded_sub.0.rn,
        decoded_sub.0.imm12
    );
    println!("Result: {} - {} = {}", a, b, registers.read(0));
}
