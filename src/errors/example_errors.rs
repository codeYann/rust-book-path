use std::num::ParseIntError;

#[derive(Debug)]
pub struct SummationError;

fn to_int(s: &str) -> Result<i32, ParseIntError> {
    return s.parse();
}

fn sum_str_vec(strs: &Vec<String>) -> Result<String, SummationError> {
    let mut acc: i32 = 0;

    for str in strs {
        acc += to_int(&(*str)).map_err(|_| SummationError)?;
    }

    return Ok(acc.to_string());
}

pub fn __exec_it() {
    let buffer = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let value = sum_str_vec(&buffer);
    println!("{:?}, {:?}", value, buffer);

    let buffer = vec!["abc".to_string()];

    let value = sum_str_vec(&buffer);
    println!("{:?}, {:?}", value, buffer);
}
