#[derive(Debug, Clone)]
pub enum Code {
    Constant(usize),
    Echo,
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Assign(String),
    Get(String),
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Pop,
    True,
    False,
}