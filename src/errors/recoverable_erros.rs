use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn __recoverable_erros() {
    let greetings_file_result = File::open("hello.txt");

    let content = match greetings_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("shit {:?}", e),
            },
            errs => {
                panic!("problem with this shit {:?}", errs);
            }
        },
    };

    println!("{:?}", content);
}

pub fn __using_unwrap_shortcut() {
    // Unwrap method is a shortcut method implemented using match expression
    // if the Result is the Ok variant then unwrap returns the value inside Ok
    // if the Result is the Err variant then unwrap will call panic! macro
    let greetings_file_result = File::open("hello.txt").unwrap();
    println!("{:?}", greetings_file_result);
}

pub fn __propagating_errors() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn __propagating_errors_using_question_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

pub fn __using_question_operator() -> Result<i32, String>{

    pub fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err("Cannot divide by 0".to_string());
        }
        return Ok(a / b);
    }

    let response = divide(10, 1)?;
    println!("{}", response);

    Ok(10)
}
