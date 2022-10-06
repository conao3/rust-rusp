use crate::core;
use crate::types;

macro_rules! defun {
    ($name: ident, $arg: ident, $env: ident, _, $body: block) => {
        pub fn $name(
            $arg: types::RuspExp,
            $env: &mut types::RuspEnv,
        ) -> anyhow::Result<types::RuspExp> {
            types::extract_args!($arg, $env, _, $body)
        }
    };
    ($name: ident, $arg: ident, $env: ident, $arglist: tt, $body: block) => {
        pub fn $name(
            $arg: types::RuspExp,
            $env: &mut types::RuspEnv,
        ) -> anyhow::Result<types::RuspExp> {
            types::extract_args!($arg, $env, $arglist, $body)
        }
    };
}

macro_rules! basic_op {
    ($arg: ident, $env: ident, $fn: expr, $init: expr, $first_init_p: expr) => {{
        let arg_lst = $arg.into_iter().collect::<Result<Vec<_>, _>>()?;

        let lst = arg_lst
            .iter()
            .map(|x| core::eval((***x).clone(), $env))
            .collect::<Result<Vec<_>, _>>()?;

        anyhow::ensure!(
            lst.iter().all(|x| x.numberp()),
            types::RuspErr::WrongTypeArgument
        );

        if lst.len() == 0 {
            return Ok(types::RuspExp::Atom(types::RuspAtom::Int($init)));
        }

        let floatp = lst.iter().any(|x| x.floatp());
        let mut init: f64 = $init as f64;
        let mut iter = lst.into_iter();

        if $first_init_p {
            match iter.next().unwrap() {
                types::RuspExp::Atom(types::RuspAtom::Int(x)) => init = x as f64,
                types::RuspExp::Atom(types::RuspAtom::Float(x)) => init = x,
                _ => unreachable!(),
            };
        }

        if floatp {
            let mut acc: f64 = init as f64;
            for x in iter {
                match x {
                    types::RuspExp::Atom(types::RuspAtom::Int(i)) => acc = $fn(acc, i as f64),
                    types::RuspExp::Atom(types::RuspAtom::Float(f)) => acc = $fn(acc, f),
                    _ => unreachable!(),
                }
            }
            return Ok(types::RuspExp::Atom(types::RuspAtom::Float(acc)));
        }

        let mut acc: i64 = init as i64;
        for x in iter {
            match x {
                types::RuspExp::Atom(types::RuspAtom::Int(i)) => acc = $fn(acc, i),
                _ => unreachable!(),
            }
        }
        Ok(types::RuspExp::Atom(types::RuspAtom::Int(acc)))
    }};
}

macro_rules! basic_pred {
    ($arg: ident, $env: ident, $fn: expr) => {{
        let arg_lst = $arg.into_iter().collect::<Result<Vec<_>, _>>()?;

        let lst = arg_lst
            .iter()
            .map(|x| core::eval((***x).clone(), $env))
            .collect::<Result<Vec<_>, _>>()?;

        anyhow::ensure!(
            lst.iter().all(|x| x.numberp()),
            types::RuspErr::WrongTypeArgument
        );

        if lst.len() <= 1 {
            return Ok(types::t!());
        }

        let floatp = lst.iter().any(|x| x.floatp());
        let mut iter = lst.into_iter();
        let mut res: bool = true;

        if floatp {
            let mut last_value: f64 = match iter.next().unwrap() {
                types::RuspExp::Atom(types::RuspAtom::Int(i)) => i as f64,
                types::RuspExp::Atom(types::RuspAtom::Float(f)) => f,
                _ => unreachable!(),
            };
            for x in iter {
                match x {
                    types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                        if !$fn(last_value, i as f64) {
                            res = false;
                            break;
                        }
                        last_value = i as f64;
                    }
                    types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                        if !$fn(last_value, f) {
                            res = false;
                            break;
                        }
                        last_value = f;
                    }
                    _ => unreachable!(),
                }
            }
            if res {
                return Ok(types::t!());
            }
            return Ok(types::nil!());
        }

        let mut last_value: i64 = match iter.next().unwrap() {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => i,
            _ => unreachable!(),
        };
        for x in iter {
            match x {
                types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                    if !$fn(last_value, i) {
                        res = false;
                        break;
                    }
                    last_value = i;
                }
                _ => unreachable!(),
            }
        }
        if res {
            return Ok(types::t!());
        }
        Ok(types::nil!())
    }};
}

defun!(arith_plus, arg, env, _, {
    basic_op!(arg, env, |acc, x| acc + x, 0, false)
});

defun!(arith_minus, arg, env, _, {
    basic_op!(arg, env, |acc, x| acc - x, 0, true)
});

defun!(arith_multiply, arg, env, _, {
    basic_op!(arg, env, |acc, x| acc * x, 1, false)
});

defun!(arith_divide, arg, env, _, {
    basic_op!(arg, env, |acc, x| acc / x, 1, true)
});

defun!(arith_eq, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc == x)
});

defun!(arith_neq, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc != x)
});

defun!(arith_lt, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc < x)
});

defun!(arith_lte, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc <= x)
});

defun!(arith_gt, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc > x)
});

defun!(arith_gte, arg, env, _, {
    basic_pred!(arg, env, |acc, x| acc >= x)
});

defun!(if_, arg, env, (cond, then, &optional else_), {
    if core::eval(*cond.clone(), env)?.non_nil_p() {
        return core::eval(*then.clone(), env)
    }
    core::eval(*else_.clone(), env)
});
