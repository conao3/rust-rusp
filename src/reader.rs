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

    fn read_atom(&mut self) -> anyhow::Result<types::RuspExp<'_>> {
        self.skip_whitespace();

        let int_pattern = regex::Regex::new(r"^([+-]?[0-9]+)(?:[ ();]|$)").unwrap();
        let float_pattern = regex::Regex::new(r"^([+-]?[0-9]*\.[0-9]+)(?:[ ();]|$)").unwrap();
        let symbol_pattern = regex::Regex::new(r"^[^ ();]+").unwrap();

        if let Some(m) = float_pattern.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let f = s.parse::<f64>().unwrap();
            self.input = &self.input[s.len()..];

            // return Ok(types::RuspExp::Atom(types::RuspAtom::Float(f)));
            return Ok(types::RuspExp::Atom(types::RuspAtom::Int(f as i64)));
        }

        if let Some(m) = int_pattern.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let i = s.parse::<i64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(types::RuspExp::Atom(types::RuspAtom::Int(i)));
        }

        if let Some(m) = symbol_pattern.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            self.input = &self.input[s.len()..];
            return Ok(types::RuspExp::Atom(types::RuspAtom::Symbol(s)));
        }

        anyhow::bail!("Failed to parse") // unreachable
    }

    fn read_cons(&mut self) -> anyhow::Result<types::RuspExp<'_>> {
        self.skip_whitespace();

        if self.input.is_empty() {
            anyhow::bail!(types::RuspErr::ReaderEofError);
        }

        if self.input.starts_with(')') {
            self.input = &self.input[1..]; // skip ')'
            return Ok(types::NIL);
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
                car: &car,
                cdr: &cdr,
            });
        }

        let cdr = self.read_cons()?;
        Ok(types::RuspExp::Cons {
            car: &car,
            cdr: &cdr,
        })
    }

    pub fn read(&mut self) -> anyhow::Result<types::RuspExp<'_>> {
        self.skip_whitespace();
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuspErr::ReaderEofError)?;

        match c {
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

        // let input = "42.3";
        // let mut reader = Reader::new(input);
        // let exp = reader.read().unwrap();
        // assert_eq!(exp, Atom(Float(42.3)));

        // let input = "   42.3";
        // let mut reader = Reader::new(input);
        // let exp = reader.read().unwrap();
        // assert_eq!(exp, Atom(Float(42.3)));

        let input = "a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol("a")));

        let input = "   a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol("a")));

        let input = "1+";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Atom(Symbol("1+")));
    }

    #[test]
    fn test_read_cons() {
        let input = "()";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, types::NIL);

        let input = "(1 2 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Cons {
                car: &Atom(Int(1)),
                cdr: &Cons {
                    car: &Atom(Int(2)),
                    cdr: &Cons {
                        car: &Atom(Int(3)),
                        cdr: &types::NIL,
                    },
                },
            }
        );

        let input = "(1 2 . 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Cons {
                car: &Atom(Int(1)),
                cdr: &Cons {
                    car: &Atom(Int(2)),
                    cdr: &Atom(Int(3)),
                },
            }
        );

        let input = "(1 2 . 3";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuspErr::ReaderEofError.to_string());
    }
}
