use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i64),
    Float(f64),
    True,
    False,
}

impl Object {
    pub fn to_bool(&self) -> bool {
        match self {
            Object::True => true,
            Object::False => false,
            Object::Float(f) => f > &0.0,
            Object::Integer(i) => i > &0,
            Object::String(s) => ! s.is_empty(),
            _ => todo!("to_bool: {:?}", self),
        }
    }
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