// I can define an inline mod and use it only in main.rs file
// When a module is defined the compiler will look for module's code in this order:
// inline
// src/garden.rs
// src/garden/mod.rs -> That's the way i've been using

mod garden {

    pub fn green() -> i32 {
        5
    }
    // It's possible to create a sub module inside a module
    pub mod veg {
        #[derive(Debug)]
        pub struct Field {}
        pub fn creating_it() -> Field {
            return Field {};
        }

        pub fn shows_it() {
            let var = super::green();
            println!("{}", var);
        }
    }

    pub fn creating_it() -> String {
        return "ola mundo".to_string();
    }
}

use garden::veg;

fn main() {
    let it = veg::Field {};
    println!("Hello, World!");
    let value = garden::creating_it();
    println!("{}", &value);
    let value = garden::veg::creating_it();
    println!("{:?}", &value);
    println!("{:?}", it);

    veg::shows_it();
}
