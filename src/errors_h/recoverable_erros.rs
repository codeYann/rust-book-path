use std::fs::File;
use std::io::{self, Read, ErrorKind};

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

pub fn __errors_on_functions(position: usize, buffer: &Vec<i32>) -> Option<&i32> {
    return match buffer.get(position) {
        Some(it) => Some(it),
        None => return None,
    }
}
