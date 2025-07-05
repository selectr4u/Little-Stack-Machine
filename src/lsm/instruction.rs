use crate::lsm::vm::{OperandSize, VM};

pub type OpcodeSize = u8;

pub type InstructionFunc = fn(&mut VM, OperandSize);

pub struct Instruction {
    pub name:      &'static str,
    pub opcode: OpcodeSize,
    pub requires_operand: bool,
    pub func: InstructionFunc,
}

pub struct RawInstruction {
    pub opcode: OpcodeSize,
    pub operand: Option<OperandSize>,
}

// default instruction set
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
BRZ - 8 - expects virtual address, and it branches to that if top value on stack is zero
BRP - 9 - expects virtual address, and it branches to that if top value on stack is zero or positive
BRA - 10 - expects virtual address, and it branches to that
HLT - 0 - halts the program
 */

pub const DEFAULT_INSTRUCTION_SET: Vec<Instruction> = vec![
    Instruction{
        name: "PUSH",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            vm.push(operand);
        }
    },
    Instruction{
        name: "POP",
        opcode: 2,
        requires_operand: false,
        func: |vm, operand| {
            vm.pop().unwrap();
        }
    },
    Instruction{
        name: "ADD",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            let b = vm.pop().unwrap();
            vm.push(a + b);
        }
    },
    Instruction{
        name: "MUL",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            let b = vm.pop().unwrap();
            vm.push(a * b);
        }
    },
    Instruction{
        name: "SUB",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            let b = vm.pop().unwrap();
            vm.push(b - a);
        }
    },
    Instruction{
        name: "DIV",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            let b = vm.pop().unwrap();
            vm.push(b / a);
        }
    },
    Instruction{
        name: "MOD",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            let b = vm.pop().unwrap();
            vm.push(b % a);
        }
    },
    Instruction{
        name: "BRZ",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            if a == 0 {
                vm.branch(operand);
            }
        }
    },
    Instruction{
        name: "BRP",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            if a >= 0 {
                vm.branch(operand);
            }
        }
    },
    Instruction{
        name: "BRA",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().unwrap();
            vm.branch(operand);
        }
    },
    Instruction{
        name: "HLT",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            vm.halt();
        }
    },
];