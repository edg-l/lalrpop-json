pub use ast::{Array, Object, Span, Value};
use lalrpop_util::ParseError;
use lexer::LexicalError;
use tokens::Token;

use crate::lexer::Lexer;

mod ast;
mod lexer;
mod tokens;

pub mod grammar {
    #![allow(unused_variables)]

    pub use self::grammar::*;
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(pub grammar);
}

pub fn parse_value(input: &str) -> Result<Value<'_>, ParseError<usize, Token<'_>, LexicalError>> {
    let lexer = Lexer::new(input);
    let parser = grammar::ValueParser::new();
    parser.parse(lexer)
}

pub fn parse_object(input: &str) -> Result<Object<'_>, ParseError<usize, Token<'_>, LexicalError>> {
    let lexer = Lexer::new(input);
    let parser = grammar::ObjectParser::new();
    parser.parse(lexer)
}

pub fn parse_array(input: &str) -> Result<Array<'_>, ParseError<usize, Token<'_>, LexicalError>> {
    let lexer = Lexer::new(input);
    let parser = grammar::ArrayParser::new();
    parser.parse(lexer)
}

#[cfg(test)]
mod test {
    use crate::parse_value;

    #[test]
    fn test_value() {
        parse_value(r#"{ "hello": "world", "a": [2, "s"] }"#).unwrap();
    }
}
