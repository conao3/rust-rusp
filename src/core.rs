use crate::types;

fn read(x: &str) -> anyhow::Result<&str> {
    Ok(x)
}

fn eval(x: &str) -> anyhow::Result<&str> {
    Ok(x)
}

fn print(x: &str) -> anyhow::Result<&str> {
    Ok(x)
}

pub fn rep(x: &str) -> anyhow::Result<&str> {
    anyhow::ensure!(!x.is_empty(), types::RuspErr::ReaderEofError);

    print(eval(read(x)?)?)
}
