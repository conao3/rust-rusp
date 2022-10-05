use crate::reader;
use crate::types;

fn read(x: &str) -> anyhow::Result<types::RuspExp> {
    let mut reader = reader::Reader::new(x);
    reader.read()
}

fn eval(x: types::RuspExp) -> anyhow::Result<types::RuspExp> {
    Ok(x)
}

fn print(x: types::RuspExp) -> anyhow::Result<String> {
    Ok(x.to_string())
}

pub fn rep(x: &str) -> anyhow::Result<String> {
    anyhow::ensure!(!x.is_empty(), types::RuspErr::ReaderEofError);

    print(eval(read(x)?)?)
}
