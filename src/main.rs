mod compiler;

use crate::compiler::Compiler;
use clap::Parser;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of spaces per indentation level
    #[arg(long, default_value_t = 2)]
    indent: i64,

    /// Insert debugging trace statements in Rickroll output
    #[arg(long)]
    trace: bool,

    /// Name of output Rickroll file
    #[arg(short)]
    output: String,

    /// Name of input Brainfuck file
    #[arg()]
    file: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(bf) = fs::read_to_string(&args.file) {
        let compiler = Compiler::read(&bf);
        match compiler.output(args.indent, args.trace) {
            Ok(output) => {
                if let Err(_) = fs::write(&args.output, output) {
                    eprintln!("Unable to write to file \"{}\"", args.output);
                }
            }
            Err(err) => eprintln!("error: {:?}", err),
        }
    } else {
        eprintln!("Unable to read file \"{}\"", args.file);
        process::exit(1);
    }
}
