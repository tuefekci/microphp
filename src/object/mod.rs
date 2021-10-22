use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Integer(i64),
    Float(f64),
    True,
    False,
    Null,
    Array(Rc<RefCell<HashMap<String, Object>>>),
}

pub fn new_array() -> Object {
    Object::Array(Rc::new(RefCell::new(HashMap::default())))
}

impl Object {
    pub fn is_string(&self) -> bool {
        matches!(self, Object::String(..))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Object::Array(..))
    }

    pub fn from_bool(b: bool) -> Self {
        match b {
            true => Self::True,
            false => Self::False
        }
    }
    
    pub fn to_bool(&self) -> bool {
        match self {
            Object::True => true,
            Object::False => false,
            Object::Float(f) => f > &0.0,
            Object::Integer(i) => i > &0,
            Object::String(s) => ! s.is_empty(),
            Object::Null => false,
            _ => todo!("to_bool: {:?}", self),
        }
    }

    pub fn to_hash(&self) -> &Rc<RefCell<HashMap<String, Object>>> {
        match self {
            Object::Array(items) => items,
            _ => unreachable!()
        }
    }

    pub fn dump(&self) -> String {
        match self {
            Object::Integer(i) => format!("int({})", i),
            Object::Float(f) => format!("double({})", f),
            Object::String(s) => format!("string({}) \"{}\"", s.len(), s),
            Object::True => format!("bool(true)"),
            Object::False => format!("bool(false)"),
            Object::Null => format!("NULL"),
            Object::Array(items) => {
                let items = items.borrow();
                let items = items.iter();
                let mut buffer: String = format!("array({}) {{\n", items.len());
                
                // TODO: Output keys here too.
                for (_, value) in items {
                    buffer.push_str(&format!("  {}\n", value.dump()));
                }

                buffer.push_str("}");
                buffer
            },
            _ => todo!()
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Object::String(s) => s.to_string(),
            Object::Integer(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::Null | Object::False => "".to_string(),
            Object::True => "1".to_string(),
            _ => todo!("Display: {:?}", self),
        })
    }
}