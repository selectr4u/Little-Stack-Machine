use std::collections::HashMap;
use std::rc::Rc;
use crate::lsm::instruction::{Instruction, RawInstruction, OpcodeSize};
use crate::lsm::stack::Stack;
use crate::lsm::vm::Value::{Number, Str};

const DEFAULT_STACK_SIZE: usize = 128; // artificial limit
const BYTECODE_SIGNATURE: &str = "!LSM!";
const BYTECODE_CONSTS_SIGNATURE: &str = "!CONSTS";
const BYTECODE_INSTRUCTIONS_SIGNATURE: &str = "!INSTR";

pub type OperandSize = f64;
pub type ConstPool = HashMap<usize, Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Number(OperandSize), // OperandSize bytes
    Str(Rc<String>), // dynamic amount of bytes
    Bool(bool), // 1 byte
    Nil, // 1 byte
}

pub trait ToNumber {
    fn to_number(self) -> Result<OperandSize, ()>;
}

impl ToNumber for Value {
    fn to_number(self) -> Result<OperandSize, ()> {
        // we want to consume it as well bc we're turning it to a number so no &self but self
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(()),
        }
    }
}

pub struct VM {
    instruction_set: Vec<Instruction>,
    code: Vec<RawInstruction>,
    const_pool: ConstPool,
    stack: Stack<Value>,
    pc: usize,
    stop: bool,
}

