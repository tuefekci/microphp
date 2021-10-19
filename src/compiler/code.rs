#[derive(Debug, Clone)]
pub enum Code {
    Constant(usize),
    Echo,
    Add,
    Subtract,
    Multiply,
    Divide,
}