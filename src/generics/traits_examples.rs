// Traits defines a "contract" that some type must do
pub trait MyTrait {
    fn sum(&self) -> Result<i32, ()>;
}

// It's possible to use traits with enums
#[derive(Debug)]
pub enum MyEnum {
    Counter(i32),
}

// In this particular trait, sum method returns a Result<i32, ()>
// if the value of Counter(v) is less than 0 sum returns a Err(())
// if not then sum returns Ok(v)

// it's clear that MyEnum should be MyStruct because why do someone creates a enum with one
// possibility?
impl MyTrait for MyEnum {
    fn sum(&self) -> Result<i32, ()> {
        match &self {
            MyEnum::Counter(v) => {
                if *v < 0 {
                    return Err(());
                }

                let mut acc = 0;
                for i in 0..=*v {
                    acc += i;
                }

                return Ok(acc);
            }
        }
    }
}
