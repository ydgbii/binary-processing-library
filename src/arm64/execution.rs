use super::instruction::add::ADD;

///We simulate the execution of an ADD instruction by performing the addition of the two operands and returning the result.
pub fn execute_add(instruction: &ADD, registers: &mut)