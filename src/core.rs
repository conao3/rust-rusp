use anyhow::Context;

use crate::builtin;
use crate::reader;
use crate::types;
use crate::util;

pub fn default_env<'a>() -> types::RuspEnv<'a> {
    let mut env = types::RuspEnv::default();

    env.variable.insert("nil".to_string(), types::nil!());
    env.variable.insert("t".to_string(), types::t!());

    types::rusp_func!(
        env,
        ("+", builtin::arith_plus),
        ("-", builtin::arith_minus),
        ("*", builtin::arith_multiply),
        ("/", builtin::arith_divide),
        ("<", builtin::arith_lt),
        ("<=", builtin::arith_lte),
        (">", builtin::arith_gt),
        (">=", builtin::arith_gte),
        ("=", builtin::arith_eq),
        ("!=", builtin::arith_neq),
        ("if", builtin::if_),
        ("set", builtin::set),
        ("setq", builtin::setq),
        ("quote", builtin::quote),
        ("lambda", builtin::lambda),
        ("apply", builtin::apply),
    )
    // ("def", builtin::def),
    // ("fn", builtin::fn_func),
    // ("let", builtin::let_func),
    // ("do", builtin::do_func),
    // ("eval", builtin::eval_func),
    // ("read", builtin::read_func),
    // ("print", builtin::print_func),
    // ("println", builtin::println_func),
    // ("list", builtin::list_func),
    // ("first", builtin::first_func),
    // ("rest", builtin::rest_func),
    // ("cons", builtin::cons_func),
    // ("concat", builtin::concat_func),
    // ("empty?", builtin::empty_func),
    // ("count", builtin::count_func),
    // ("apply", builtin::apply_func),
    // ("map", builtin::map_func),
    // ("filter", builtin::filter_func),
    // ("load", builtin::load_func),
    // ("time", builtin::time_func),
    // ("exit", builtin::exit_func),
    // ("throw", builtin::throw_func),
    // ("try", builtin::try_func),
    // ("catch", builtin::catch_func),
    // ("throw?", builtin::throwp_func)
}

fn read(x: &str) -> anyhow::Result<types::RuspExp> {
    let mut reader = reader::Reader::new(x);
    reader.read()
}

pub fn eval_lambda(
    func: &types::RuspExp,
    args: &types::RuspExp,
    env: &mut types::RuspEnv,
) -> anyhow::Result<types::RuspExp> {
    match func {
        types::RuspExp::Atom(types::RuspAtom::Lambda { params, body }) => {
            let symbols = params
                .into_iter()
                .map(|x_| {
                    let x = x_?;
                    match &**x {
                        types::RuspExp::Atom(types::RuspAtom::Symbol(s)) => Ok(s),
                        _ => Err(anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
                            expected: "symbol".into(),
                            actual: x.to_string().into()
                        })),
                    }
                })
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| {
                    anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
                        expected: "list<symbol>".into(),
                        actual: params.to_string().into()
                    })
                })?;

            let mut new_env = types::RuspEnv {
                outer: Some(env),
                ..Default::default()
            };

            for elm in util::safe_zip_eq(symbols, args.into_iter()) {
                let (sym, val_) = elm?;
                let val = val_?;
                new_env
                    .variable
                    .insert(sym.to_string(), eval(val, &mut env.clone())?);
            }

            eval(body, &mut new_env)
        }
        _ => Err(anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
            expected: "lambda".into(),
            actual: func.to_string().into()
        })),
    }
}

pub fn eval(x: &types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    match x {
        types::RuspExp::Atom(atom) => match atom {
            types::RuspAtom::Symbol(s) => Ok(env.get_variable(s)?.clone()),
            _ => Ok(x.clone()),
        },
        types::RuspExp::Cons { ref car, ref cdr } => || -> anyhow::Result<types::RuspExp> {
            if let types::RuspExp::Atom(types::RuspAtom::Symbol(s)) = &**car {
                let func = env.get_function(s)?;
                return match *func {
                    types::RuspExp::Atom(types::RuspAtom::Func(f)) => f(cdr, env),
                    _ => Err(anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
                        expected: "function".into(),
                        actual: format!("{}", func).into()
                    })),
                };
            }

            if let types::RuspExp::Cons{car: ref car_car, cdr: _} = &**car &&
                let types::RuspExp::Atom(types::RuspAtom::Symbol(s)) = &**car_car &&
                s == "lambda" {
                return eval_lambda(&eval(car, env)?, cdr, env);
            }

            Err(anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
                expected: "symbol".into(),
                actual: format!("{}", car).into()
            }))
        }(),
    }
}

fn print(x: types::RuspExp) -> anyhow::Result<String> {
    Ok(x.to_string())
}

pub fn rep(mut x: &str, env: &mut types::RuspEnv) -> anyhow::Result<String> {
    x = x.trim_start(); // simple skip whitespace
    anyhow::ensure!(!x.is_empty(), types::RuspErr::ReplEmptyError);

    print(eval(&read(x)?, env)?)
}
