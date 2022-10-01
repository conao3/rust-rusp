use crate::types;

fn read(x: String) -> Result<String, types::RuspErr> {
    Ok(x)
}

fn eval(x: String) -> Result<String, types::RuspErr> {
    Ok(x)
}

fn print(x: String) -> Result<String, types::RuspErr> {
    Ok(x)
}

pub fn rep(x: String) -> Result<String, types::RuspErr> {
    if x.is_empty() {
        return Err(types::RuspErr {
            type_: types::RuspErrType::EmptyInput,
            reason: "Empty input".to_string(),
        });
    }

    Ok(print(eval(read(x)?)?)?)
}
