use anyhow::Context;

use crate::types;

pub struct Reader<'a> {
    input: &'a str,
}

impl Reader<'_> {
    pub fn new(input: &str) -> Reader {
        Reader { input }
    }

    pub fn read_atom(&self) -> anyhow::Result<types::RuspExp> {
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuspErr::ReaderEofError)?;
        match c {
            '0'..='9' => {
                let mut inx = 0;
                let mut is_float = false;
                for c in self.input.chars() {
                    if c == '.' {
                        is_float = true;
                    }
                    inx += 1;
                }
                if is_float {
                    let f = self.input[..inx]
                        .parse::<f64>()
                        .with_context(|| "Failed to parse float")?;
                    Ok(types::RuspExp::Atom(types::RuspAtom::Float(f)))
                } else {
                    let i = self.input[..inx]
                        .parse::<i64>()
                        .with_context(|| "Failed to parse int")?;
                    Ok(types::RuspExp::Atom(types::RuspAtom::Int(i)))
                }
            }
            _ => Ok(types::RuspExp::Atom(types::RuspAtom::Symbol(
                self.input.to_string(),
            ))),
        }
    }

    pub fn read(&self) -> anyhow::Result<types::RuspExp> {
        self.read_atom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::RuspAtom::*;
    use types::RuspErr;
    use types::RuspExp::*;

    #[test]
    fn test_read_atom() {
        let input = "";
        let readner = Reader::new(input);
        let exp = readner.read().unwrap_err();
        assert_eq!(exp.to_string(), RuspErr::ReaderEofError.to_string());

        let input = "42";
        let readner = Reader::new(input);
        let exp = readner.read().unwrap();
        assert_eq!(exp, Atom(Int(42)));

        let input = "42.3";
        let readner = Reader::new(input);
        let exp = readner.read().unwrap();
        assert_eq!(exp, Atom(Float(42.3)));
    }
}
