#[derive(Debug)]
pub enum _Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn _value_in_cents(coin: &_Coins) -> u8 {
    return match coin {
        &_Coins::Penny => 1,
        &_Coins::Nickel => 5,
        &_Coins::Dime => 10,
        &_Coins::Quarter => 25,
    };
}

pub fn _matching_things() -> String {
    let name = "";
    return match name {
        "name" => String::from("Ai calica"),
        &_ => String::from("Dude"),
    };
}

pub fn _if_let_short_hand() {
    let value = _Coins::Penny;
    if let _Coins::Penny = value {
        println!("I told long ago");
    } else {
        println!("You're wrong!");
    }
}
