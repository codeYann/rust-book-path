pub fn _scalars() {
    let local = 127_u8; // It's possible to define a u8 var with 127 as a value
    let byte = b'a';
    let binary = 0b1111 << 3;
    let hex = 0xFFF;
    println!("{}, {}, {}, {}", local, byte, binary, hex);
}

pub fn _chars() {
    // A char in rust holds 4 bytes!
    let rust = '🦀';
    println!("{}", rust);
}
