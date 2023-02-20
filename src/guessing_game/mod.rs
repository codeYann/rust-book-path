use rand::Rng; // Using rand lib for generate a random number
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    // add code here
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be in [1, 100] range");
        }
        return Self { value };
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

pub fn _make_guess() {
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
        let guess_number: i32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let final_guessed_number = Guess::new(guess_number);

        println!("You guessed: {}", final_guessed_number.value());

        println!("Please type a number");
        // Comparing the user input and secret number
        match final_guessed_number.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => println!("Too big!!"),
        }
    }
}
