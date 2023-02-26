use std::env;
use std::fmt::{self, Write};
use std::fs::read_to_string;
use std::process;

enum Var {
    Zero,
    Pointer,
    Tape,
    Temp,
    Buffer,
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pointer => "Pointer",
                Self::Tape => "Tape",
                Self::Temp => "Temp",
                Self::Buffer => "Buffer",
                Self::Zero => "Zero",
            }
        )
    }
}

enum Expr {
    Inc(Var),
    Dec(Var),
    ArrayAccess(Var, Var),
    IsZero(Var),
    Zero,
    EmptyArray,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inc(v) => write!(f, "{} + 1", v),
            Self::Dec(v) => write!(f, "{} - 1", v),
            Self::ArrayAccess(array, idx) => write!(f, "{} : {}", array, idx),
            Self::IsZero(v) => write!(f, "{} == 0", v),
            Self::Zero => write!(f, "0"),
            Self::EmptyArray => write!(f, "ARRAY"),
        }
    }
}

enum Function {
    ArrayReplace(Var, Var, Var),
    ArrayPop(Var, Var),
    ArrayLength(Var),
    CharToInt(Var),
    IntToChar(Var),
    PutChar(Var),
    ReadLine,
}

impl Function {
    fn name(&self) -> &str {
        match self {
            Self::ArrayReplace(_, _, _) => "ArrayReplace",
            Self::ArrayPop(_, _) => "ArrayPop",
            Self::CharToInt(_) => "CharToInt",
            Self::IntToChar(_) => "IntToChar",
            Self::PutChar(_) => "PutChar",
            Self::ArrayLength(_) => "ArrayLength",
            Self::ReadLine => "ReadLine",
        }
    }
    fn args(&self) -> String {
        match self {
            Self::ArrayReplace(a, b, c) => format!("{}, {}, {}", a, b, c),
            Self::ArrayPop(a, b) => format!("{}, {}", a, b),
            Self::CharToInt(v) => v.to_string(),
            Self::IntToChar(v) => v.to_string(),
            Self::PutChar(v) => v.to_string(),
            Self::ArrayLength(v) => v.to_string(),
            Self::ReadLine => "hello".to_owned(),
        }
    }
}

enum Cmd {
    Declare(Var),
    Assign(Var, Expr),
    Call(Function, Var),
    CallNoReturn(Function),
    If(Expr),
    EndIf,
}

#[derive(Debug)]
enum CompilerError {
    FormatError(fmt::Error),
}

impl From<fmt::Error> for CompilerError {
    fn from(err: fmt::Error) -> Self {
        Self::FormatError(err)
    }
}

struct Compiler {
    cmds: Vec<Cmd>,
}

