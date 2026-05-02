use std::collections::HashMap;
use std::process::exit;

#[derive(Clone)]
enum Op {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Op>),
}

impl Op {
    fn to_string(&self) -> String {
        match self {
            Op::None => "None".to_string(),
            Op::Bool(b) => b.to_string(),
            Op::Int(i) => i.to_string(),
            Op::Float(f) => f.to_string(),
            Op::String(s) => s.clone(),
            Op::List(l) => format!("[{}]", l.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")),
        }
    }
}

#[derive(Clone)]
enum Instruction {
    LoadConst(Op),
    LoadInt(i64),
    LoadFloat(f64),
    LoadString(String),
    StoreName(String),
    LoadName(String),
    BinaryAdd,
    BinarySubtract,
    BinaryMultiply,
    BinaryDivide,
    BinaryModulo,
    CompareOp(i32),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Jump(usize),
    Call(String, usize),
    Return,
}

struct VM {
    stack: Vec<Op>,
    locals: HashMap<String, Op>,
    code: Vec<Instruction>,
    pc: usize,
}

impl VM {
    fn new(code: Vec<Instruction>) -> Self {
        VM { stack: Vec::new(), locals: HashMap::new(), code, pc: 0 }
    }

    fn run(&mut self) -> Result<Op, String> {
        loop {
            if self.pc >= self.code.len() { return Ok(Op::None); }
            let instr = self.code[self.pc].clone();
            self.pc += 1;
            match instr {
                Instruction::LoadConst(v) => self.stack.push(v),
                Instruction::LoadInt(i) => self.stack.push(Op::Int(i)),
                Instruction::LoadFloat(f) => self.stack.push(Op::Float(f)),
                Instruction::LoadString(s) => self.stack.push(Op::String(s)),
                Instruction::StoreName(n) => { if let Some(v) = self.stack.pop() { self.locals.insert(n, v); } }
                Instruction::LoadName(n) => { if let Some(v) = self.locals.get(&n) { self.stack.push((*v).clone()); } }
                Instruction::BinaryAdd => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (a, b) { 
                        (Op::Int(i1), Op::Int(i2)) => Op::Int(i1.saturating_add(i2)), 
                        (Op::Float(f1), Op::Float(f2)) => Op::Float(f1 + f2), 
                        _ => Op::None 
                    });
                }
                Instruction::BinarySubtract => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (a, b) { (Op::Int(i1), Op::Int(i2)) => Op::Int(i1 - i2), (Op::Float(f1), Op::Float(f2)) => Op::Float(f1 - f2), _ => Op::None });
                }
                Instruction::BinaryMultiply => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (a, b) { (Op::Int(i1), Op::Int(i2)) => Op::Int(i1 * i2), (Op::Float(f1), Op::Float(f2)) => Op::Float(f1 * f2), _ => Op::None });
                }
                Instruction::BinaryDivide => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (a, b) { (Op::Int(i1), Op::Int(i2)) if i2 != 0 => Op::Float(i1 as f64 / i2 as f64), _ => Op::None });
                }
                Instruction::BinaryModulo => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (a, b) { (Op::Int(i1), Op::Int(i2)) => Op::Int(i1 % i2), _ => Op::None });
                }
                Instruction::CompareOp(op) => {
                    let b = self.stack.pop().ok_or("underflow")?;
                    let a = self.stack.pop().ok_or("underflow")?;
                    self.stack.push(match (&a, &b) { (Op::Int(i1), Op::Int(i2)) => match op { 0 => Op::Bool(i1 < i2), 1 => Op::Bool(i1 <= i2), 2 => Op::Bool(i1 == i2), 3 => Op::Bool(i1 != i2), 4 => Op::Bool(i1 > i2), 5 => Op::Bool(i1 >= i2), _ => Op::Bool(false) }, _ => Op::Bool(false) });
                }
                Instruction::JumpIfFalse(a) => { if let Some(&Op::Bool(false)) = self.stack.last() { self.pc = a; } }
                Instruction::JumpIfTrue(a) => { if let Some(v) = self.stack.last() { let t = matches!(v, Op::Bool(true)) || matches!(v, Op::Int(i) if *i != 0); if t { self.pc = a; } } }
                Instruction::Jump(a) => self.pc = a,
                Instruction::Call(f, n) => {
                    let mut args = Vec::new();
                    for _ in 0..n { args.push(self.stack.pop().ok_or("underflow")?); }
                    args.reverse();
                    match f.as_str() {
                        "print" => { for (i, a) in args.iter().enumerate() { if i > 0 { print!(" "); } print!("{}", a.to_string()); } println!(); }
                        "len" => self.stack.push(match args.get(0) { Some(Op::String(s)) => Op::Int(s.len() as i64), Some(Op::List(l)) => Op::Int(l.len() as i64), _ => Op::None }),
                        _ => {}
                    }
                }
                Instruction::Return => return Ok(self.stack.pop().unwrap_or(Op::None)),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { eprintln!("Usage: {} test_name", args[0]); exit(1); }
    let test = &args[1];
    let code: Vec<Instruction> = match test.as_str() {
        "hello" => vec![Instruction::LoadString("Hello, World!".to_string()), Instruction::Call("print".to_string(), 1), Instruction::Return],
        "add" => vec![Instruction::LoadInt(1), Instruction::LoadInt(2), Instruction::BinaryAdd, Instruction::Call("print".to_string(), 1), Instruction::Return],
        "loop" => vec![
            Instruction::LoadInt(0), Instruction::StoreName("i".to_string()),
            Instruction::LoadName("i".to_string()), Instruction::Call("print".to_string(), 1),
            Instruction::LoadName("i".to_string()), Instruction::LoadInt(1), Instruction::BinaryAdd, Instruction::StoreName("i".to_string()),
            Instruction::LoadName("i".to_string()), Instruction::LoadInt(5), Instruction::CompareOp(0),
            Instruction::JumpIfFalse(13),
            Instruction::Jump(2),
            Instruction::Return
        ],
        "list" => vec![Instruction::LoadInt(1), Instruction::LoadInt(2), Instruction::LoadInt(3), Instruction::LoadInt(4), Instruction::Call("list".to_string(), 4), Instruction::Call("print".to_string(), 1), Instruction::Return],
        "fib" => vec![
            Instruction::LoadInt(0), Instruction::StoreName("a".to_string()),
            Instruction::LoadInt(1), Instruction::StoreName("b".to_string()),
            Instruction::LoadInt(0), Instruction::StoreName("i".to_string()),
            Instruction::LoadName("a".to_string()), Instruction::Call("print".to_string(), 1),
            Instruction::LoadName("a".to_string()), Instruction::LoadName("b".to_string()), Instruction::BinaryAdd, Instruction::StoreName("c".to_string()),
            Instruction::LoadName("b".to_string()), Instruction::StoreName("a".to_string()),
            Instruction::LoadName("c".to_string()), Instruction::StoreName("b".to_string()),
            Instruction::LoadName("i".to_string()), Instruction::LoadInt(1), Instruction::BinaryAdd, Instruction::StoreName("i".to_string()),
            Instruction::LoadName("i".to_string()), Instruction::LoadInt(5), Instruction::CompareOp(0),
            Instruction::JumpIfFalse(24),
            Instruction::Jump(5),
            Instruction::Return
        ],
        "ifelse" => vec![Instruction::LoadInt(5), Instruction::StoreName("x".to_string()), Instruction::LoadName("x".to_string()), Instruction::LoadInt(3), Instruction::CompareOp(4), Instruction::JumpIfFalse(9), Instruction::LoadString("x > 3".to_string()), Instruction::Call("print".to_string(), 1), Instruction::Jump(11), Instruction::LoadString("x <= 3".to_string()), Instruction::Call("print".to_string(), 1), Instruction::Return],
        "arith" => vec![Instruction::LoadInt(10), Instruction::LoadInt(3), Instruction::BinaryAdd, Instruction::Call("print".to_string(), 1), Instruction::LoadInt(10), Instruction::LoadInt(3), Instruction::BinarySubtract, Instruction::Call("print".to_string(), 1), Instruction::LoadInt(10), Instruction::LoadInt(3), Instruction::BinaryMultiply, Instruction::Call("print".to_string(), 1), Instruction::LoadInt(10), Instruction::LoadInt(3), Instruction::BinaryDivide, Instruction::Call("print".to_string(), 1), Instruction::LoadInt(10), Instruction::LoadInt(3), Instruction::BinaryModulo, Instruction::Call("print".to_string(), 1), Instruction::Return],
        _ => { eprintln!("Unknown: {}", test); exit(1); }
    };
    let mut vm = VM::new(code);
    match vm.run() { Ok(r) => println!("Result: {}", r.to_string()), Err(e) => { eprintln!("Error: {}", e); exit(1); } }
}