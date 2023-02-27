pub fn _dealing_with_string_type() {
    let mut s = String::new();
    s.push_str("alou, mundo");
    println!("{}", s);

    let data = "this is it";
    let wrapper = data.to_string();

    println!("{data}, {wrapper}");
}

pub fn _concat_with_plus_operator() {
    let s1 = String::from("This world");
    let s2 = String::from(" is beautiful");
    let s3 = format!("{s1}{s2}");

    println!("{s1}");

    println!("{s3}");
}

pub fn _slicing_string() {
    let s1 = String::from("This world");

    let s2 = &s1[0..4];
    let s3 = &s1[0..=2];

    println!("{s2} {s3}");
}

pub fn _iterating_over_strings() {
    let s1 = "Здравствуйте".to_string();
    let mut empty_string = String::new();

    empty_string.push_str(&s1[0..]);
    

    println!("{empty_string}");
}
