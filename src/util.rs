#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum UtliErr {
    #[error("NoSameLengthIteratorError")]
    NoSameLengthIteratorError,
}

#[derive(Clone, Debug)]
pub struct SafeZipEq<I, J> {
    a: I,
    b: J,
}

pub fn safe_zip_eq<I, J>(i: I, j: J) -> SafeZipEq<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    SafeZipEq {
        a: i.into_iter(),
        b: j.into_iter(),
    }
}

impl<I, J> Iterator for SafeZipEq<I, J>
where
    I: Iterator,
    J: Iterator,
{
    type Item = anyhow::Result<(I::Item, J::Item)>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (Some(a), Some(b)) => Some(Ok((a, b))),
            (None, Some(_)) | (Some(_), None) => {
                Some(Err(anyhow::anyhow!(UtliErr::NoSameLengthIteratorError)))
            }
        }
    }
}

impl<I, J> ExactSizeIterator for SafeZipEq<I, J>
where
    I: ExactSizeIterator,
    J: ExactSizeIterator,
{
}
