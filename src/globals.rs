use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use crate::compiler::Code;
use crate::object::Object;
use crate::stdlib::*;
use crate::vm::Machine;

pub type InternalFunctionCallback = fn (&mut Machine, Vec<Object>) -> Object;

#[derive(Clone)]
pub struct InternalFunction {
    pub name: String,
    pub callback: InternalFunctionCallback,
}

impl Debug for InternalFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Globals {
    functions: HashMap<String, Function>,
    constants: HashMap<String, Object>,
}

impl Globals {
    pub fn new() -> Self {
        let mut s = Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };

        macro_rules! internal {
            ($name:ident) => {
                s.create_internal_function(stringify!($name), $name)
            };
        }

        // Type conversions and checkers.
        internal!(is_string);
        internal!(strval);

        // Filesystem.
        internal!(basename);

        // Misc.
        internal!(define);
        internal!(var_dump);

        // Arrays.
        internal!(count);

        s
    }

    pub fn create_constant(&mut self, name: String, value: Object) {
        self.constants.insert(name, value);
    }

    pub fn get_constant(&mut self, name: String) -> Option<Object> {
        self.constants.get(&name).cloned()
    }

    pub fn create_user_function(&mut self, name: String, instructions: Vec<Code>) {
        self.functions.insert(name, Function::User(instructions));
    }

    pub fn get_user_function(&mut self, name: String) -> Vec<Code> {
        match self.functions.get(&name) {
            Some(Function::User(code)) => code.to_vec(),
            _ => panic!("Cannot find user function for name {}", name),
        }
    }

    pub fn is_user_function(&mut self, name: impl Into<String>) -> bool {
        matches!(self.functions.get(&name.into()), Some(Function::User(..)))
    }

    pub fn create_internal_function(&mut self, name: impl Into<String> + Copy, callback: InternalFunctionCallback) {
        let internal = InternalFunction { name: name.into(), callback };

        self.functions.insert(name.into(), Function::Internal(internal));
    }

    pub fn get_internal_function(&mut self, name: impl Into<String>) -> InternalFunction {
        match self.functions.get(&name.into()) {
            Some(Function::Internal(i)) => i.clone(),
            _ => unreachable!()
        }
    }
}

pub enum Function {
    User(Vec<Code>),
    Internal(InternalFunction)
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Function::User(code) => format!("User({:?})", code),
            Function::Internal(InternalFunction { name, .. }) => format!("InternalFunction({})", name),
        })
    }
}