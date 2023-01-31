// if i want to use a const i nedd to specify data type and writting it in upper case
pub const TREE_HOURS: u32 = 60*60*3;

pub fn vars_mut() {
    let mut x = 10;
    println!("x = {}", x);

    x = 15;

    println!("x = {}, {}", x, TREE_HOURS);
}

pub fn shadowing() {
    // It's possible to use the same name and have a other type using shadowing.
    // Under the hood rust creates a new var and bound the new value but reusing the same name
    let message = "ola mundo!";

    println!("{}", message);

    {
        let message = 15.2 + 5.0;
        println!("{}", message);
    }

    println!("{}", message);
}
