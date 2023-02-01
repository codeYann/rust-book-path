// To use a const you need to use SCREAMING_SNAKE pattern and define a type for this const
const SECONDS_IN_MINUTE: u32 = 60;
const MINUTE_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTE_IN_HOUR;

/*
 * Rust uses RAII approach to deal with memory and manger resources.
 * If you try to declare a variable in global scopes rust won't compile
 * because all resources need to have a scopes.
 * It's really important to know about resources life time
*/

pub fn _variable_shadowing() {
    let total = 30;
    println!("a number: {}", total);
    let total = "trinta!";
    println!("a string: {}", total);
}

pub fn _variable_shadowing_scope() {
    let name = "Yanzin";
    println!("My name is {}", name);
    {
        // Using shadowing in a new scope to change name and reusing var name
        let name = "Zuluzão";
        println!("Sorry! my real name is {}", name);
    }
}

pub fn _calc_time() {
    let total = 30;
    {
        let total = total * SECONDS_IN_HOUR;
        println!("{}", total);
    }
    println!("{}", total);
}
