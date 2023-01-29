mod guessing_game;

fn greetings(a: String) {
    println!("this is my string: {}", a);
}

fn main() {
    println!("Hello, world!");
    greetings("My name is Yan!".to_string());
    guessing_game::make_guess();
}
