use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i64),
    Float(f64),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Object::String(s) => s.to_string(),
            Object::Integer(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            _ => todo!("Display: {:?}", self),
        })
    }
}