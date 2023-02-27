use std::fmt::{self, Write};
use std::iter;

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

enum Literal {
    Char(char),
    Int(u8),
    EmptyArray,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Char(c) => {
                if *c == '\n' {
                    write!(f, "'\\n'")
                } else if *c == '\'' {
                    write!(f, "'\\\''")
                } else if *c == '\\' {
                    write!(f, "'\\\\'")
                } else {
                    write!(f, "'{}'", c)
                }
            }
            Self::EmptyArray => write!(f, "ARRAY"),
        }
    }
}

enum Expr {
    Inc(Var),
    Dec(Var),
    ArrayAccess(Var, Var),
    IsEqualLiteral(Var, Literal),
    IsEqualVar(Var, Var),
    IsNotEqualLiteral(Var, Literal),
    Literal(Literal),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inc(v) => write!(f, "{} + 1", v),
            Self::Dec(v) => write!(f, "{} - 1", v),
            Self::ArrayAccess(array, idx) => write!(f, "{} : {}", array, idx),
            Self::IsEqualLiteral(v, l) => write!(f, "{} == {}", v, l),
            Self::IsNotEqualLiteral(v, l) => write!(f, "{} != {}", v, l),
            Self::IsEqualVar(v, v2) => write!(f, "{} == {}", v, v2),
            Self::Literal(l) => write!(f, "{}", l),
        }
    }
}

enum Function {
    ArrayReplace(Var, Var, Var),
    ArrayPush(Var, Var, Var),
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
            Self::ArrayPush(_, _, _) => "ArrayPush",
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
            Self::ArrayPush(a, b, c) => format!("{}, {}, {}", a, b, c),
            Self::ArrayPop(a, b) => format!("{}, {}", a, b),
            Self::CharToInt(v) => v.to_string(),
            Self::IntToChar(v) => v.to_string(),
            Self::PutChar(v) => v.to_string(),
            Self::ArrayLength(v) => v.to_string(),
            Self::ReadLine => "you".to_owned(),
        }
    }
}

enum Cmd {
    DeclareVar(Var),
    DeclareFn(Function),
    Return(Expr),
    DeclareChorus,
    Assign(Var, Expr),
    Call(Function, Var),
    CallNoReturn(Function),
    StartCond(Expr),
    EndIf,
    EndWhile,
}

#[derive(Debug)]
pub enum CompilerError {
    FormatError(fmt::Error),
    UnbalancedBrackets,
}

impl From<fmt::Error> for CompilerError {
    fn from(err: fmt::Error) -> Self {
        Self::FormatError(err)
    }
}

pub struct Compiler {
    cmds: Vec<Cmd>,
}

