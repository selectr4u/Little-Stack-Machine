mod lsm;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dbg!(&args);

    if args.len() < 3 {
        eprintln!("usage: lsm --file <source file>/--string <string> ");
        std::process::exit(1);
    }

    // check if it's --file or --string
    match args[1].as_str() {
        "--file" => {
            // get entirety of file contents

            // send the contents into the vm
        }

        "--string" => {
            // we just send the input after --string into the vm
        }
        _ => {
            eprintln!("invalid argument");
        }
    }

}
