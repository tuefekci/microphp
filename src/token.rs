use logos::Logos;

#[derive(Debug, Logos, PartialEq, Clone)]
pub enum Token<'t> {
    #[regex(r"\$[a-zA-Z_]+")]
    Variable(&'t str),
    #[regex(r"[a-zA-Z_]+")]
    Identifier,
    #[regex(r##""(?:[^"\\]|\\.)*""##)]
    String(&'t str),

    #[token("<?php")]
    OpenTag,
    #[token("#!(.*)")]
    Shebang,

    #[token("echo")]
    Echo,

    #[token(";")]
    SemiColon,

    Eof,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn generate(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}