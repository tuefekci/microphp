use crate::parser::{Statement, Expression, Op};
use crate::object::Object;
use crate::globals::Globals;
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

struct Scope {
    instructions: Vec<Code>,
}

impl Scope {
    fn new() -> Self {
        Self { instructions: Vec::new() }
    }
}

struct Compiler {
    constants: Vec<Object>,
    scopes: Vec<Scope>,
    globals: Globals,
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
            Statement::Return(expression) => {
                if let Some(e) = expression {
                    self.expression(e);

                    self.emit(Code::ReturnWith);
                } else {
                    self.emit(Code::Return);
                }
            },
            Statement::Function(name, args, body) => {
                // We do this now so that any recursive function calls are aware of the function,
                // otherwise they'll try to do an internal function call.
                // TODO: Swap internal decision logic around so that it defaults to a user function.
                self.globals.create_user_function(name.clone(), Vec::new());
                
                self.enter_scope();

                for arg in args {
                    self.emit(Code::Assign(arg));
                    self.emit(Code::Pop);
                }

                for statement in body {
                    self.compile(statement);
                }

                self.constant(Object::Null);
                self.emit(Code::Return);

                let scope = self.leave_scope();

                self.globals.create_user_function(name, scope.instructions);
            },
            Statement::IfElse(condition, then, otherwise) => {
                self.expression(condition);

                let jump_if_not_position = self.emit(Code::JumpIfFalse(usize::MAX));

                for statement in then {
                    self.compile(statement);
                }

                if otherwise.is_empty() {
                    let after_then_position = self.len();

                    self.replace(jump_if_not_position, Code::JumpIfFalse(after_then_position));
                } else {
                    let jump_position = self.emit(Code::Jump(usize::MAX));

                    let after_then_position = self.len();
                    self.replace(jump_if_not_position, Code::JumpIfFalse(after_then_position));

                    for statement in otherwise {
                        self.compile(statement);
                    }

                    let after_otherwise_position = self.len();
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
                let then_start_position = self.len();

                self.breakable_scope = true;

                for statement in then {
                    self.compile(statement);
                }

                self.breakable_scope = false;

                let condition_position = self.len();

                self.expression(condition);

                self.emit(Code::JumpIfTrue(then_start_position));
                
                self.replace(condition_jump_position, Code::Jump(condition_position));

                let after_position = self.len();

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
            Expression::Identifier(i) => {
                self.emit(Code::GetConstant(i));
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
                                    Op::Concat => self.emit(Code::Concat),
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
            Expression::Call(callable, mut args) => {
                self.emit(Code::InitCall(callable.clone()));

                args.reverse();

                for arg in args {
                    self.expression(arg);
                    self.emit(Code::SendArg);
                }

                if self.globals.is_user_function(callable) {
                    self.emit(Code::DoUserCall);
                } else {
                    self.emit(Code::DoInternalCall);
                }
            },
            _ => todo!("{:?}", expression)
        }
    }

    fn len(&mut self) -> usize {
        self.scope().instructions.len()
    }

    fn emit(&mut self, code: Code) -> usize {
        self.scope().instructions.push(code);
        self.scope().instructions.len() - 1
    }

    fn replace(&mut self, position: usize, code: Code) {
        self.scope().instructions[position] = code
    }

    fn constant(&mut self, object: Object) {
        self.constants.push(object);
        self.emit(Code::Constant(self.constants.len() - 1));
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn leave_scope(&mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    fn scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}

pub fn compile(ast: Vec<Statement>) -> (Vec<Object>, Vec<Code>, Globals) {
    let ast = ast.into_iter();

    let scopes = vec![
        Scope::new(),
    ];

    let mut compiler = Compiler {
        constants: Vec::new(),
        scopes,
        globals: Globals::new(),
        breakable_scope: false,
        breakable_positions: Vec::new(),
    };

    for node in ast {
        compiler.compile(node);
    }

    let constants = compiler.constants.clone();
    let instructions = compiler.scope().instructions.clone();

    (constants, instructions, compiler.globals)
}