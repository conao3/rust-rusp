use crate::types;

static INT_PATTERN: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^([+-]?[0-9]+)(?:[ ();]|$)").unwrap());
static FLOAT_PATTERN: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(r"^([+-]?[0-9]*\.[0-9]+)(?:[ ();]|$)").unwrap()
});
static SYMBOL_PATTERN: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[^ ();]+").unwrap());

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

    fn read_string(&mut self) -> anyhow::Result<types::RuspExp> {
        let mut result = String::new();
        let mut chars = self.input.chars();

        chars.next(); // skip first "
        loop {
            let c = chars.next();
            match c {
                Some('"') => {
                    break;
                }
                Some('\\') => match chars.next() {
                    Some('\"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some(s) => {
                        return Err(anyhow::anyhow!(types::RuspErr::ReaderInvalidEscapeError {
                            char: s
                        }))
                    }
                    None => return Err(anyhow::anyhow!(types::RuspErr::ReaderUnclosedStringError)),
                },
                Some(c) => {
                    result.push(c);
                }
                None => anyhow::bail!(types::RuspErr::ReaderUnclosedStringError),
            }
        }
        self.input = chars.as_str();
        Ok(types::RuspExp::Atom(types::RuspAtom::String(result)))
    }

    fn read_atom(&mut self) -> anyhow::Result<types::RuspExp> {
        self.skip_whitespace();

        if let Some(m) = FLOAT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let f = s.parse::<f64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(types::RuspExp::Atom(types::RuspAtom::Float(f)));
        }

        if let Some(m) = INT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let i = s.parse::<i64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(types::RuspExp::Atom(types::RuspAtom::Int(i)));
        }

        if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            self.input = &self.input[s.len()..];
            return Ok(types::RuspExp::Atom(types::RuspAtom::Symbol(s.to_string())));
        }

        unreachable!()
    }

    fn read_cons(&mut self) -> anyhow::Result<types::RuspExp> {
        self.skip_whitespace();

        anyhow::ensure!(!self.input.is_empty(), types::RuspErr::ReaderEofError);

        if self.input.starts_with(')') {
            self.input = &self.input[1..]; // skip ')'
            return Ok(types::nil!());
        }

        let car = self.read()?;

        self.skip_whitespace();
        if self.input.starts_with('.') {
            self.input = &self.input[1..]; // skip '.'

            self.skip_whitespace();

            if self.input.starts_with(')') {
                self.input = &self.input[1..]; // skip ')'
                anyhow::bail!(types::RuspErr::ReaderEofError);
            }

            let cdr = self.read()?;

            self.skip_whitespace();
            anyhow::ensure!(self.input.starts_with(')'), types::RuspErr::ReaderEofError);

            self.input = &self.input[1..]; // skip ')'

            return Ok(types::RuspExp::Cons {
                car: Box::new(car),
                cdr: Box::new(cdr),
            });
        }

        let cdr = self.read_cons()?;
        Ok(types::RuspExp::Cons {
            car: Box::new(car),
            cdr: Box::new(cdr),
        })
    }

    pub fn read(&mut self) -> anyhow::Result<types::RuspExp> {
        self.skip_whitespace();
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuspErr::ReaderEofError)?;

        match c {
            '\'' => {
                self.input = &self.input[1..]; // skip '\''
                Ok(types::RuspExp::Cons {
                    car: Box::new(types::RuspExp::Atom(types::RuspAtom::Symbol(
                        "quote".to_string(),
                    ))),
                    cdr: Box::new(types::RuspExp::Cons {
                        car: Box::new(self.read()?),
                        cdr: Box::new(types::nil!()),
                    }),
                })
            }
            ':' => {
                self.input = &self.input[1..]; // skip ':'

                if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
                    let s = m.get(0).unwrap().as_str();
                    self.input = &self.input[s.len()..];
                    return Ok(types::RuspExp::Atom(types::RuspAtom::Keyword(
                        s.to_string(),
                    )));
                }

                Ok(types::RuspExp::Atom(types::RuspAtom::Keyword(
                    "".to_string(),
                )))
            }
            '\"' => self.read_string(),
            '(' => {
                self.input = &self.input[1..]; // skip '('
                self.read_cons()
            }
            ')' => {
                self.input = &self.input[1..]; // skip ')'
                Err(anyhow::anyhow!(types::RuspErr::ReaderError))
            }
            _ => self.read_atom(),
        }
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

        let input = "   ";
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

        let input = "1+";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol("1+".to_string())));
    }

    #[test]
    fn test_read_cons() {
        let input = "()";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, types::nil!());

        let input = "(1 2 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Cons {
                car: Box::new(Atom(Int(1))),
                cdr: Box::new(Cons {
                    car: Box::new(Atom(Int(2))),
                    cdr: Box::new(Cons {
                        car: Box::new(Atom(Int(3))),
                        cdr: Box::new(types::nil!()),
                    }),
                }),
            }
        );

        let input = "(1 2 . 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Cons {
                car: Box::new(Atom(Int(1))),
                cdr: Box::new(Cons {
                    car: Box::new(Atom(Int(2))),
                    cdr: Box::new(Atom(Int(3))),
                }),
            }
        );

        let input = "(1 2 . 3";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuspErr::ReaderEofError.to_string());
    }
}
