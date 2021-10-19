use crate::token::Token;
use std::slice::Iter;

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Echo(Expression),
}

#[derive(Debug)]
pub enum Expression {
    String(String),
}

struct Parser<'p> {
    tokens: Iter<'p, Token<'p>>,
    current: Token<'p>,
    peek: Token<'p>,
}

impl<'p> Parser<'p> {
    fn statement(&mut self) -> Node {
        Node::Statement(match self.current {
            Token::Echo => self.echo(),
            _ => todo!("{:?}", self.current)
        })
    }

    fn echo(&mut self) -> Statement {
        self.read();

        let expression = self.expression(0);

        self.semi();

        Statement::Echo(expression)
    }

    fn expression(&mut self, bp: u8) -> Expression {
        match self.current {
            Token::String(s) => {
                self.read();

                let mut string = String::from(s);
                string.remove(0);
                string.pop();

                Expression::String(string)
            },
            _ => todo!()
        }
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

    fn next(&mut self) -> Option<Node> {
        if self.current == Token::Eof {
            return None
        }

        Some(self.statement())
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
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