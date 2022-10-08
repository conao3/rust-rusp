#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum RuspErr {
    #[error("DummyError")]
    DummyError,

    #[error("ReplEmptyError")]
    ReplEmptyError,
    #[error("ReaderError")]
    ReaderError,
    #[error("ReaderEofError")]
    ReaderEofError,

    #[error("WrongTypeArgument")]
    WrongTypeArgument {
        expected: std::borrow::Cow<'static, str>,
        actual: std::borrow::Cow<'static, str>,
    },
    #[error("WrongNumberOfArguments")]
    WrongNumberOfArguments {
        required: usize,
        allowed: Option<usize>,
        actual: usize,
    },
    #[error("VoidVariable")]
    VoidVariable {
        name: std::borrow::Cow<'static, str>,
    },
    #[error("VoidFunction")]
    VoidFunction {
        name: std::borrow::Cow<'static, str>,
    },
}

#[derive(Clone)]
pub enum RuspAtom {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Keyword(String),
    Func(fn(&RuspExp, &mut RuspEnv) -> anyhow::Result<RuspExp>),
    Lambda {
        params: Box<RuspExp>,
        body: Box<RuspExp>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuspExp {
    Atom(RuspAtom),
    Cons {
        car: Box<RuspExp>,
        cdr: Box<RuspExp>,
    },
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct RuspEnv<'a> {
    pub variable: std::collections::HashMap<String, RuspExp>,
    pub function: std::collections::HashMap<String, RuspExp>,
    pub outer: Option<&'a RuspEnv<'a>>,
}

macro_rules! rusp_func {
    ($env: ident, $(($key:expr, $value:path)),*,) => {
        {
            $(
                $env.function.insert(
                    $key.to_string(),
                    crate::types::RuspExp::Atom(crate::types::RuspAtom::Func($value)),
                );
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
            RuspAtom::Keyword(i) => write!(f, ":{}", i),
            RuspAtom::Func(_) => write!(f, "#<function>"),
            RuspAtom::Lambda { params, body } => write!(f, "#<lambda {} {}>", params, body),
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
            RuspAtom::Keyword(s) => format!(":{}", s),
            RuspAtom::Func(_) => "#<function>".to_string(),
            RuspAtom::Lambda { params, body } => format!("#<lambda {} {}>", params, body),
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for RuspExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            RuspExp::Atom(atom) => atom.to_string(),
            RuspExp::Cons { car, cdr } => || -> String {
                if let RuspExp::Atom(RuspAtom::Symbol(s)) = &**car && s == "quote" {
                    if let RuspExp::Cons { car, cdr } = &**cdr {
                        if cdr.nilp() {
                            return format!("'{}", car);
                        }
                    }
                }
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

                if cell.non_nil_p() {
                    lst.push(format!(". {}", cell));
                }

                format!("({})", lst.join(" "))
            }(),
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for RuspEnv<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut var_lst: Vec<String> = vec![];
        for (k, v) in &self.variable {
            var_lst.push(format!("({} {})", k, v));
        }
        let mut func_lst: Vec<String> = vec![];
        for (k, v) in &self.function {
            func_lst.push(format!("({} {})", k, v));
        }
        write!(
            f,
            "(var ({}) fn ({}))",
            var_lst.join(" "),
            func_lst.join(" ")
        )
    }
}

impl RuspExp {
    pub fn nilp(&self) -> bool {
        match self {
            RuspExp::Atom(RuspAtom::Symbol(s)) => s == "nil",
            _ => false,
        }
    }

    pub fn non_nil_p(&self) -> bool {
        !self.nilp()
    }

    pub fn atom(&self) -> bool {
        matches!(self, RuspExp::Atom(_))
    }

    pub fn consp(&self) -> bool {
        matches!(self, RuspExp::Cons { .. })
    }

    pub fn listp(&self) -> bool {
        self.nilp() || self.consp()
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
            // dotlist is not allowed
            lst.push(Err(anyhow::anyhow!(RuspErr::WrongTypeArgument {
                expected: "list".into(),
                actual: cell.to_string().into()
            })));
        }
        lst.into_iter()
    }

    pub fn car(&self) -> anyhow::Result<&RuspExp> {
        match self {
            RuspExp::Cons { car, .. } => Ok(car),
            _ => Err(anyhow::anyhow!(RuspErr::WrongTypeArgument {
                expected: "cons".into(),
                actual: self.to_string().into()
            })),
        }
    }

    pub fn cdr(&self) -> anyhow::Result<&RuspExp> {
        match self {
            RuspExp::Cons { cdr, .. } => Ok(cdr),
            _ => Err(anyhow::anyhow!(RuspErr::WrongTypeArgument {
                expected: "cons".into(),
                actual: self.to_string().into()
            })),
        }
    }
}

impl RuspEnv<'_> {
    pub fn get_variable(&self, key: &str) -> anyhow::Result<&RuspExp> {
        if let Some(val) = self.variable.get(key) {
            return Ok(val);
        }

        if let Some(env) = &self.outer {
            return env.get_variable(key);
        }

        Err(anyhow::anyhow!(RuspErr::VoidVariable {
            name: key.to_string().into()
        }))
    }

    pub fn get_function(&self, key: &str) -> anyhow::Result<&RuspExp> {
        if let Some(val) = self.function.get(key) {
            return Ok(val);
        }

        if let Some(env) = &self.outer {
            return env.get_function(key);
        }

        Err(anyhow::anyhow!(RuspErr::VoidFunction {
            name: key.to_string().into()
        }))
    }
}

macro_rules! extract_args {
    (@var $var: ident, $args: ident, $args_len: ident, $nil: ident) => {
        let $var = $args.pop_front().ok_or_else(|| anyhow::anyhow!(types::RuspErr::WrongNumberOfArguments {
            required: 1,
            allowed: None,
            actual: $args_len,
        }))?;
    };
    (@var &optional $var: ident, $args: ident, $args_len: ident, $nil: ident) => {
        let $var = $args.pop_front().unwrap_or_else(|| &$nil);
    };
    ($arg: ident, $env: ident, _, $body: block) => {{
        $body
    }};
    ($arg: ident, $env: ident, ($($(& $annotation: ident)? $var: ident),+), $body: block) => {{
        let mut args = $arg.into_iter().collect::<Result<std::collections::VecDeque<_>, _>>()?;
        let args_len = args.len();
        #[allow(unused_variables)]
        let nil = Box::new(crate::types::nil!());
        $(
            crate::types::extract_args!(@var $(& $annotation)? $var, args, args_len, nil);
        )+
        anyhow::ensure!(args.is_empty(), types::RuspErr::WrongNumberOfArguments {
            required: 1,
            allowed: None,
            actual: args_len,
        });
        $body
    }};
}

pub(crate) use extract_args;

pub struct ListIter<'a>(&'a RuspExp);

impl<'a> Iterator for ListIter<'a> {
    type Item = anyhow::Result<&'a Box<RuspExp>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let RuspExp::Cons { car, cdr } = self.0 {
            self.0 = cdr;
            return Some(Ok(car));
        }

        if !self.0.nilp() {
            // dotlist is not allowed
            return Some(Err(anyhow::anyhow!(RuspErr::WrongTypeArgument {
                expected: "nil".into(),
                actual: self.0.to_string().into()
            })));
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
