pub fn dealing_with_vectors() {
    // When we define a vector we can use Vec::new(), but we need to specify the type of vector
    let mut v: Vec<i32> = vec![15, 320, 215];
    v.push(10);

    let mut u: Vec<f32> = vec![10.0, 231.2, 22.5];
    u.push(0.000);

    if v[0] == 10 {
        println!("They're equals");
    } else {
        println!("They're not equal")
    }

    println!("{:?}, {:?}", v, u);
}

pub fn reading_values_from_vecs() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);

    // First option
    let first_value = &v[0];
    if let 15 = first_value {
        println!("Ohh yeahh!");
    }

    // Second option
    let fv: Option<&i32> = v.get(0);

    let number = Some(&v[0]);

    match number {
        Some(number) => println!("Oh yeah {number}"),
        None => println!("Shit!"),
    };

    match fv {
        Some(fv) => println!("the value is {:?}", fv),
        None => println!("there is no first value"),
    };

    println!("{:?}", v);
}

pub fn borrowing_in_vec() {
    let mut v = vec![1, 2, 3, 4];

    let first = &mut v[0];
    *first = 15;

    let first = &mut v[1];
    *first = 16;

    println!("{:?}", v);
}

pub fn iterating_over_a_vec() {

    let mut v = vec![1, 2, 3, 4];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 1; 
        println!("{}", i);
    }
}

pub fn vecs_and_enums() {

    #[derive(Debug)]
    pub enum Ball {
        BasketBall(String),
        FootBall(String),
        VolleyBall(String),
    }

    let store_ball = vec![
        Ball::BasketBall(String::from("Clapton")),
        Ball::FootBall(String::from("Topper")),
        Ball::VolleyBall(String::from("haikyuu")),
    ];

    for balls in &store_ball {
        match balls {
            Ball::FootBall(v) => println!("Yeahh {v}"),
            Ball::BasketBall(v) => println!("tmr n c {v}"),
            Ball::VolleyBall(v) => println!("even my type {v}"),
        }
    }


}
