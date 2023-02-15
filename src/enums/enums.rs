#[derive(Debug)]
pub enum _IpAddrKind {
    _V4(i32),
    _V6(i32),
}

pub fn __exec_ip_addr_kind() {
    let four = _IpAddrKind::_V4(127_000_1);
    let six = _IpAddrKind::_V6(012_125_221);

    println!("{:?}, {:?}", four, six);
}

#[derive(Debug)]
pub enum _Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

impl _Message {
    pub fn _call(&self) {
        println!("ola mundo");
    }
}
