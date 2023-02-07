pub fn _on() {
    let a = String::from("Yanzada!");
    let b = a;

    if b.is_empty() {
        println!("No");
    } else {
        println!("Yes");
    }
    println!("{}", b);

}

pub fn __say_hello(msg: &String) -> () {
    println!("Hello, {}", msg);
}

pub fn __say_goodbye(msg: &String) -> () {
    println!("I'm no good at goodbyes, {}", msg);
}

pub fn __to_uppercase(text: &mut String) {
    *text = text.to_uppercase();
}
