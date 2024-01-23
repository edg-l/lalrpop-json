use std::collections::HashMap;


#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Self {
            lo, hi
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object<'input> {
    pub values: HashMap<&'input str, Value<'input>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Array<'input> {
    pub values: Vec<Value<'input>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Value<'input> {
    String(&'input str, Span),
    Number(&'input str, Span),
    Bool(bool, Span),
    Array(Array<'input>),
    Object(Object<'input>),
    Null(Span),
}
