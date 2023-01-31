use rand::Rng; // Using rand lib for generate a random number
use std::cmp::Ordering;
use std::io;

pub fn make_guess() {
    println!("guess the number!");

    // Generate random number in [1, 100]
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut chances = 5;

    loop {
        println!("You got {} chanes", chances);

        if chances == 0 {
            println!("Game over! :(");
            break;
        }

        // Define a mutable string
        let mut guess_str = String::new();

        // Read a text from stdin and appending it to guess_str buffer
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        chances = chances - 1;

        // Parsing guess_str to u32 number and checking if input is valid!
        let guess_number: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess_number);

        println!("Please type a number");
        // Comparing the user input and secret number
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => println!("Too big!!"),
        }
    }
}
