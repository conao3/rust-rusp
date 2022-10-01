use crate::types;

fn read(x: &str) -> Result<&str, types::RuspErr> {
    Ok(x)
}

fn eval(x: &str) -> Result<&str, types::RuspErr> {
    Ok(x)
}

fn print(x: &str) -> Result<&str, types::RuspErr> {
    Ok(x)
}

pub fn rep(x: &str) -> Result<&str, types::RuspErr> {
    if x.is_empty() {
        return Err(types::RuspErr {
            type_: types::RuspErrType::EmptyInput,
            reason: "Empty input".to_string(),
        });
    }

    Ok(print(eval(read(x)?)?)?)
}
