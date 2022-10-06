use anyhow::Context;

use crate::core;
use crate::types;

macro_rules! defun {
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
    ($env: expr, $fn: expr, $init: expr, $first_init_p: expr) => {
        |arg: types::RuspExp| -> anyhow::Result<types::RuspExp> {
            let arg_lst = arg
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("{}", stringify!($fn)))?;

            let lst = arg_lst
                .iter()
                .map(|x| core::eval((***x).clone(), $env))
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("{}", stringify!($fn)))?;

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
        }
    };
}

macro_rules! basic_pred {
    ($env: expr, $fn: expr) => {
        |arg: types::RuspExp| -> anyhow::Result<types::RuspExp> {
            let arg_lst = arg
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("{}", stringify!($fn)))?;

            let lst = arg_lst
                .iter()
                .map(|x| core::eval((***x).clone(), $env))
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("{}", stringify!($fn)))?;

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
        }
    };
}

pub fn arith_plus(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc + x, 0, false)(arg)
}

pub fn arith_minus(
    arg: types::RuspExp,
    env: &mut types::RuspEnv,
) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc - x, 0, true)(arg)
}

pub fn arith_multiply(
    arg: types::RuspExp,
    env: &mut types::RuspEnv,
) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc * x, 1, false)(arg)
}

pub fn arith_divide(
    arg: types::RuspExp,
    env: &mut types::RuspEnv,
) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc / x, 1, true)(arg)
}

pub fn arith_eq(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc == x)(arg)
}

pub fn arith_neq(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc != x)(arg)
}

pub fn arith_lt(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc < x)(arg)
}

pub fn arith_lte(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc <= x)(arg)
}

pub fn arith_gt(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc > x)(arg)
}

pub fn arith_gte(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_pred!(env, |acc, x| acc >= x)(arg)
}

defun!(if_, arg, env, (cond, then, &optional else_), {
    if core::eval(*cond.clone(), env)?.non_nil_p() {
        core::eval(*then.clone(), env)
    } else {
        core::eval(*else_.clone(), env)
    }
});
