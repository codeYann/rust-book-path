#[derive(Debug)]
pub enum IpAddrKind {
    V4(i32),
    V6(i32),
}

pub fn __exec_ip_addr_kind() {
    let four = IpAddrKind::V4(127_000_1);
    let six = IpAddrKind::V6(012_125_221);

    println!("{:?}, {:?}", four, six);
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        println!("ola mundo");
    }
}
