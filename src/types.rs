#[derive(Debug, Eq, PartialEq)]
pub enum RuspErr {
    ReaderError,
    ReaderEofError,
    ReaderInternalError(String),
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
    Cons(RuspAtom, RuspAtom),
}
