use anyhow::Context;

use crate::types;

pub fn plus(arg: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    let mut acc_int: i64 = 0;
    let mut acc_float: f64 = 0.0;
    let mut float_p = false;

    for exp_ in arg.into_iter() {
        let exp = exp_.with_context(|| types::RuspErr::WrongTypeArgument)?;
        match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                if float_p {
                    acc_float += i as f64;
                    continue;
                }
                acc_int += i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                if !float_p {
                    acc_float = acc_int as f64;
                    float_p = true;
                }
                acc_float += f;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        }
    }

    if float_p {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Float(acc_float)));
    }

    Ok(types::RuspExp::Atom(types::RuspAtom::Int(acc_int)))
}

pub fn minus(arg: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    let mut acc_int: i64 = 0;
    let mut acc_float: f64 = 0.0;
    let mut float_p = false;

    let mut iter = arg.into_iter();
    let first = iter.next();
    if first.is_none() {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Int(0)));
    }

    match first.unwrap() {
        Ok(exp) => match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                acc_int = i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                acc_float = f;
                float_p = true;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        },
        Err(e) => anyhow::bail!(e),
    }

    for exp_ in iter {
        let exp = exp_.with_context(|| types::RuspErr::WrongTypeArgument)?;
        match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                if float_p {
                    acc_float -= i as f64;
                    continue;
                }
                acc_int -= i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                if !float_p {
                    acc_float = acc_int as f64;
                    float_p = true;
                }
                acc_float -= f;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        }
    }

    if float_p {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Float(acc_float)));
    }

    Ok(types::RuspExp::Atom(types::RuspAtom::Int(acc_int)))
}

pub fn multiply(arg: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    let mut acc_int: i64 = 1;
    let mut acc_float: f64 = 1.0;
    let mut float_p = false;

    for exp_ in arg.into_iter() {
        let exp = exp_.with_context(|| types::RuspErr::WrongTypeArgument)?;
        match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                if float_p {
                    acc_float += i as f64;
                    continue;
                }
                acc_int *= i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                if !float_p {
                    acc_float = acc_int as f64;
                    float_p = true;
                }
                acc_float *= f;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        }
    }

    if float_p {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Float(acc_float)));
    }

    Ok(types::RuspExp::Atom(types::RuspAtom::Int(acc_int)))
}

pub fn divide(arg: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    let mut acc_int: i64 = 0;
    let mut acc_float: f64 = 0.0;
    let mut float_p = false;

    let mut iter = arg.into_iter();
    let first = iter.next();
    if first.is_none() {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Int(1)));
    }

    match first.unwrap() {
        Ok(exp) => match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                acc_int = i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                acc_float = f;
                float_p = true;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        },
        Err(e) => anyhow::bail!(e),
    }

    for exp_ in iter {
        let exp = exp_.with_context(|| types::RuspErr::WrongTypeArgument)?;
        match **exp {
            types::RuspExp::Atom(types::RuspAtom::Int(i)) => {
                if float_p {
                    acc_float /= i as f64;
                    continue;
                }
                acc_int /= i;
            }
            types::RuspExp::Atom(types::RuspAtom::Float(f)) => {
                if !float_p {
                    acc_float = acc_int as f64;
                    float_p = true;
                }
                acc_float /= f;
            }
            _ => anyhow::bail!(types::RuspErr::WrongTypeArgument),
        }
    }

    if float_p {
        return Ok(types::RuspExp::Atom(types::RuspAtom::Float(acc_float)));
    }

    Ok(types::RuspExp::Atom(types::RuspAtom::Int(acc_int)))
}
