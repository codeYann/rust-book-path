mod enums;

fn main() {
    println!("Hello, World!");

    let enumeration = enums::pattern_match::_Coins::Dime;
    let result = enums::pattern_match::_value_in_cents(&enumeration);
    println!("{:?}, {}", enumeration, result);

    let enumeration = enums::pattern_match::_Coins::Nickel;
    let result = enums::pattern_match::_value_in_cents(&enumeration);
    println!("{:?}, {}", enumeration, result);

    let enumeration = enums::pattern_match::_Coins::Penny;
    let result = enums::pattern_match::_value_in_cents(&enumeration);
    println!("{:?}, {}", enumeration, result);

    let enumeration = enums::pattern_match::_Coins::Quarter;
    let result = enums::pattern_match::_value_in_cents(&enumeration);
    println!("{:?}, {}", enumeration, result);

}
