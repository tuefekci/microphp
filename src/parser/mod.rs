use crate::token::Token;
use std::slice::Iter;

#[derive(Debug)]
pub enum Statement {
    Echo(Expression),
    Expression(Expression),
    IfElse(Expression, Vec<Statement>, Vec<Statement>),
    // <test>, <body>
    While(Expression, Vec<Statement>),
    // <init>, <test>, <increment>, <body>
    For(Option<Expression>, Option<Expression>, Option<Expression>, Vec<Statement>),
    Function(String, Vec<String>, Vec<Statement>),
    Return(Option<Expression>),
    Const(String, Expression),
    Break,
}

#[derive(Debug, Clone)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    True,
    False,
    Null,
    Array(Vec<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Assign(Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
    Variable(String),
    Identifier(String),
    Index(Box<Expression>, Box<Expression>),
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
            Token::If => self.r#if(),
            Token::While => self.r#while(),
            Token::For => self.r#for(),
            Token::Function => self.function(),
            Token::Const => {
                self.read();

                let name = self.identifier();

                self.expect(Token::Assign);

                let value = self.expression(0);

                self.semi();

                Statement::Const(name, value)
            },
            Token::Return => {
                self.read();

                if self.current == Token::SemiColon {
                    self.semi();
                    
                    Statement::Return(None)
                } else {
                    let expression = self.expression(0);
                    self.semi();
                    Statement::Return(Some(expression))
                }
            },
            Token::Break => {
                self.read();
                self.semi();
                
                Statement::Break
            },
            _ => {
                let expression = self.expression(0);

                self.semi();

                Statement::Expression(expression)
            }
        }
    }

    fn function(&mut self) -> Statement {
        self.read();

        let identifier = self.identifier();

        self.expect(Token::LeftParen);
        
        let mut args = Vec::new();

        while self.current != Token::RightParen {
            args.push(match self.current {
                Token::Variable(i) => {
                    self.read();

                    i.to_string()
                },
                Token::Comma => {
                    self.read();
                    continue;
                },
                _ => unreachable!()
            });
        }

        // TODO: Add parameter parsing here.
        self.expect(Token::RightParen);

        let body = self.block();

        Statement::Function(identifier, args, body)
    }

    fn identifier(&mut self) -> String {
        match self.current {
            Token::Identifier(i) => {
                self.read();
                
                i.to_string()
            },
            _ => panic!("Expected identifier"),
        }
    }

    fn echo(&mut self) -> Statement {
        self.read();

        let expression = self.expression(0);

        self.semi();

        Statement::Echo(expression)
    }

    fn r#if(&mut self) -> Statement {
        self.read();

        self.expect(Token::LeftParen);
        let condition = self.expression(0);
        self.expect(Token::RightParen);

        let then = self.block();

        let mut otherwise = Vec::new();

        if self.current == Token::Else {
            self.read();

            otherwise = self.block();
        }

        Statement::IfElse(condition, then, otherwise)
    }

    fn r#while(&mut self) -> Statement {
        self.read();

        self.expect(Token::LeftParen);
        let condition = self.expression(0);
        self.expect(Token::RightParen);

        let then = self.block();

        Statement::While(condition, then)
    }

    fn r#for(&mut self) -> Statement {
        self.read();

        self.expect(Token::LeftParen);

        let init = if self.current == Token::SemiColon {
            self.semi();
            
            None
        } else {
            let expression = self.expression(0);
            
            self.semi();

            Some(expression)
        };

        let test = if self.current == Token::SemiColon {
            self.semi();

            None
        } else {
            let expression = self.expression(0);
            
            self.semi();

            Some(expression)
        };

        let increment = if self.current == Token::RightParen {
            None
        } else {
            let expression = self.expression(0);

            Some(expression)
        };

        self.expect(Token::RightParen);

        let then = self.block();

        Statement::For(init, test, increment, then)
    }

    fn block(&mut self) -> Vec<Statement> {
        self.expect(Token::LeftBrace);

        let mut block = Vec::new();

        while self.current != Token::RightBrace {
            block.push(self.statement());
        }

        self.expect(Token::RightBrace);

        block
    }

    fn expression(&mut self, bp: u8) -> Expression {
        let mut lhs = match self.current {
            Token::Null => {
                self.read();

                Expression::Null
            }
            Token::True => {
                self.read();

                Expression::True
            },
            Token::False => {
                self.read();

                Expression::False
            },
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
            Token::Identifier(i) => {
                self.read();

                Expression::Identifier(i.to_string())
            },
            Token::LeftBracket => {
                self.read();

                let mut items = Vec::new();

                while self.current != Token::RightBracket {
                    items.push(self.expression(0));

                    if self.current == Token::Comma {
                        self.read();
                    }
                }

                self.expect(Token::RightBracket);

                Expression::Array(items)
            },
            _ => todo!("{:?}", self.current),
        };

        loop {
            if self.current == Token::Eof || self.current == Token::SemiColon {
                break;
            }

            if let Some((lbp, _)) = postfix_binding_power(&self.current) {
                if lbp < bp {
                    break;
                }

                let op = self.current.clone();

                self.read();

                lhs = postfix(self, lhs, &op);

                continue;
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

fn postfix_binding_power(token: &Token) -> Option<(u8, ())> {
    Some(match token {
        Token::LeftParen | Token::LeftBracket => (19, ()),
        _ => return None
    })
}

fn postfix(parser: &mut Parser, lhs: Expression, op: &Token) -> Expression {
    match op {
        Token::LeftParen => {
            let name = match lhs {
                Expression::Identifier(i) => i,
                _ => unreachable!()
            };

            let mut args = Vec::new();

            while parser.current != Token::RightParen {
                args.push(parser.expression(0));

                if parser.current == Token::Comma {
                    parser.read()
                }
            }

            parser.expect(Token::RightParen);

            Expression::Call(name, args)
        },
        Token::LeftBracket => {
            let index = parser.expression(0);

            parser.expect(Token::RightBracket);

            Expression::Index(Box::new(lhs), Box::new(index))
        },
        _ => todo!("postfix: {:?}", op),
    }
}

fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
    Some(match token {
        Token::Multiply | Token::Divide => (13, 14),
        Token::Plus | Token::Minus => (11, 12),
        Token::Dot => (11, 11),
        Token::LessThan | Token::GreaterThan => (9, 10),
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
                Token::LessThan => Op::LessThan,
                Token::GreaterThan => Op::GreaterThan,
                Token::Dot => Op::Concat,
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
    LessThan,
    GreaterThan,
    Concat,
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
        tokens,
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