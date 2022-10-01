#[allow(dead_code)]
#[derive(Debug)]
pub enum RuspErrType {
    EmptyInput,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RuspErr {
    pub type_: RuspErrType,
    pub reason: String,
}
