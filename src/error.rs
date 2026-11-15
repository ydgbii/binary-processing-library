#[derive(Debug, PartialEq)]
pub enum EncodeError {
    InvalidRegister { field: &'static str, value: u8 },
    InvalidImmediate { value: u16 },
}
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    InvalidInputLength,
    UnknownInstruction,
    InvalidRegisterNumber,
}
