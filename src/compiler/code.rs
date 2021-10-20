#[derive(Debug, Clone)]
pub enum Code {
    Constant(usize),
    Echo,
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign(String),
    Get(String),
    Jump(usize),
    JumpIfFalse(usize),
    Pop,
    True,
    False,
}