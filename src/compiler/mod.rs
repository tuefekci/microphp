use crate::parser::{Statement, Expression};
use crate::object::Object;
use std::slice::Iter;
use code::Code;

mod code;

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
            _ => todo!("{:?}", statement)
        }
    }

    fn expression(&mut self, expression: Expression) {
        match expression {
            Expression::String(s) => {
                self.constant(Object::String(s))
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