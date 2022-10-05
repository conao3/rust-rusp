use crate::builtin;
use crate::reader;
use crate::types;

pub fn default_env() -> types::RuspEnv {
    let mut env = types::RuspEnv::default();

    env.value.insert(
        "nil".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Symbol("nil".to_string())),
    );
    env.value.insert(
        "t".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Symbol("t".to_string())),
    );

    env.function.insert(
        "+".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Func(builtin::plus)),
    );
    env.function.insert(
        "-".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Func(builtin::minus)),
    );
    env.function.insert(
        "*".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Func(builtin::multiply)),
    );
    env.function.insert(
        "/".to_string(),
        types::RuspExp::Atom(types::RuspAtom::Func(builtin::divide)),
    );
    env
}

fn read(x: &str) -> anyhow::Result<types::RuspExp> {
    let mut reader = reader::Reader::new(x);
    reader.read()
}

pub fn eval(x: types::RuspExp, env: &mut types::RuspEnv) -> anyhow::Result<types::RuspExp> {
    match x {
        types::RuspExp::Atom(atom) => match atom {
            types::RuspAtom::Symbol(s) => env
                .value
                .get(&s)
                .ok_or_else(|| anyhow::anyhow!("Symbol not found: {}", s))
                .map(|x| x.clone()),
            _ => Ok(types::RuspExp::Atom(atom)),
        },
        types::RuspExp::Cons { car, cdr } => match *car {
            types::RuspExp::Atom(types::RuspAtom::Symbol(s)) => {
                let func = env
                    .function
                    .get(&s)
                    .ok_or_else(|| anyhow::anyhow!("Function not found: {}", s))?;

                match *func {
                    types::RuspExp::Atom(types::RuspAtom::Func(f)) => f(*cdr, env),
                    _ => Err(anyhow::anyhow!("Not a function: {}", s)),
                }
            }
            _ => Err(anyhow::anyhow!("Not a function")),
        },
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