impl Compiler {
    pub fn read(program: &str) -> Compiler {
        let mut compiler = Self { cmds: Vec::new() };
        compiler.define_char_to_int();
        compiler.define_int_to_char();
        compiler.declare_chorus();
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
    pub fn output(&self, indent: i64, trace: bool) -> Result<String, CompilerError> {
        let mut res = String::new();
        let mut level = 0i64;
        let mut in_chorus = false;
        for (ln, cmd) in self.cmds.iter().enumerate() {
            match cmd {
                Cmd::EndIf | Cmd::EndWhile => {
                    if level == 0 {
                        return Err(CompilerError::UnbalancedBrackets);
                    } else {
                        level -= 1;
                    }
                }
                _ => {}
            }
            if trace && in_chorus {
                for _ in 0..level * indent {
                    write!(res, " ")?;
                }
                writeln!(res, "Never gonna say {}", ln)?;
            }
            for _ in 0..level * indent {
                write!(res, " ")?;
            }
            match cmd {
                Cmd::DeclareVar(v) => writeln!(res, "Never gonna let {} down", v)?,
                Cmd::DeclareFn(f) => {
                    writeln!(res, "[Verse {}]", f.name())?;
                    writeln!(res, "(Ooh give you {})", f.args())?;
                }
                Cmd::Return(e) => writeln!(
                    res,
                    "(Ooh) Never gonna give, never gonna give (give you {})",
                    e
                )?,
                Cmd::DeclareChorus => {
                    writeln!(res, "[Chorus]")?;
                    in_chorus = true
                }
                Cmd::Assign(v, e) => writeln!(res, "Never gonna give {} {}", v, e)?,
                Cmd::Call(f, v) => {
                    write!(res, "(Ooh give you {}) ", v)?;
                    writeln!(res, "Never gonna run {} and desert {}", f.name(), f.args())?;
                }
                Cmd::CallNoReturn(f) => {
                    writeln!(res, "Never gonna run {} and desert {}", f.name(), f.args())?
                }
                Cmd::StartCond(e) => {
                    writeln!(res, "Inside we both know {}", e)?;
                    level += 1;
                }
                Cmd::EndIf => {
                    writeln!(res, "Your heart's been aching but you're too shy to say it")?;
                }
                Cmd::EndWhile => {
                    writeln!(res, "We know the game and we're gonna play it")?;
                }
            }
        }
        if level == 0 {
            Ok(res)
        } else {
            Err(CompilerError::UnbalancedBrackets)
        }
    }
    fn define_char_to_int(&mut self) {
        self.cmds
            .push(Cmd::DeclareFn(Function::CharToInt(Var::Temp)));
        for c in iter::once('\n').chain(' '..='~') {
            self.cmds.push(Cmd::StartCond(Expr::IsEqualLiteral(
                Var::Temp,
                Literal::Char(c),
            )));
            self.cmds
                .push(Cmd::Return(Expr::Literal(Literal::Int(c as u8))));
            self.cmds.push(Cmd::EndIf);
        }
        self.cmds.push(Cmd::Return(Expr::Literal(Literal::Int(0))));
    }
    fn define_int_to_char(&mut self) {
        self.cmds
            .push(Cmd::DeclareFn(Function::IntToChar(Var::Temp)));
        for i in iter::once(b'\n').chain(b' '..=b'~') {
            self.cmds.push(Cmd::StartCond(Expr::IsEqualLiteral(
                Var::Temp,
                Literal::Int(i),
            )));
            self.cmds
                .push(Cmd::Return(Expr::Literal(Literal::Char(i as char))));
            self.cmds.push(Cmd::EndIf);
        }
        self.cmds
            .push(Cmd::Return(Expr::Literal(Literal::Char('$'))));
    }
    fn declare_chorus(&mut self) {
        self.cmds.push(Cmd::DeclareChorus);
    }
    fn init_vars(&mut self) {
        self.cmds.push(Cmd::DeclareVar(Var::Zero));
        self.cmds.push(Cmd::DeclareVar(Var::Tape));
        self.cmds.push(Cmd::DeclareVar(Var::Temp));
        self.cmds.push(Cmd::DeclareVar(Var::Buffer));
        self.cmds.push(Cmd::DeclareVar(Var::Pointer));
        self.cmds
            .push(Cmd::Assign(Var::Zero, Expr::Literal(Literal::Int(0))));
        self.cmds
            .push(Cmd::Assign(Var::Tape, Expr::Literal(Literal::EmptyArray)));
        self.cmds.push(Cmd::Call(
            Function::ArrayPush(Var::Tape, Var::Zero, Var::Zero),
            Var::Tape,
        ));
        self.cmds
            .push(Cmd::Assign(Var::Temp, Expr::Literal(Literal::Int(0))));
        self.cmds
            .push(Cmd::Assign(Var::Buffer, Expr::Literal(Literal::EmptyArray)));
        self.cmds
            .push(Cmd::Assign(Var::Pointer, Expr::Literal(Literal::Int(0))));
    }
    fn inc_pointer(&mut self) {
        self.cmds
            .push(Cmd::Assign(Var::Pointer, Expr::Inc(Var::Pointer)));
        self.cmds
            .push(Cmd::Call(Function::ArrayLength(Var::Tape), Var::Temp));
        self.cmds
            .push(Cmd::StartCond(Expr::IsEqualVar(Var::Pointer, Var::Temp)));
        self.cmds.push(Cmd::Call(
            Function::ArrayPush(Var::Tape, Var::Temp, Var::Zero),
            Var::Tape,
        ));
        self.cmds.push(Cmd::EndIf);
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
        self.cmds.push(Cmd::StartCond(Expr::IsEqualLiteral(
            Var::Temp,
            Literal::Int(0),
        )));
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
        self.cmds.push(Cmd::StartCond(Expr::IsNotEqualLiteral(
            Var::Temp,
            Literal::Int(0),
        )));
    }
    fn cond_jump_end(&mut self) {
        self.cmds.push(Cmd::EndWhile);
    }
}
