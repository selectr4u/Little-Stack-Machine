mod lsm;

use std::env;
use crate::lsm::{Instruction, RawInstruction, DEFAULT_INSTRUCTION_SET, VM};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dbg!(&args);

    if args.len() < 3 {
        eprintln!("usage: lsm --file <source file>/--string <string> ");
        std::process::exit(1);
    }

    let instruction_set = DEFAULT_INSTRUCTION_SET.to_vec();
    let mut vm = VM::new(instruction_set, None, None);

    // check if it's --file or --string
    match args[1].as_str() {
        "--file" => {
            // get entirety of file contents

            // send the contents into the vm
        }

        "--string" => {
            // we just send the input after --string into the vm
            let string = &args[2..];

            vm.load_bytecode(string.to_string());

            vm.run();
        }
        _ => {
            eprintln!("invalid argument");
        }
    }

}
