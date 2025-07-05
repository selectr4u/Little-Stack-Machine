use crate::lsm::instruction::{Instruction, RawInstruction};

/*
instruction set for reference
NAME - OPCODE - DETAILS
PUSH - 1 - expects number as operand and pushes it onto stack
POP - 2 - pops a value off the stack
ADD - 3 - adds the two topmost values on the stack
MUL - 4 - multiplies the two topmost values on the stack
SUB - 5 - subtracts second value on stack by first value on the stack
DIV - 6 - divides second value on stack by first value on the stack
MOD - 7 - gets the modulus for second value on stack by first value on the stack
BRZ - 8 - expects virtual address and it branches to that if
HLT - 0 - halts the program
 */

pub type OperandSize = isize;

pub struct VM {
    instruction_set: Vec<Instruction>,
    code: Vec<RawInstruction>,
    stack: Vec<OperandSize>,
    pc: usize,
}

impl VM {
    pub fn new(instruction_set: Vec<Instruction>) -> VM {
        Self::new(instruction_set)
    }

    pub fn load_bytecode(&mut self, bytecode: &Vec<u8>) {
        // every virtual register is technically 1 byte + OperandSize
        // so we go through the vector of bytes in increments of (1 byte + OperandSize)

    }
}