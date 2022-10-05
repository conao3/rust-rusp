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

    #[error("WrongTypeArgument")]
    WrongTypeArgument,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuspAtom {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Func(fn(RuspExp) -> anyhow::Result<RuspExp>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuspExp {
    Atom(RuspAtom),
    Cons {
        car: Box<RuspExp>,
        cdr: Box<RuspExp>,
    },
}

#[derive(Debug, PartialEq, Default)]
pub struct RuspEnv {
    pub value: std::collections::HashMap<String, RuspExp>,
    pub function: std::collections::HashMap<String, RuspExp>,
}

impl std::fmt::Display for RuspAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RuspAtom::Int(i) => i.to_string(),
            RuspAtom::Float(i) => i.to_string(),
            RuspAtom::String(s) => s.to_string(),
            RuspAtom::Symbol(s) => s.to_string(),
            RuspAtom::Func(_) => "#<function>".to_string(),
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

pub struct ListIter<'a>(&'a RuspExp);

impl<'a> Iterator for ListIter<'a> {
    type Item = anyhow::Result<&'a Box<RuspExp>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            RuspExp::Cons { car, cdr } => {
                self.0 = cdr;
                Some(Ok(car))
            }
            RuspExp::Atom(atom) => {
                match atom {
                    RuspAtom::Symbol(s) if s == "nil" => None,
                    _ => Some(Err(anyhow::anyhow!(RuspErr::WrongTypeArgument))),
                }
            },
        }
    }
}

impl<'a> IntoIterator for &'a RuspExp {
    type Item = anyhow::Result<&'a Box<RuspExp>>;
    type IntoIter = ListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}
