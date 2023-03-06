mod generics;
use generics::traits_examples::{MyEnum, MyTrait};

// Writting a function to consume this particular .sum() function
// It's important to notice that consume_this_trait receive a &impl MyTrait as a param
// in other words any type that implements MyTrait is able to use this function
// also consume_this_trait returns a Result<i32, ()> that means that we can use question mark
// operator
fn consume_this_trait(it: &impl MyTrait) -> Result<i32, ()> {
    let v = it.sum()?;
    return Ok(v);
}

fn main() -> Result<(), ()> {
    let v = MyEnum::Counter(15);
    let it = consume_this_trait(&v)?;

    println!("{it}");
    return Ok(());
}
