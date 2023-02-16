#[derive(Debug)]

pub struct Custom {
    name: String,
    age: usize,
}

pub enum Item {
    Foo(String),
    Bar(usize),
    Baz(Custom),
}

pub fn __exec_enum_example() {
    let _foo = Item::Foo(String::from("hello"));

    if let Item::Foo(s) = Item::Foo(String::from("world")) {
        println!("{}", s);
    }

    let other_value = Item::Baz(Custom {
        name: "Yanzada".to_string(),
        age: 21,
    });

    if let Item::Baz(custom) = other_value {
        println!("{}, {}", custom.name, custom.age);
    }

    let value: usize = 21;

    if let Item::Bar(number) = Item::Bar(value) {
        println!("{}", number);
    }
}

#[derive(Debug)]
pub enum Bar {
    Music,
}

pub fn do_something(value: &Bar) {
    println!("{:?}", value);
}

pub fn __test_enum_() {
    let v = Bar::Music;
    do_something(&v);
    println!("{:?}", v);
}
