mod compiler;

use crate::compiler::Compiler;
use std::env;
use std::fs::read_to_string;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let print_help = || eprintln!("Usage: {} <file>", args[0]);
    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            print_help();
        } else {
            if let Ok(bf) = read_to_string(&args[1]) {
                let compiler = Compiler::read(&bf);
                match compiler.output(2, false) {
                    Ok(output) => println!("{}", output),
                    Err(err) => eprintln!("error: {:?}", err),
                }
            } else {
                eprintln!("Unable to read file \"{}\"", args[1]);
                process::exit(1);
            }
        }
    } else {
        print_help();
        process::exit(1);
    }
}
