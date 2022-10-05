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

impl std::fmt::Display for RuspAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RuspAtom::Int(i) => i.to_string(),
            RuspAtom::Float(i) => i.to_string(),
            RuspAtom::String(s) => s.to_string(),
            RuspAtom::Symbol(s) => s.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for RuspExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            RuspExp::Atom(atom) => atom.to_string(),
            RuspExp::Cons { car, cdr } => {
                let mut lst: Vec<String> = vec![];
                let mut cell = cdr;

                lst.push(car.to_string());
                while let RuspExp::Cons {
                    car: cell_car,
                    cdr: cell_cdr,
                } = &**cell
                {
                    lst.push(cell_car.to_string());
                    cell = cell_cdr;
                }

                if let RuspExp::Atom(atom) = &**cell {
                    match atom {
                        RuspAtom::Symbol(s) if s == "nil" => (),
                        _ => {
                            lst.push('.'.to_string());
                            lst.push(atom.to_string());
                        }
                    }
                }
                format!("({})", lst.join(" "))
            }
        };
        write!(f, "{}", str)
    }
}
