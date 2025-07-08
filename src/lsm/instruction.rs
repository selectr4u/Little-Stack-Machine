use crate::lsm::vm::{OperandSize, ToNumber, Value, VM};

pub type OpcodeSize = u8;

pub type InstructionFunc = fn(&mut VM, Option<Value>);

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
DUP - 11 - duplicates the top of the stack
OUT - 100 - prints the topmost item on the stack (debug)
HLT - 0 - halts the program
 */

impl ToNumber for Value {
    fn to_number(self) -> Result<OperandSize, ()> {
        // we want to consume it as well bc we're turning it to a number so no &self but self
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(()),
        }
    }
}


pub const DEFAULT_INSTRUCTION_SET: &[Instruction] = &[
    Instruction {
        name: "PUSH",
        opcode: 1,
        requires_operand: true,
        func: |vm, operand| {
            // although it's a value, i mean we can push anything provided..
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
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");

            // check they are both numbers
            let is_a_num = matches!(a, Value::Number {..});
            let is_b_num = matches!(b, Value::Number {..});

            if is_a_num && is_b_num {
                vm.push(Value::Number(a.to_number().unwrap() + b.to_number().unwrap()));
            } else {
                panic!("cannot perform addition on non numbers");
            }
        }
    },
    Instruction {
        name: "MUL",
        opcode: 4,
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(Value::Number(a.to_number().unwrap() * b.to_number().unwrap()));
        }
    },
    Instruction {
        name: "SUB",
        opcode: 5,
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(Value::Number(b.to_number().unwrap() - a.to_number().unwrap()));
        }
    },
    Instruction {
        name: "DIV",
        opcode: 6,
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(Value::Number(b.to_number().unwrap() / a.to_number().unwrap()));
        }
    },
    Instruction {
        name: "MOD",
        opcode: 7,
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            let b = vm.pop().expect("stack underflow > 2");
            vm.push(Value::Number(b.to_number().unwrap() % a.to_number().unwrap()));
        }
    },
    Instruction {
        name: "BRZ",
        opcode: 8,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().expect("stack underflow > 1");
            if a.to_number().expect("couldn't turn into number") == 0 as OperandSize {
                vm.branch(operand.unwrap().to_number().unwrap());
            }
        }
    },
    Instruction {
        name: "BRP",
        opcode: 9,
        requires_operand: true,
        func: |vm, operand| {
            let a = vm.pop().expect("stack underflow > 1");
            if a.to_number().expect("couldn't turn into number") >= 0 as OperandSize {
                vm.branch(operand.unwrap().to_number().unwrap());
            }
        }
    },
    Instruction {
        name: "BRA",
        opcode: 10,
        requires_operand: true,
        func: |vm, operand| {
            vm.branch(operand.unwrap().to_number().unwrap());
        }
    },
    Instruction {
        name: "HLT",
        opcode: 0,
        requires_operand: false,
        func: |vm, _operand| {
            vm.halt();
        }
    },
    Instruction {
        name: "OUT",
        opcode: 100,
        requires_operand: false,
        func: |vm, _operand| {
            let a = vm.pop().expect("stack underflow > 1");
            println!("{:?}", a);
            // and push back on stack for like a peek like behaviour
            vm.push(a);
        }
    },
    Instruction {
        name: "DUP",
        opcode: 11,
        requires_operand: false,
        func: |vm, _operand| {
            let a_ref = vm.peek().expect("stack underflow > 1");
            let a = a_ref.clone();
            vm.push(a);
        }
    },
    Instruction {
        name: "PUSHC",
        opcode: 12,
        requires_operand: true,
        func: |vm, operand| {
            // operand is the index/key for the const pool // TODO clarify
            let a = vm.get_const_copy(operand.clone().unwrap().to_number().expect("operand not supplied/not a proper key"));

            match a {
                Some(a) => {
                    vm.push(a);
                }
                None => {
                    // TODO clarify index or key
                    panic!("no value at index/key provided {:?}", operand.unwrap());
                }
            }
        }
    },
];