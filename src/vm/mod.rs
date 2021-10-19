use crate::object::Object;
use crate::compiler::Code;

struct Machine {
    constants: Vec<Object>,
    instructions: Vec<Code>,

    stack: Vec<Object>,
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
                }
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
    };

    machine.run();
}