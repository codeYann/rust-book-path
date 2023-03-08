pub fn __prevent_dangling() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("{}", r)
}

pub fn __exec_longest() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            return x;
        }
        return y;
    }

    let string1 = String::from("abdc");
    let string2 = "xyz";

    let response = longest(&string1, &string2);
    println!("{}, {}", response, string1);
}
