use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn make_guess() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut chances = 5;

    loop {
        if chances == 0 {
            break;
        }

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        chances = chances - 1;

        let guess_number: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess_number);

        println!("Please type a number");
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
