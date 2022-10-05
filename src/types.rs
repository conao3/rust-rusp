#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum RuspErr {
    #[error("ReplEmptyError")]
    ReplEmptyError,
    #[error("ReaderError")]
    ReaderError,
    #[error("ReaderEofError")]
    ReaderEofError,
    #[error("ReaderInternalError")]
    ReaderInternalError,
    #[error("TypeError")]
    TypeError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuspAtom<'a> {
    Int(i64),
    String(&'a str),
    Symbol(&'a str),
}

pub const T: RuspExp = RuspExp::Atom(RuspAtom::Symbol("t"));
pub const NIL: RuspExp = RuspExp::Atom(RuspAtom::Symbol("nil"));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuspExp<'a> {
    Atom(RuspAtom<'a>),
    Cons {
        car: &'a RuspExp<'a>,
        cdr: &'a RuspExp<'a>,
    },
}

#[derive(Debug, PartialEq)]
pub struct RuspEnv<'a> {
    pub bindings: std::collections::HashMap<String, RuspExp<'a>>,
}

impl std::fmt::Display for RuspAtom<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            RuspAtom::Int(i) => i.to_string(),
            // RuspAtom::Float(i) => i.to_string(),
            RuspAtom::String(s) => s.to_string(),
            RuspAtom::Symbol(s) => s.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for RuspExp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            RuspExp::Atom(atom) => atom.to_string(),
            RuspExp::Cons { car, cdr } => {
                let mut lst: Vec<String> = vec![];
                let mut cell = *cdr;

                lst.push(car.to_string());
                while let RuspExp::Cons {
                    car: cell_car,
                    cdr: cell_cdr,
                } = *cell
                {
                    lst.push(cell_car.to_string());
                    cell = cell_cdr;
                }

                let cdr_atom = cell;
                match *cdr_atom {
                    NIL => (),
                    _ => {
                        lst.push('.'.to_string());
                        lst.push(cdr_atom.to_string());
                    }
                }

                format!("({})", lst.join(" "))
            }
        };
        write!(f, "{}", str)
    }
}

impl<'a> RuspExp<'a> {
    pub fn cons(&'a self, car: &'a RuspExp<'a>) -> RuspExp<'a> {
        RuspExp::Cons { car, cdr: self }
    }
}
