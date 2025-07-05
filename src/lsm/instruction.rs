use crate::lsm::vm::{OperandSize, VM};

type OpcodeSize = u8;
type ArgCountSize = u8;

pub type InstructionFunc = fn(&mut VM, OperandSize);

pub struct Instruction {
    name:      &'static str,
    opcode: OpcodeSize,
    arg_count: ArgCountSize,
    func: InstructionFunc,
}

pub struct RawInstruction {
    opcode: OpcodeSize,
    operand: OperandSize,
}