use crate::parser::{Statement, Expression, Op};
use crate::object::Object;
pub use code::Code;

mod code;

macro_rules! binary_op {
    ($lhs:expr, $op:expr, $rhs:expr) => {
        match $op {
            Op::Add => $lhs + $rhs,
            Op::Subtract => $lhs - $rhs,
            Op::Multiply => $lhs * $rhs,
            Op::Divide => $lhs / $rhs,
        }
    };
}

struct Compiler {
    constants: Vec<Object>,
    instructions: Vec<Code>,
}

impl Compiler {
    fn compile(&mut self, statement: Statement) {
        match statement {
            Statement::Echo(expression) => {
                self.expression(expression);
                self.emit(Code::Echo);
            },
            Statement::Expression(expression) => {
                self.expression(expression);
                self.emit(Code::Pop);
            },
            _ => todo!("{:?}", statement)
        }
    }

    fn expression(&mut self, expression: Expression) {
        match expression {
            Expression::String(s) => {
                self.constant(Object::String(s))
            },
            Expression::Integer(i) => {
                self.constant(Object::Integer(i))
            },
            Expression::Float(f) => {
                self.constant(Object::Float(f))
            },
            Expression::Variable(v) => {
                self.emit(Code::Get(v))
            },
            Expression::Infix(lhs, op, rhs) => {
                let lhs = *lhs;
                let rhs = *rhs;

                match (lhs.clone(), rhs.clone()) {
                    (Expression::Integer(l), Expression::Integer(r)) => {
                        if op == Op::Divide {
                            self.constant(Object::Float(binary_op!(l as f64, op, r as f64)))
                        } else {
                            self.constant(Object::Integer(binary_op!(l, op, r)))
                        }
                    },
                    (Expression::Float(l), Expression::Integer(r)) => {
                        self.constant(Object::Float(binary_op!(l, op, r as f64)))
                    },
                    (Expression::Integer(l), Expression::Float(r)) => {
                        self.constant(Object::Float(binary_op!(l as f64, op, r)))
                    },
                    (Expression::Float(l), Expression::Float(r)) => {
                        self.constant(Object::Float(binary_op!(l, op, r)))
                    },
                    _ => {
                        self.expression(lhs);
                        self.expression(rhs);

                        match op {
                            Op::Add => self.emit(Code::Add),
                            Op::Subtract => self.emit(Code::Subtract),
                            Op::Multiply => self.emit(Code::Multiply),
                            Op::Divide => self.emit(Code::Divide),
                        }
                    },
                }
            },
            Expression::Assign(target, value) => {
                self.expression(*value);

                match *target {
                    Expression::Variable(v) => self.emit(Code::Assign(v)),
                    _ => unreachable!("Assign to: {:?}", target),
                };
            },
            _ => todo!("{:?}", expression)
        }
    }

    fn emit(&mut self, code: Code) {
        self.instructions.push(code);
    }

    fn constant(&mut self, object: Object) {
        self.constants.push(object);
        self.emit(Code::Constant(self.constants.len() - 1))
    }
}

pub fn compile(ast: Vec<Statement>) -> (Vec<Object>, Vec<Code>) {
    let mut ast = ast.into_iter();

    let mut compiler = Compiler {
        constants: Vec::new(),
        instructions: Vec::new(),
    };

    while let Some(node) = ast.next() {
        compiler.compile(node);
    }

    (compiler.constants, compiler.instructions)
}