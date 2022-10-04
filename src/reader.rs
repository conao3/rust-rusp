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

    fn read_int(&mut self, end: usize) -> anyhow::Result<types::RuspExp> {
        let i = self.input[..end]
            .parse::<i64>()
            .with_context(|| "Failed to parse int")?;
        self.input = &self.input[end..];

        Ok(types::RuspExp::Atom(types::RuspAtom::Int(i)))
    }

    fn read_float(&mut self, end: usize) -> anyhow::Result<types::RuspExp> {
        let f = self.input[..end]
            .parse::<f64>()
            .with_context(|| "Failed to parse float")?;
        self.input = &self.input[end..];

        Ok(types::RuspExp::Atom(types::RuspAtom::Float(f)))
    }

    fn read_atom(&mut self) -> anyhow::Result<types::RuspExp> {
        let int_pattern = regex::Regex::new(r"^[0-9]+").unwrap();
        let float_pattern = regex::Regex::new(r"^[0-9]*\.[0-9]+").unwrap();

        if let Some(re) = float_pattern.find(self.input) {
            return self.read_float(re.end());
        }

        if let Some(re) = int_pattern.find(self.input) {
            return self.read_int(re.end());
        }

        Ok(types::RuspExp::Atom(types::RuspAtom::Symbol(
            self.input.to_string(),
        )))
    }

    pub fn read(&mut self) -> anyhow::Result<types::RuspExp> {
        self.skip_whitespace();
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuspErr::ReaderEofError)?;
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

        let input = "a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol('a'.to_string())));

        let input = "   a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol('a'.to_string())));
    }
}
