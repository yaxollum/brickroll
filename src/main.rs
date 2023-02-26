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
                println!("{}", bf);
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
