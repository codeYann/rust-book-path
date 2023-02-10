#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
    height: f32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

pub fn __test_struct() {
    let me = Person {
        name: "Yan Rodrigues".to_string(),
        age: 21,
        email: "ccyan05@alu.ufc.br".to_string(),
        height: 1.90,
    };

    let me_2 = Person {
        name: "Yan 2".to_string(),
        age: 21,
        height: 1.90,
        ..me // In this line we move me ownership to me_2
    };

    println!("{:?}", me_2);
    println!(
        "{}, {}, {}, {}",
        me_2.name, me_2.age, me_2.email, me_2.height
    );
}

pub fn __tuple_structs() {
    let mut black = Color(0, 0, 0);
    let blue = &mut black;
    println!("{:?}", blue);
    blue.0 = 1;
    blue.2 = 5;
    println!("{:?}", blue);
    println!("{:?}", black);
}

pub fn __ownership_on_structs() {
    let mut strc = Person {
        name: "Bla".to_string(),
        age: 21,
        email: "2".to_string(),
        height: 2.05,
    };

    let strc2 = &mut strc;
    strc2.email = "Salve salve fml".to_string();

    println!("{:?}", strc);
}
