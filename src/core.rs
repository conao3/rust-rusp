#[allow(dead_code)]
#[derive(Debug)]
pub enum RuspErrType {
    EmptyInput,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RuspErr {
    type_: RuspErrType,
    reason: String,
}

fn read(x: String) -> Result<String, RuspErr> {
    Ok(x)
}

fn eval(x: String) -> Result<String, RuspErr> {
    Ok(x)
}

fn print(x: String) -> Result<String, RuspErr> {
    Ok(x)
}

pub fn rep(x: String) -> Result<String, RuspErr> {
    if x.is_empty() {
        return Err(RuspErr {
            type_: RuspErrType::EmptyInput,
            reason: "Empty input".to_string(),
        });
    }

    Ok(print(eval(read(x)?)?)?)
}
