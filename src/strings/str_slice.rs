pub fn _str_slice() {
    /*
     * In this case we're using a str slice which is a string that is allocate in static memory
     * space.
     * Under the hood rust returns a pointer to a memory block that's the reason the annotate
     * literal_str as &str type.
     * */
    let literal_str: &str = "This is my first literal string! \n";
    let literal_str = literal_str.trim(); // removes white spaces

    println!("{}", literal_str);
}

pub fn _str_string() {
    // In this particular case we define a String type which is a string allocated in heap
    // memory which means that string is possible to encrease and shrink its size
    let mut name = "Yan Rodrigues".to_string();
    name.push_str(" Aqueles picks");
    name.push('\n');

    let name = &name; // Using shadowing to mapping from String to &st

    println!("{}", name);
}