impl Compiler {
    fn read(program: &str) -> Compiler {
        let mut compiler = Self { cmds: Vec::new() };
        compiler.init_vars();
        for c in program.chars() {
            match c {
                '>' => compiler.inc_pointer(),
                '<' => compiler.dec_pointer(),
                '+' => compiler.inc_data(),
                '-' => compiler.dec_data(),
                '.' => compiler.output_byte(),
                ',' => compiler.read_byte(),
                '[' => compiler.cond_jump(),
                ']' => compiler.cond_jump_end(),
                _ => {}
            };
        }
        compiler
    }
    fn output(&self) -> Result<String, CompilerError> {
        let mut res = String::new();
        writeln!(res, "[Chorus]")?;
        for cmd in &self.cmds {
            match cmd {
                Cmd::Declare(v) => writeln!(res, "Never gonna let {} down", v)?,
                Cmd::Assign(v, e) => writeln!(res, "Never gonna give {} {}", v, e)?,
                Cmd::Call(f, v) => {
                    write!(res, "(Ooh give you {}) ", v)?;
                    writeln!(res, "Never gonna run {} and desert {}", f.name(), f.args())?;
                }
                Cmd::CallNoReturn(f) => {
                    writeln!(res, "Never gonna run {} and desert {}", f.name(), f.args())?
                }
                Cmd::If(e) => writeln!(res, "Inside we both know {}", e)?,
                Cmd::EndIf => writeln!(res, "We know the game and we're gonna play it")?,
            }
        }
        Ok(res)
    }
    fn init_vars(&mut self) {
        self.cmds.push(Cmd::Declare(Var::Zero));
        self.cmds.push(Cmd::Declare(Var::Tape));
        self.cmds.push(Cmd::Declare(Var::Temp));
        self.cmds.push(Cmd::Declare(Var::Buffer));
        self.cmds.push(Cmd::Declare(Var::Pointer));
        self.cmds.push(Cmd::Assign(Var::Zero, Expr::Zero));
        self.cmds.push(Cmd::Assign(Var::Tape, Expr::EmptyArray));
        self.cmds.push(Cmd::Assign(Var::Temp, Expr::Zero));
        self.cmds.push(Cmd::Assign(Var::Buffer, Expr::EmptyArray));
        self.cmds.push(Cmd::Assign(Var::Pointer, Expr::Zero));
    }
    fn inc_pointer(&mut self) {
        self.cmds
            .push(Cmd::Assign(Var::Pointer, Expr::Inc(Var::Pointer)));
    }
    fn dec_pointer(&mut self) {
        self.cmds
            .push(Cmd::Assign(Var::Pointer, Expr::Dec(Var::Pointer)));
    }
    fn inc_data(&mut self) {
        self.cmds.push(Cmd::Assign(
            Var::Temp,
            Expr::ArrayAccess(Var::Tape, Var::Pointer),
        ));
        self.cmds.push(Cmd::Assign(Var::Temp, Expr::Inc(Var::Temp)));
        self.cmds.push(Cmd::Call(
            Function::ArrayReplace(Var::Tape, Var::Pointer, Var::Temp),
            Var::Tape,
        ));
    }
    fn dec_data(&mut self) {
        self.cmds.push(Cmd::Assign(
            Var::Temp,
            Expr::ArrayAccess(Var::Tape, Var::Pointer),
        ));
        self.cmds.push(Cmd::Assign(Var::Temp, Expr::Dec(Var::Temp)));
        self.cmds.push(Cmd::Call(
            Function::ArrayReplace(Var::Tape, Var::Pointer, Var::Temp),
            Var::Tape,
        ));
    }
    fn output_byte(&mut self) {
        self.cmds.push(Cmd::Assign(
            Var::Temp,
            Expr::ArrayAccess(Var::Tape, Var::Pointer),
        ));
        self.cmds
            .push(Cmd::Call(Function::IntToChar(Var::Temp), Var::Temp));
        self.cmds
            .push(Cmd::CallNoReturn(Function::PutChar(Var::Temp)));
    }
    fn read_byte(&mut self) {
        self.cmds
            .push(Cmd::Call(Function::ArrayLength(Var::Buffer), Var::Temp));
        self.cmds.push(Cmd::If(Expr::IsZero(Var::Temp)));
        self.cmds.push(Cmd::Call(Function::ReadLine, Var::Buffer));
        self.cmds.push(Cmd::EndIf);
        self.cmds.push(Cmd::Assign(
            Var::Temp,
            Expr::ArrayAccess(Var::Buffer, Var::Zero),
        ));
        self.cmds.push(Cmd::Call(
            Function::ArrayPop(Var::Buffer, Var::Zero),
            Var::Buffer,
        ));
        self.cmds
            .push(Cmd::Call(Function::CharToInt(Var::Temp), Var::Temp));
        self.cmds.push(Cmd::Call(
            Function::ArrayReplace(Var::Tape, Var::Pointer, Var::Temp),
            Var::Tape,
        ));
    }
    fn cond_jump(&mut self) {
        self.cmds.push(Cmd::Assign(
            Var::Temp,
            Expr::ArrayAccess(Var::Tape, Var::Pointer),
        ));
        self.cmds.push(Cmd::If(Expr::IsZero(Var::Temp)));
    }
    fn cond_jump_end(&mut self) {
        self.cmds.push(Cmd::EndIf);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let print_help = || eprintln!("Usage: {} <file>", args[0]);
    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            print_help();
        } else {
            if let Ok(bf) = read_to_string(&args[1]) {
                let compiler = Compiler::read(&bf);
                match compiler.output() {
                    Ok(output) => println!("{}", output),
                    Err(err) => eprintln!("{:?}", err),
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
