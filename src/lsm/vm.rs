use crate::lsm::instruction::{Instruction, RawInstruction, OpcodeSize};
use crate::lsm::stack::Stack;

const DEFAULT_STACK_SIZE: usize = 128; // artificial limit

pub type OperandSize = isize;

pub struct VM {
    instruction_set: Vec<Instruction>,
    code: Vec<RawInstruction>,
    stack: Stack<OperandSize>,
    pc: usize,
    stop: bool,
}

impl VM {
    pub fn new(instruction_set: Vec<Instruction>, initial_code : Option<Vec<RawInstruction>>, stack_size: Option<usize>  ) -> VM {
        let local_stack_size : usize;

        match stack_size {
            Some(size) => local_stack_size = size,
            None => local_stack_size = DEFAULT_STACK_SIZE
        }

        match initial_code {
            Some(raw) => {
                VM{instruction_set, code: raw, stack: Stack::new(local_stack_size), pc: 0, stop: false }
            }
            None => {
                VM{instruction_set: vec![], code: vec![], stack: Stack::new(local_stack_size), pc: 0, stop: false}
            }
        }

    }

    // finds the matching instruction struct for the opcode
    pub fn get_instruction_match_for_opcode(&self, opcode: OpcodeSize) -> Option<&Instruction> {
        for instruction in self.instruction_set.iter() {
            if instruction.opcode == opcode {
                return Some(instruction);
            }
        }

        None
    }

    // loads bytecode into the code memory of the vm



    pub fn load_bytecode(&mut self, bytecode: &[u8]) {
        // every virtual register is technically OpcodeSize + OperandSize
        // so we go through the vector of bytes in increments of (OpcodeSize + OperandSize)
        let opcode_bytes_size = size_of::<OpcodeSize>();
        let operand_bytes_size = size_of::<OperandSize>();

        let raw_instruction_size = size_of::<OpcodeSize>() + size_of::<OperandSize>();

        let mut raw_instructions_vec = Vec::with_capacity(bytecode.len() / raw_instruction_size);

        let mut local_opcode: OpcodeSize = 0;
        let mut local_operand: OperandSize;
        let mut bytes_encountered = Vec::with_capacity(raw_instruction_size);
        let mut opcode_encountered = false;

        for byte in bytecode {
            if opcode_encountered {
                if bytes_encountered.len() == operand_bytes_size {
                    // we've got the operand now!
                    let raw_operand = bytes_encountered.clone();
                    local_operand = OperandSize::from_le_bytes(raw_operand.try_into().unwrap());

                    // reset bytes encountered
                    bytes_encountered = Vec::with_capacity(operand_bytes_size);

                    // now form our instruction and push!
                    raw_instructions_vec.push(RawInstruction{opcode: local_opcode, operand: Some(local_operand)});

                    opcode_encountered = false;
                }
            } else {
                if bytes_encountered.len() == opcode_bytes_size {
                    opcode_encountered = true;
                    let raw_opcode = bytes_encountered.clone();
                    local_opcode = OpcodeSize::from_le_bytes(raw_opcode.try_into().unwrap());

                    // reset bytes encountered
                    bytes_encountered = Vec::with_capacity(raw_instruction_size);

                    // we've encountered our opcode, but let's check we also need operand
                    // this does mean searching through our set of instructions
                    let instruction = self.get_instruction_match_for_opcode(local_opcode);
                    match instruction {
                        Some(instruction) => {
                            if !instruction.requires_operand {
                                // finalise instruction and push since it doesn't require operand
                                raw_instructions_vec.push(RawInstruction{opcode : local_opcode, operand : None});
                                // back to searching for more opcodes!
                                opcode_encountered = false;
                            }
                        }
                        None => {
                            // no matching instruction </3
                            panic!("illegal instruction");
                        }
                    }
                }

            }

            bytes_encountered.push(*byte);
        }

        self.code.append(&mut raw_instructions_vec);
    }


    // runs the vm
    pub fn run(&mut self) {
        self.stop = false;

        loop {
            if self.stop == true {
                break;
            }

            self.pc += 1;

            let current_address = self.pc - 1; //  so a branch doesnt need to do (addr - 1)

            // and a check to make sure we don't go out of limits
            if current_address >= self.code.len() {
                self.stop = true;
                break;
            }

            let current_raw_instruction = &self.code[current_address];

            let opcode = current_raw_instruction.opcode;
            let operand = current_raw_instruction.operand;

            let matching_instruction = self.get_instruction_match_for_opcode(opcode);

            if matching_instruction.is_none() {
                self.stop = true;
                break;
            }

            (matching_instruction.unwrap().func)(self, operand);
        }
    }

    // pops the topmost item off of the operand stack
    pub fn pop(&mut self) -> Option<OperandSize> {
        self.stack.pop()
    }

    // pushes the supplied operand onto the operand stack
    pub fn push(&mut self, operand: OperandSize) {
        self.stack.push(operand);
    }

    // peeks at the top of the operand stack
    pub fn peek(&mut self) -> Option<&OperandSize> {
        self.stack.peek()
    }

    // branches to supplied virtual address
    pub fn branch(&mut self, operand: OperandSize) {
        self.pc = operand as usize
    }

    // halts the vm
    pub fn halt(&mut self) {
        self.stop = true;
    }

    // dumps the contents of the code memory (bytecode) into a readable manner
    pub fn dump(&self) -> String {
        !unimplemented!()
    }
}