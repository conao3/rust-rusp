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

#[derive(Clone)]
pub enum RuspAtom {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Func(fn(RuspExp, &mut RuspEnv) -> anyhow::Result<RuspExp>),
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

macro_rules! rusp_func {
    ($env: ident, $(($key:expr, $value:path)),*) => {
        {
            $(
                $env.function.insert($key.to_string(), crate::types::RuspExp::Atom(crate::types::RuspAtom::Func($value)));
            )*
            $env
        }
    };
}

macro_rules! nil {
    () => {
        crate::types::RuspExp::Atom(crate::types::RuspAtom::Symbol("nil".to_string()))
    };
}

macro_rules! t {
    () => {
        crate::types::RuspExp::Atom(crate::types::RuspAtom::Symbol("t".to_string()))
    };
}

pub(crate) use nil;
pub(crate) use rusp_func;
pub(crate) use t;

impl PartialEq for RuspAtom {
    fn eq(&self, other: &RuspAtom) -> bool {
        if let RuspAtom::Func(_) = self {
            return false;
        }
        if let RuspAtom::Func(_) = other {
            return false;
        }
        self == other
    }
}

impl std::fmt::Debug for RuspAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuspAtom::Int(i) => write!(f, "{}", i),
            RuspAtom::Float(i) => write!(f, "{}", i),
            RuspAtom::String(i) => write!(f, "{}", i),
            RuspAtom::Symbol(i) => write!(f, "{}", i),
            RuspAtom::Func(_) => write!(f, "#<function>"),
        }
    }
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

impl RuspExp {
    pub fn nilp(&self) -> bool {
        match self {
            RuspExp::Atom(RuspAtom::Symbol(s)) => s == "nil",
            _ => false,
        }
    }

    pub fn atom(&self) -> bool {
        matches!(self, RuspExp::Atom(_))
    }

    pub fn consp(&self) -> bool {
        matches!(self, RuspExp::Cons { .. })
    }

    pub fn intp(&self) -> bool {
        matches!(self, RuspExp::Atom(RuspAtom::Int(_)))
    }

    pub fn floatp(&self) -> bool {
        matches!(self, RuspExp::Atom(RuspAtom::Float(_)))
    }

    pub fn numberp(&self) -> bool {
        self.intp() || self.floatp()
    }

    pub fn stringp(&self) -> bool {
        matches!(self, RuspExp::Atom(RuspAtom::String(_)))
    }

    pub fn symbolp(&self) -> bool {
        matches!(self, RuspExp::Atom(RuspAtom::Symbol(_)))
    }

    pub fn functionp(&self) -> bool {
        matches!(self, RuspExp::Atom(RuspAtom::Func(_)))
    }

    pub fn iter_mut(&mut self) -> std::vec::IntoIter<anyhow::Result<&mut RuspExp>> {
        let mut lst: Vec<anyhow::Result<&mut RuspExp>> = vec![];
        let mut cell = self;

        while let RuspExp::Cons {
            car: cell_car,
            cdr: cell_cdr,
        } = cell
        {
            lst.push(Ok(cell_car));
            cell = cell_cdr;
        }

        if !cell.nilp() {
            lst.push(Err(anyhow::anyhow!("dotlist")));
        }
        lst.into_iter()
    }
}

pub struct ListIter<'a>(&'a RuspExp);

impl<'a> Iterator for ListIter<'a> {
    type Item = anyhow::Result<&'a Box<RuspExp>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let RuspExp::Cons { car, cdr } = self.0 {
            self.0 = cdr;
            return Some(Ok(car));
        }

        if !self.0.nilp() {
            return Some(Err(anyhow::anyhow!("dotlist")));
        }

        None
    }
}

impl<'a> IntoIterator for &'a RuspExp {
    type Item = anyhow::Result<&'a Box<RuspExp>>;
    type IntoIter = ListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}
