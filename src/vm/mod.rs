use crate::object::{Object, new_array};
use crate::compiler::Code;
use crate::globals::{Globals, InternalFunction};
use std::collections::HashMap;

#[derive(Debug)]
struct Frame {
    ip: usize,
    instructions: Vec<Code>,
    environment: HashMap<String, Object>,
    stack: Vec<Object>,
    internal: Option<InternalFunction>,
}

impl Frame {
    fn new(instructions: Vec<Code>) -> Self {
        Self {
            ip: 0,
            instructions,
            environment: HashMap::new(),
            stack: Vec::new(),
            internal: None,
        }
    }

    fn internal(internal: InternalFunction) -> Self {
        Self {
            ip: 0,
            instructions: Vec::new(),
            environment: HashMap::new(),
            stack: Vec::new(),
            internal: Some(internal),
        }
    }

    fn set(&mut self, name: String, value: Object) {
        self.environment.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Option<&Object> {
        self.environment.get(name)
    }

    fn push(&mut self, value: Object) {
        self.stack.push(value);
    }
}

pub struct Machine {
    constants: Vec<Object>,
    frames: Vec<Frame>,
    buffer: Vec<Frame>,

    pub globals: Globals,
}

impl Machine {
    fn frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn buffer(&mut self) -> &mut Frame {
        self.buffer.last_mut().unwrap()
    }

    fn push(&mut self, value: Object) {
        self.frame().stack.push(value)
    }

    fn pop(&mut self) -> Option<Object> {
        self.frame().stack.pop()
    }

    fn next(&mut self) {
        self.frame().ip += 1;
    }