impl VM {
    pub fn new(instruction_set: Vec<Instruction>, initial_code : Option<Vec<RawInstruction>>, initial_consts : Option<ConstPool>,stack_size: Option<usize>  ) -> VM {
        let local_initial_code : Vec<RawInstruction>;
        let local_stack_size : usize;
        let local_initial_consts: ConstPool;

        match initial_code {
            Some(raw) => local_initial_code = raw,
            None => local_initial_code = vec![]
        }

        match initial_consts {
            Some(consts) => local_initial_consts = consts,
            None => local_initial_consts = HashMap::new(),
        }

        match stack_size {
            Some(size) => local_stack_size = size,
            None => local_stack_size = DEFAULT_STACK_SIZE
        }

        VM{instruction_set, code: local_initial_code, stack: Stack::new(local_stack_size), const_pool: local_initial_consts, pc: 0, stop: false }
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
    pub fn load_bytecode(&mut self, bytecode: &mut [u8]) {
        let mut bytecode = bytecode;

        // check the signature at the top
        // so get the length in bytes of the signature
        let signature_bytes_size = size_of_val(BYTECODE_SIGNATURE);

        // and then we want to basically get all bytes that would encompass that in the bytecode
        let raw_signature_bytes_from_bc: &[u8] = &bytecode[0..signature_bytes_size];
        let signature_bytes_as_str = str::from_utf8(raw_signature_bytes_from_bc).unwrap();

        if signature_bytes_as_str != BYTECODE_SIGNATURE {
            panic!("unsupported bytecode signature: {}", signature_bytes_as_str);
        }

        bytecode = &mut bytecode[signature_bytes_size..]; // basically truncate it or wtv

        // and now we want to probably read constants, which has its own signature so check signature is there
        let const_signature_bytes_size = size_of_val(BYTECODE_CONSTS_SIGNATURE);
        let const_signature_bytes_from_bc: &[u8] = &bytecode[0..signature_bytes_size];
        let const_signature_bytes_as_str = str::from_utf8(const_signature_bytes_from_bc).unwrap();

        let mut got_instruction_signature = false;
        let mut cursor = 0;

        if const_signature_bytes_as_str == BYTECODE_CONSTS_SIGNATURE {
            bytecode = &mut bytecode[const_signature_bytes_size-1..]; // basically truncate it or wtv
            // assume constants exist bc we got a signature
            /*
            constant follows this pattern:
            type - data
            and therefore is this many bytes:
            1 byte - bytes based off of data type
             */

            // we kinda need a 'cursor' approach with this one
            // so every type byte where there's no match, check for instructions signature
            let mut const_count = 0;

            loop {
                if cursor >= bytecode.len() - 1 {
                    break;
                }

                let token = bytecode[cursor];

                match token {
                    1 => {
                        // int
                        // is OperandSize bytes
                        let int_bytes = &bytecode[cursor..size_of::<OperandSize>()];
                        self.const_pool.insert(const_count, Number(OperandSize::from_le_bytes(int_bytes.try_into().unwrap())));
                        cursor += size_of::<OperandSize>() + 1;
                        const_count += 1;
                    }
                    2 => {
                        // string
                        // has a dynamic amount of bytes
                        // string length is u32
                        let string_length_in_bytes = &bytecode[cursor + 1.. cursor + 1 + size_of::<u32>()];
                        let string_length = u32::from_le_bytes(string_length_in_bytes.try_into().unwrap());
                        let string_bytes = &bytecode[cursor + 1 + size_of::<u32>()..cursor + 1 + size_of::<u32>() + string_length as usize];
                        let string = str::from_utf8(&string_bytes);

                        if string.is_ok() {
                            self.const_pool.insert(const_count, Str(Rc::new(string.unwrap().to_string())));
                            cursor += 1 + size_of::<u32>() + string_length as usize;
                            const_count += 1;
                        } else {
                            panic!("couldn't decode string from {:?} with provided length {}", string_bytes, string_length);
                        }
                    }
                    3 => {
                        // boolean
                        // is 1 byte
                        let boolean_byte = bytecode[cursor + 1];
                        self.const_pool.insert(const_count, Value::Bool(boolean_byte != 0));
                        cursor += 2;
                        const_count += 1;
                    }
                    4 => {
                        // nil
                        // is 0 bytes
                        self.const_pool.insert(const_count, Value::Nil);
                        cursor += 1;
                        const_count += 1;
                    }
                    _ => {
                        // not specified so we check for instructions signature here
                        // also let's increment the cursor
                        let instr_signature_bytes_size = size_of_val(BYTECODE_INSTRUCTIONS_SIGNATURE);
                        let instr_signature_bytes_from_bc: &[u8] = &bytecode[cursor..instr_signature_bytes_size];
                        let instr_signature_bytes_as_str = str::from_utf8(instr_signature_bytes_from_bc).unwrap();

                        if instr_signature_bytes_as_str == BYTECODE_INSTRUCTIONS_SIGNATURE {
                            got_instruction_signature = true;
                            cursor += instr_signature_bytes_size;
                        }
                    }
                }
            }
        }

        bytecode = &mut bytecode[cursor..];

        // and now we probably want to check there's an instructions signature
        if !got_instruction_signature {
            let instr_signature_bytes_size = size_of_val(BYTECODE_INSTRUCTIONS_SIGNATURE);
            let instr_signature_bytes_from_bc: &[u8] = &bytecode[cursor..instr_signature_bytes_size];
            let instr_signature_bytes_as_str = str::from_utf8(instr_signature_bytes_from_bc).unwrap();

            if instr_signature_bytes_as_str == BYTECODE_INSTRUCTIONS_SIGNATURE {
                bytecode = &mut bytecode[instr_signature_bytes_size-1..];
                got_instruction_signature = true;
            } else {
                return;
            }
        }

        // below is code prior to the constant table, so to leave it untouched we reassign what bytecode is to not include constants
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

            // bc all instructions will always provide an operand as a number
            // and because we rely on the stack -> so much easier
            match operand {
                Some(operand) => {
                    (matching_instruction.unwrap().func)(self, Some(Value::Number(operand)));
                }
                None => {
                    (matching_instruction.unwrap().func)(self, None);
                }
            }
        }
    }

    // pops the topmost item off of the operand stack
    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    // pushes the supplied operand onto the operand stack
    pub fn push(&mut self, operand: Value) {
        self.stack.push(operand);
    }

    // peeks at the top of the operand stack
    pub fn peek(&mut self) -> Option<&Value> {
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

    // gets the reference to a value at specified key of the const pool
    pub fn get_const_ref(&self, key: OperandSize) -> Option<&Value> {
        self.const_pool.get(&(key as usize))
    }

    // gets the copy of a value at specified key of the const pool
    pub fn get_const_copy(&self, key: OperandSize) -> Option<Value> {
        if let Some(value) = self.const_pool.get(&(key as usize)) {
            Some(value.clone())
        } else {
            None
        }
    }

    // removes the value at specified key of the const pool
    pub fn remove_const(&mut self, key: OperandSize) {
        self.const_pool.remove(&(key as usize));
    }

    // stores the const provided and returns key value
    pub fn store_const(&mut self, value: Value) -> usize {
        // key should be length of hashmap + 1 (basically a counter)
        let key = self.const_pool.len() + 1usize;

        self.const_pool.insert(key, value);

        key
    }
}