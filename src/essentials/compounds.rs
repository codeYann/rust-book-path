pub fn _tuples() {
    let mut numbers: (i32, i64, f32, char) = (10, 12212212, 23213.5, '🦀');
    println!("{:?}", numbers);

    numbers.0 = 12;
    numbers.1 = 12312312312;
    numbers.2 = 1.21;
    println!("{:?}", numbers);
}

pub fn _arrays() {
    let mut infos: [&str; 4] = ["alora", "alora2", "alora3", "alora4"];
    println!("{:?}", infos);
    infos = ["alora4", "alora3", "alora2", "alora"];
    println!("{:?}", infos);
    let new_infos = &infos[0..=1]; // slicing an array using ..=
    println!("{:?}", new_infos);
}
