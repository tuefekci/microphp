use crate::object::Object;
use crate::compiler::Code;
use std::collections::HashMap;

struct Machine {
    constants: Vec<Object>,
    instructions: Vec<Code>,

    stack: Vec<Object>,
    globals: HashMap<String, Object>,
}

impl Machine {
    fn run(&mut self) {
        let mut ip: usize = 0;

        while ip < self.instructions.len() {
            let op = self.instructions.get(ip).unwrap();

            match op {
                Code::Constant(index) => {
                    let value = self.constants.get(*index).unwrap().clone();

                    self.stack.push(value);

                    ip += 1;
                },
                Code::Echo => {
                    let value = self.stack.pop().unwrap();

                    print!("{}", value);

                    ip += 1;
                },
                Code::Pop => {
                    self.stack.pop();

                    ip += 1;
                },
                Code::Assign(v) => {
                    let value = self.stack.pop().unwrap();

                    self.globals.insert(v.to_string(), value.clone());
                    self.stack.push(value);

                    ip += 1;
                },
                Code::Get(v) => {
                    let value = self.globals.get(v).unwrap().clone();

                    self.stack.push(value);

                    ip += 1;
                },
                Code::Add | Code::Subtract | Code::Divide | Code::Multiply => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    self.stack.push(match op {
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
                        _ => todo!("{:?}", op),
                    });

                    ip += 1;
                },
                _ => todo!("{:?}", op)
            }
        }
    }
}

pub fn run(constants: Vec<Object>, instructions: Vec<Code>) {
    let mut machine = Machine {
        constants,
        instructions,
        stack: Vec::new(),
        globals: HashMap::new(),
    };

    machine.run();
}