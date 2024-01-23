use std::{fmt::Display, ops::Range};

use logos::{Logos, SpannedIter};

use crate::tokens::{LexingError, Token};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Clone)]
pub enum LexicalError {
    InvalidToken(LexingError, Range<usize>),
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalError::InvalidToken(err, span) => {
                write!(f, "lexical error at ({:?}): {:?}", err, span)
            }
        }
    }
}

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(err) => Err(LexicalError::InvalidToken(err, span)),
        })
    }
}
