use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Object::String(s) => s,
            _ => todo!("Display: {:?}", self),
        })
    }
}