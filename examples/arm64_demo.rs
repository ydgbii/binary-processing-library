
use base64::{Engine as _, engine::general_purpose::STANDARD};
use binary_processing_library::decoder::decode_buffer;
use binary_processing_library::instruction::Instruction;

fn main() {
    let encoded = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ABQAkQAQANE=".to_string());

    let bytes = STANDARD
        .decode(&encoded)
        .expect("Invalid Base64 input");

    println!("ARM64 Instruction Decoder");
    println!("-------------------------");
    println!("Input Base64: {}", encoded);
    println!("Decoded bytes: {}", bytes.len());


    let instructions =
        decode_buffer(&bytes).expect("Decoding failed");

    println!("Instructions: {}\n", instructions.len());

    for (index, instruction) in instructions.iter().enumerate() {
        let word = u32::from_le_bytes(
            bytes[index * 4..index * 4 + 4]
                .try_into()
                .unwrap(),
        );

        match instruction {
            Instruction::Add(add) => {
                println!(
                    "[{}] ADD X{}, X{}, #{}",
                    index,
                    add.0.rd.value(),
                    add.0.rn.value(),
                    add.0.imm12.value()
                );
            }

            Instruction::Sub(sub) => {
                println!(
                    "[{}] SUB X{}, X{}, #{}",
                    index,
                    sub.0.rd.value(),
                    sub.0.rn.value(),
                    sub.0.imm12.value()
                );
            }
        }

        println!("    Machine code: 0x{:08X}\n", word);
    }

    println!("Decoding completed successfully.");
}

