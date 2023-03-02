mod errors;
use errors::recoverable_erros;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let value = recoverable_erros::__using_question_operator()?;

    println!("{}", value);

    return Ok(());
}
