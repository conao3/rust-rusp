#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum RuspErr {
    #[error("ReaderError")]
    ReaderError,
    #[error("ReaderEofError")]
    ReaderEofError,
    #[error("ReaderInternalError")]
    ReaderInternalError,
}

#[derive(Debug, PartialEq)]
pub enum RuspAtom {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
}

#[derive(Debug, PartialEq)]
pub enum RuspExp {
    Atom(RuspAtom),
    Cons {
        car: Box<RuspExp>,
        cdr: Box<RuspExp>,
    },
}
