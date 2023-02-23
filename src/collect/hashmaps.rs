use std::collections::HashMap;

pub fn __creating_new_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Yan".to_string(), 21);
    scores.insert("aYn".to_string(), 20);
    scores.insert("anY".to_string(), 52);
    println!("{:?}", scores);

    // Using pattern match to handle .get method
    let value = scores.get("anY");
    match value {
        Some(v) => println!("{}", v),
        None => println!("shit"),
    };
}

pub fn __iterating_over_hashmap() {
    let mut table: HashMap<char, u8> = HashMap::new();
    table.insert('a', 97);
    table.insert('b', 98);
    table.insert('c', 99);
    table.insert('d', 100);

    for (key, _) in &table {
        println!("{}, {}", key, *key as u8);
    }

    let it = 'b';
    let value = table.get(&it);

    match value {
        Some(x) => println!("{}", x),
        None => println!("it was nothing"),
    };
}

pub fn __updating_hash_map() {
    let mut table: HashMap<String, u8> = HashMap::new();
    table.insert("random string".to_string(), 95);
    table.insert("strings are amazing".to_string(), 99);
    table.insert("email".to_string(), 95);

    for (key, value) in &table {
        println!("{key}, {value}");
    }

    table.insert("email".to_string(), 105);

    for (key, value) in &table {
        println!("{key}, {value}");
    }
}

pub fn __entry_method() {
    let mut table: HashMap<String, u8> = HashMap::new();
    table.insert("cut it".to_string(), 95);
    table.insert("i guess i better of alone".to_string(), 99);
    table.entry("email".to_string()).or_insert(105);

    for (key, value) in &table {
        println!("{key}, {value}");
    }

    // or_insert returns a mutable reference to "email" key
    let value = table.entry("email".to_string()).or_insert(106);
    *value = 104;

    for (key, value) in &table {
        println!("{key}, {value}");
    }
}

pub fn __updating_based_old_values() {
    let text = "hello hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
