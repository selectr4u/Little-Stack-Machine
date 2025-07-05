use crate::lsm::vm::{OperandSize, VM};

pub type OpcodeSize = u8;

pub type InstructionFunc = fn(&mut VM, Option<OperandSize>);

#[derive(Clone)]
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
OUT - 11 - prints the topmost item on the stack (debug)
DUP - 12 - duplicates the top of the stack
HLT - 0 - halts the program
 */

pub const DEFAULT_INSTRUCTION_SET: &[Instruction] = &[
    Instruction {
        name: "PUSH",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            vm.push(operand.unwrap());
        }
    },
    Instruction {
        name: "POP",
        opcode: 2,
        requires_operand: false,
        func: |vm, _operand| {
            vm.pop().unwrap();
        }
    },
    Instruction {
        name: "ADD",
        opcode: 3,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(a + b);
        }
    },
    Instruction {
        name: "MUL",
        opcode: 4,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(a * b);
        }
    },
    Instruction {
        name: "SUB",
        opcode: 5,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(b - a);
        }
    },
    Instruction {
        name: "DIV",
        opcode: 6,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(b / a);
        }
    },
    Instruction {
        name: "MOD",
        opcode: 7,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(b % a);
        }
    },
    Instruction {
        name: "BRZ",
        opcode: 8,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().expect("stack underflow > 1");
            if a == 0 {
                vm.branch(operand.unwrap());
            }
        }
    },
    Instruction {
        name: "BRP",
        opcode: 9,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().expect("stack underflow > 1");
            if a >= 0 {
                vm.branch(operand.unwrap());
            }
        }
    },
    Instruction {
        name: "BRA",
        opcode: 10,
        requires_operand: true,
        func: |vm, operand| {
            vm.branch(operand.unwrap());
        }
    },
    Instruction {
        name: "HLT",
        opcode: 0,
        requires_operand: true,
        func: |vm, _operand| {
            vm.halt();
        }
    },
    Instruction {
        name: "OUT",
        opcode: 11,
        requires_operand: true,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            println!("{}", a);
            // and push back on stack for like a peek like behaviour
            vm.push(a);
        }
    },
    Instruction {
        name: "DUP",
        opcode: 12,
        requires_operand: true,
        func: |vm, _operand| {
            let a_ref = vm.peek().expect("stack underflow > 1");
            let a = a_ref.clone();
            vm.push(a);
        }
    },
];