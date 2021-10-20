use crate::token::Token;
use std::slice::Iter;

#[derive(Debug)]
pub enum Statement {
    Echo(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    Infix(Box<Expression>, Op, Box<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    Variable(String),
}

struct Parser<'p> {
    tokens: Iter<'p, Token<'p>>,
    current: Token<'p>,
    peek: Token<'p>,
}

impl<'p> Parser<'p> {
    fn statement(&mut self) -> Statement {
        match self.current {
            Token::Echo => self.echo(),
            _ => {
                let expression = self.expression(0);

                self.semi();

                Statement::Expression(expression)
            }
        }
    }

    fn echo(&mut self) -> Statement {
        self.read();

        let expression = self.expression(0);

        self.semi();

        Statement::Echo(expression)
    }

    fn expression(&mut self, bp: u8) -> Expression {
        let mut lhs = match self.current {
            Token::String(s) => {
                self.read();

                let mut string = String::from(s);
                string.remove(0);
                string.pop();

                Expression::String(string)
            },
            Token::Integer(i) => {
                self.read();

                Expression::Integer(i)
            },
            Token::Float(f) => {
                self.read();

                Expression::Float(f)
            },
            Token::Variable(v) => {
                self.read();

                Expression::Variable(v.to_string())
            },
            _ => todo!("{:?}", self.current),
        };

        loop {
            if self.current == Token::Eof || self.current == Token::SemiColon {
                break;
            }

            if let Some((lbp, rbp)) = infix_binding_power(&self.current) {
                if lbp < bp {
                    break;
                }

                let op = self.current.clone();

                self.read();

                let rhs = self.expression(rbp);

                lhs = infix(lhs, &op, rhs);

                continue;
            }

            break;
        }

        lhs
    }

    fn read(&mut self) {
        self.current = std::mem::replace(&mut self.peek, if let Some(t) = self.tokens.next() { t.clone() } else { Token::Eof });
    }

    fn semi(&mut self) {
        self.expect(Token::SemiColon);
    }

    fn expect(&mut self, token: Token) {
        if std::mem::discriminant(&self.current) != std::mem::discriminant(&token) {
            eprintln!("Expected {:?}.", token);
            std::process::exit(0);
        }

        self.read();
    }

    fn next(&mut self) -> Option<Statement> {
        if self.current == Token::Eof {
            return None
        }

        Some(self.statement())
    }
}

fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
    Some(match token {
        Token::Multiply | Token::Divide => (13, 14),
        Token::Plus | Token::Minus => (11, 12),
        Token::Assign => (2, 1),
        _ => return None
    })
}

fn infix(lhs: Expression, op: &Token, rhs: Expression) -> Expression {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);

    match op {
        Token::Assign => Expression::Assign(lhs, rhs),
        _ => {
            Expression::Infix(lhs, match op {
                Token::Plus => Op::Add,
                Token::Minus => Op::Subtract,
                Token::Multiply => Op::Multiply,
                Token::Divide => Op::Divide,
                _ => todo!("infix op: {:?}", op),
            }, rhs)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    match tokens.first() {
        Some(Token::OpenTag) => (),
        _ => {
            eprintln!("Expected open-tag.");
            std::process::exit(1);
        },
    };

    let mut tokens = tokens.iter();
    tokens.next();

    let mut parser = Parser {
        tokens: tokens,
        current: Token::Eof,
        peek: Token::Eof,
    };

    parser.read();
    parser.read();

    let mut program = Vec::new();

    while let Some(n) = parser.next() {
        program.push(n);
    }

    program
}