use crate::lsm::instruction::{Instruction, RawInstruction, OpcodeSize};

const STACK_SIZE: usize = 128; // artificial limit

pub type OperandSize = isize;

pub struct VM {
    instruction_set: Vec<Instruction>,
    code: Vec<RawInstruction>,
    stack: Vec<OperandSize>,
    pc: usize,
    stop: bool,
}

impl VM {
    pub fn new(instruction_set: Vec<Instruction>) -> VM {
        Self::new(instruction_set)
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
        let mut local_operand: Option<OperandSize>;
        let mut bytes_encountered = Vec::with_capacity(raw_instruction_size);
        let mut opcode_encountered = false;

        for byte in bytecode {
            if opcode_encountered {
                if bytes_encountered.len() == operand_bytes_size {
                    // we've got the operand now!
                    let raw_operand : &[u8] = bytes_encountered.clone().try_into().unwrap();
                    local_operand = raw_operand.try_into().unwrap();

                    // reset bytes encountered
                    bytes_encountered = Vec::with_capacity(operand_bytes_size);

                    // now form our instruction and push!
                    raw_instructions_vec.push(RawInstruction{opcode: local_opcode, operand: local_operand});

                    opcode_encountered = false;
                }
            } else {
                if bytes_encountered.len() == opcode_bytes_size {
                    opcode_encountered = true;
                    let raw_opcode : &[u8] = bytes_encountered.clone().try_into().unwrap();
                    local_opcode = raw_opcode.try_into().unwrap();

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
        self.stop = true;

        loop {
            if self.stop == true {
                break;
            }

            let current_raw_instruction = &self.code[self.pc];

            let opcode = current_raw_instruction.opcode;
            let operand = current_raw_instruction.operand;

            let matching_instruction = self.get_instruction_match_for_opcode(opcode);

            if matching_instruction.is_none() {
                break;
            }

            matching_instruction.unwrap().func(self, operand);

            self.pc += 1; // optimistic so if we want to go to a specific pc, an instruction would go to (addr - 1)
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

    // branches to supplied virtual address
    pub fn branch(&mut self, operand: OperandSize) {
        self.pc = (operand - 1) as usize
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