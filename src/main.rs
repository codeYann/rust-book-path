// mod errors_h;
// use errors_h::recoverable_erros;

fn to_int(s: &str) -> Option<i32> {
    return s.parse().ok();
}

fn sum_str_vec(strs: &Vec<String>) -> String {
    let mut acc: i32 = 0;

    for s in strs {
        acc += match to_int(&s) {
            Some(number) => number,
            None => 0,
        }
    }

    return acc.to_string();
}

fn main() {
    let buffer = vec![
        "Once upon a time".to_string(),
        "1".to_string(),
        "2".to_string(),
    ];
    let value = sum_str_vec(&buffer);
    println!("{}", value);
}
