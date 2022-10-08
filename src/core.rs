use crate::builtin;
use crate::reader;
use crate::types;

pub fn default_env() -> types::RuspEnv {
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

pub fn eval(x: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    match x {
        types::RuspExp::Atom(atom) => match atom {
            types::RuspAtom::Symbol(s) => Ok(env.get_variable(&s)?.clone()),
            _ => Ok(types::RuspExp::Atom(atom)),
        },
        types::RuspExp::Cons { ref car, ref cdr } => || -> anyhow::Result<types::RuspExp> {
            if let types::RuspExp::Atom(types::RuspAtom::Symbol(s)) = &**car {
                let func = env.get_function(s)?;
                return match *func {
                    types::RuspExp::Atom(types::RuspAtom::Func(f)) => f(*cdr.clone(), env),
                    _ => Err(anyhow::anyhow!(types::RuspErr::WrongTypeArgument {
                        expected: "function".into(),
                        actual: format!("{}", func).into()
                    })),
                };
            }

            if let types::RuspExp::Cons{car: ref car_car, cdr: ref car_cdr} = &**car &&
                let types::RuspExp::Atom(types::RuspAtom::Symbol(s)) = &**car_car &&
                s == "lambda" {
                return eval(
                    types::RuspExp::Cons {
                        car: Box::new(types::RuspExp::Atom(types::RuspAtom::Symbol("apply".to_string()))),
                        cdr: Box::new(types::RuspExp::Cons{
                            car: Box::new(*car.clone()),
                            cdr: Box::new(types::RuspExp::Cons{
                                car: Box::new(types::RuspExp::Atom(types::RuspAtom::Symbol("quote".to_string()))),
                                cdr: Box::new(*car_cdr.clone()),
                            })
                        }),
                    }, env
                );
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

    print(eval(read(x)?, env)?)
}
