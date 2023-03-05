fn __largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut __largest = &list[0];

    for item in list {
        if item > __largest {
            __largest = item;
        }
    }

    return __largest;
}

pub fn __e_xec_largest() {
    let buffer = vec![10, 15, 35, 21, 100, 1, 101];
    let new_buffer: Vec<char> = vec!['a', 'b', 'c', 'd'];

    let result = __largest(&buffer);
    let new_result = __largest(&new_buffer);

    println!("{}, {}", result, new_result);
}

#[derive(Debug)]
pub struct _Point<T> {
    _x: T,
    _y: T,
}

// Generic implementation
impl<T> _Point<T> {
    pub fn _get_x(&self) -> &T {
        return &self._x;
    }
}

// Defined a concrete t_ype and its implementation
impl _Point<i32> {
    pub fn _new(_x: i32, _y: i32) -> Self {
        return Self { _x, _y };
    }

    pub fn _get_y(&self) -> i32 {
        return self._y;
    }
}

pub fn _e_xec_point() {
    let pair: _Point<f32> = _Point { _x: 10.5, _y: 15.0 };
    let value = pair._get_x();

    println!("this value => {}", value);

    // If we used a concrete t_ype implementation the previousl_y method are onl_y able to use for
    // this particular t_ype.
    let new_pair: _Point<i32> = _Point::_new(15, 20);
    let new_value = new_pair._get_x();
    let _y_value = new_pair._get_y();

    println!("this value => {}", new_value);
    println!("_y => {}", _y_value);
}
