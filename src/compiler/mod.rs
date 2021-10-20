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
            _ => todo!()
        }
    };
}

struct Compiler {
    constants: Vec<Object>,
    instructions: Vec<Code>,
    breakable_scope: bool,
    breakable_positions: Vec<usize>,
}

impl Compiler {
    fn compile(&mut self, statement: Statement) {
        match statement {
            Statement::Echo(expression) => {
                self.expression(expression);
                self.emit(Code::Echo);
            },
            Statement::IfElse(condition, then, otherwise) => {
                self.expression(condition);

                let jump_if_not_position = self.emit(Code::JumpIfFalse(usize::MAX));

                for statement in then {
                    self.compile(statement);
                }

                if otherwise.is_empty() {
                    let after_then_position = self.instructions.len();

                    self.replace(jump_if_not_position, Code::JumpIfFalse(after_then_position));
                } else {
                    let jump_position = self.emit(Code::Jump(usize::MAX));

                    let after_then_position = self.instructions.len();
                    self.replace(jump_if_not_position, Code::JumpIfFalse(after_then_position));

                    for statement in otherwise {
                        self.compile(statement);
                    }

                    let after_otherwise_position = self.instructions.len();
                    self.replace(jump_position, Code::Jump(after_otherwise_position));
                }
            },
            Statement::Break => {
                if ! self.breakable_scope {
                    panic!("Invalid break scope.")
                }

                let position = self.emit(Code::Jump(usize::MAX));

                self.breakable_positions.push(position);
            },
            Statement::While(condition, then) => {
                let condition_jump_position = self.emit(Code::Jump(usize::MAX));
                let then_start_position = self.instructions.len();

                self.breakable_scope = true;

                for statement in then {
                    self.compile(statement);
                }

                self.breakable_scope = false;

                let condition_position = self.instructions.len();

                self.expression(condition);

                self.emit(Code::JumpIfTrue(then_start_position));
                
                self.replace(condition_jump_position, Code::Jump(condition_position));

                let after_position = self.instructions.len();

                for break_position in self.breakable_positions.clone() {
                    self.replace(break_position, Code::Jump(after_position));
                }

                self.breakable_positions = Vec::new();
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
            Expression::True => {
                self.emit(Code::True);
            },
            Expression::False => {
                self.emit(Code::False);
            },
            Expression::String(s) => {
                self.constant(Object::String(s));
            },
            Expression::Integer(i) => {
                self.constant(Object::Integer(i));
            },
            Expression::Float(f) => {
                self.constant(Object::Float(f));
            },
            Expression::Variable(v) => {
                self.emit(Code::Get(v));
            },
            Expression::Infix(lhs, op, rhs) => {
                let lhs = *lhs;
                let rhs = *rhs;

                match op {
                    Op::LessThan | Op::GreaterThan => {
                        self.expression(lhs);
                        self.expression(rhs);

                        match op {
                            Op::LessThan => self.emit(Code::LessThan),
                            Op::GreaterThan => self.emit(Code::GreaterThan),
                            _ => unreachable!()
                        };
                    },
                    _ => {
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
                                    _ => unreachable!()
                                };
                            },
                        };
                    }
                };
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

    fn emit(&mut self, code: Code) -> usize {
        self.instructions.push(code);
        self.instructions.len() - 1
    }

    fn replace(&mut self, position: usize, code: Code) {
        self.instructions[position] = code
    }

    fn constant(&mut self, object: Object) {
        self.constants.push(object);
        self.emit(Code::Constant(self.constants.len() - 1));
    }
}

pub fn compile(ast: Vec<Statement>) -> (Vec<Object>, Vec<Code>) {
    let mut ast = ast.into_iter();

    let mut compiler = Compiler {
        constants: Vec::new(),
        instructions: Vec::new(),
        breakable_scope: false,
        breakable_positions: Vec::new(),
    };

    while let Some(node) = ast.next() {
        compiler.compile(node);
    }

    (compiler.constants, compiler.instructions)
}