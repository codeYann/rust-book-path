// Traits defines a "contract" that some type must do
pub trait _MyTrait {
    fn _sum(&self) -> Result<i32, ()>;
}

// It's possible to use traits with enums
#[derive(Debug)]
pub enum _MyEnum {
    _Counter(i32),
}

// In this particular trait, _sum method returns a Result<i32, ()>
// if the value of _Counter(v) is less than 0 _sum returns a Err(())
// if not then _sum returns Ok(v)

// it's clear that _MyEnum should be MyStruct because why do someone creates a enum with one
// possibility?
impl _MyTrait for _MyEnum {
    fn _sum(&self) -> Result<i32, ()> {
        match &self {
            _MyEnum::_Counter(v) => {
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
