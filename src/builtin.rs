use anyhow::Context;

use crate::core;
use crate::types;

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
                    types::RuspExp::Atom(types::RuspAtom::Int(x)) =>
                        init = x as f64,
                    types::RuspExp::Atom(types::RuspAtom::Float(x)) =>
                        init = x,
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

pub fn plus(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc + x, 0, false)(arg)
}

pub fn minus(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc - x, 0, true)(arg)
}

pub fn multiply(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc * x, 1, false)(arg)
}

pub fn divide(arg: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    basic_op!(env, |acc, x| acc / x, 1, true)(arg)
}
