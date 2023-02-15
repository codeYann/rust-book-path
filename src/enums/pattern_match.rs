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
