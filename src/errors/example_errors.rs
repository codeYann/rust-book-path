fn _to_int(s: &str) -> Option<i32> {
    return s.parse().ok();
}

fn _sum_str_vec(strs: &Vec<String>) -> String {
    let mut acc: i32 = 0;

    for s in strs {
        acc += match _to_int(&s) {
            Some(number) => number,
            None => 0,
        }
    }

    return acc.to_string();
}
