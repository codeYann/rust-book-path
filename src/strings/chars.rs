pub fn _chars() {
    let l0 = 'Y';
    let l1 = 'a';
    let l2 = 'N';

    println!("{l0}, {l1}, {l2}");
}

pub fn _strs() {
    // str slice
    let nome: &str = "Yan"; // returns a pointer to a memory block [start, length]
    let mut nome = nome.to_string(); // convert a str slice to a String

    // The main difference between str slice and String is where we define both.
    // str slice is defined in static memory space
    // String is define in heap memory space

    nome.push_str(" Rodrigues"); // We can use push_str to append a new str slice to nome var
    println!("{nome}")
}