    fn run(&mut self) {
        while self.frame().ip < self.frame().instructions.len() {
            let ip = self.frame().ip;
            let op = self.frame().instructions.get(ip).unwrap().clone();

            match op {
                Code::Constant(index) => {
                    let value = self.constants.get(index).unwrap().clone();

                    self.push(value);

                    self.next();
                },
                Code::True => {
                    self.push(Object::True);
                    self.next();
                },
                Code::False => {
                    self.push(Object::False);
                    self.next();
                },
                Code::Jump(position) => {
                    self.frame().ip = position
                },
                Code::JumpIfFalse(position) => {
                    let value = self.pop().unwrap();

                    if ! value.to_bool() {
                        self.frame().ip = position
                    } else {
                        self.next();
                    }
                },
                Code::JumpIfTrue(position) => {
                    let value = self.pop().unwrap();

                    if value.to_bool() {
                        self.frame().ip = position;
                    } else {
                        self.next();
                    }
                }
                Code::Echo => {
                    let value = self.pop().unwrap();

                    print!("{}", value);

                    self.next();
                },
                Code::Pop => {
                    self.pop();

                    self.next();
                },
                Code::Assign(v) => {
                    let value = self.pop().unwrap();

                    self.frame().set(v.to_string(), value.clone());
                    self.push(value);

                    self.next();
                },
                Code::Get(v) => {
                    let value = self.frame().get(&v).unwrap().clone();

                    self.push(value);

                    self.next();
                },
                Code::GetConstant(c) => {
                    match self.globals.get_constant(c.clone()) {
                        Some(o) => self.push(o),
                        _ => panic!("Undefined constant {}", c),
                    };

                    self.next();
                },
                Code::Add | Code::Subtract | Code::Divide | Code::Multiply |
                Code::LessThan | Code::GreaterThan | Code::Concat => {
                    let rhs = self.pop().unwrap();
                    let lhs = self.pop().unwrap();

                    self.push(match op {
                        Code::Add => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
                            (Object::Float(l), Object::Integer(r)) => Object::Float(l + r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::Float(l as f64 + r),
                            (Object::Float(l), Object::Float(r)) => Object::Float(l + r),
                            _ => unreachable!()
                        },
                        Code::Subtract => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l - r),
                            (Object::Float(l), Object::Integer(r)) => Object::Float(l - r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::Float(l as f64 - r),
                            (Object::Float(l), Object::Float(r)) => Object::Float(l - r),
                            _ => unreachable!()
                        },
                        Code::Multiply => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::Integer(l * r),
                            (Object::Float(l), Object::Integer(r)) => Object::Float(l * r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::Float(l as f64 * r),
                            (Object::Float(l), Object::Float(r)) => Object::Float(l * r),
                            _ => unreachable!()
                        },
                        Code::Divide => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::Float(l as f64 / r as f64),
                            (Object::Float(l), Object::Integer(r)) => Object::Float(l / r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::Float(l as f64 / r),
                            (Object::Float(l), Object::Float(r)) => Object::Float(l / r),
                            _ => unreachable!()
                        },
                        Code::GreaterThan => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::from_bool(l > r),
                            (Object::Float(l), Object::Integer(r)) => Object::from_bool(l > r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::from_bool(l as f64 > r),
                            (Object::Float(l), Object::Float(r)) => Object::from_bool(l > r),
                            _ => unreachable!()
                        },
                        Code::LessThan => match (lhs, rhs) {
                            (Object::Integer(l), Object::Integer(r)) => Object::from_bool(l < r),
                            (Object::Float(l), Object::Integer(r)) => Object::from_bool(l < r as f64),
                            (Object::Integer(l), Object::Float(r)) => Object::from_bool((l as f64) < r),
                            (Object::Float(l), Object::Float(r)) => Object::from_bool(l < r),
                            _ => unreachable!()
                        },
                        Code::Concat => {
                            Object::String(format!("{}{}", lhs, rhs))
                        },
                        _ => todo!("{:?}", op),
                    });

                    self.next();
                },
                Code::InitCall(callable) => {
                    let frame = if self.globals.is_user_function(&callable) {
                        let instructions = self.globals.get_user_function(callable);
                        Frame::new(instructions)
                    } else {
                        Frame::internal(self.globals.get_internal_function(&callable))
                    };

                    self.push_buffer(frame);
                    self.next();
                },
                Code::DoUserCall => {
                    let frame = self.pop_buffer();

                    self.next();
                    self.push_frame(frame);
                },
                Code::DoInternalCall => {
                    let frame = self.pop_buffer();
                    
                    let internal = frame.internal.unwrap();
                    let mut args = frame.stack;
                    args.reverse();
                    let callback = internal.callback;
                    
                    let result = callback(self, args);

                    self.push(result);

                    self.next();
                },
                Code::SendArg => {
                    let value = self.pop().unwrap();

                    self.buffer().push(value);
                    self.next();
                },
                Code::Return => {            
                    self.pop_frame();

                    self.push(Object::Null);
                },
                Code::ReturnWith => {
                    let value = self.pop().unwrap();

                    self.pop_frame();

                    self.push(value);
                },
                Code::InitArray => {
                    self.push(new_array());
                    self.next();
                },
                Code::AddToArray => {
                    let value = self.pop().unwrap();
                    let array = self.pop();
                    
                    match array.clone() {
                        Some(Object::Array(items)) => {
                            let len = items.borrow().len();

                            items.borrow_mut().insert(len.to_string(), value);
                            
                            self.push(array.unwrap());
                        },
                        _ => unreachable!()
                    };

                    self.next();
                },
                Code::GetArrayItem => {
                    let index = self.pop().unwrap();
                    let array = self.pop();

                    match array {
                        Some(Object::Array(items)) => {
                            let hash = items.borrow();
                            let value = hash.get(&index.to_string()).unwrap();

                            self.push(value.clone());
                        },
                        _ => unreachable!()
                    };

                    self.next();
                }
                _ => todo!("{:?}", op)
            }
        }
    }

    fn push_buffer(&mut self, frame: Frame) {
        self.buffer.push(frame)
    }

    fn pop_buffer(&mut self) -> Frame {
        self.buffer.pop().unwrap()
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }
}

pub fn run(constants: Vec<Object>, instructions: Vec<Code>, globals: Globals) {
    let frames = vec![
        Frame::new(instructions),
    ];

    let mut machine = Machine { constants, frames, buffer: Vec::new(), globals };

    machine.run();
}