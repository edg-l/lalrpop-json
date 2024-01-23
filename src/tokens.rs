use logos::Logos;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    #[default]
    Other,
}

impl From<Infallible> for LexingError {
    fn from(_: Infallible) -> Self {
        LexingError::Other
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError, skip r"[ \r\t\n\f]+")]
pub enum Token<'input> {
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")]
    Number(&'input str),
    #[regex(r#""([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))*""#)]
    String(&'input str),
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
}
