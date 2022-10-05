use crate::reader;
use crate::types;

fn read<'a>(x: &'a str) -> anyhow::Result<types::RuspExp<'a>> {
    let mut reader = reader::Reader::new(x);
    reader.read()
}

fn eval(x: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    Ok(x)
}

fn print(x: types::RuspExp) -> anyhow::Result<String> {
    Ok(x.to_string())
}

pub fn rep(mut x: &str) -> anyhow::Result<String> {
    x = x.trim_start(); // simple skip whitespace
    anyhow::ensure!(!x.is_empty(), types::RuspErr::ReplEmptyError);

    print(eval(read(x)?)?)
}
