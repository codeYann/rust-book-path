fn _takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn _make_explicity_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn change_str(some_string: &mut String) {
    some_string.push_str("alora");
}

pub fn _exec_ownership() {
    let s1 = String::from("ola mundo!");
    let number: i32 = 150;

    /* When we passing s1 to _takes_ownership function
     * s1 moves to some_string var
     * remeber that some_string is a valid var allocated in _takes_ownership scope
     * Which means that s1 moves ownership to some_string
     *
     * To fix this problem we can borrow s1 ownership to some_string var
     */
    _takes_ownership(&s1);

    println!("{}", s1);

    _make_explicity_copy(number);

    println!("{}", number);
}
