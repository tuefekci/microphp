use logos::Logos;

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Token<'t> {
    #[regex(r"\$[a-zA-Z_]+")]
    Variable(&'t str),
    #[regex(r"[a-zA-Z_]+")]
    Identifier(&'t str),

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("function")]
    Function,
    #[token("return")]
    Return,
    #[token("for")]
    For,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r##""(?:[^"\\]|\\.)*""##)]
    String(&'t str),
    #[regex(r"[0-9]+", |l| l.slice().parse())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |l| l.slice().parse())]
    Float(f64),

    #[token("<?php")]
    OpenTag,
    #[token("#!(.*)")]
    Shebang,

    #[token("echo")]
    Echo,

    #[token(";")]
    SemiColon,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("=")]
    Assign,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    Eof,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn generate(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}