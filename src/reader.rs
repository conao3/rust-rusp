use anyhow::Context;

use crate::types;

pub struct Reader<'a> {
    input: &'a str,
}

impl Reader<'_> {
    pub fn new(input: &str) -> Reader {
        Reader { input }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn read_number(&mut self) -> anyhow::Result<types::RuspExp> {
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
            return Ok(types::RuspExp::Atom(types::RuspAtom::Float(f)));
        }

        let i = self.input[..inx]
            .parse::<i64>()
            .with_context(|| "Failed to parse int")?;
        Ok(types::RuspExp::Atom(types::RuspAtom::Int(i)))
    }

    fn read_atom(&mut self) -> anyhow::Result<types::RuspExp> {
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuspErr::ReaderEofError)?;
        match c {
            '0'..='9' => self.read_number(),
            _ => Ok(types::RuspExp::Atom(types::RuspAtom::Symbol(
                self.input.to_string(),
            ))),
        }
    }

    pub fn read(&mut self) -> anyhow::Result<types::RuspExp> {
        self.skip_whitespace();
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
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuspErr::ReaderEofError.to_string());

        let input = "42";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Int(42)));

        let input = "42.3";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Float(42.3)));

        let input = "   42.3";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Float(42.3)));
    }
}
