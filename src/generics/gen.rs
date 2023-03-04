fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub fn _exec_largest() {
    let buffer = vec![10, 15, 35, 21, 100, 1, 101];
    let new_buffer: Vec<char> = vec!['a', 'b', 'c', 'd'];

    let result = largest(&buffer);
    let new_result = largest(&new_buffer);

    println!("{}, {}", result, new_result);
}

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

// Generic implementation
impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        return &self.x;
    }
}

// Defined a concrete type and its implementation
impl Point<i32> {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }

    pub fn get_y(&self) -> i32 {
        return self.y
    } 
}

pub fn _exec_point() {
    let pair: Point<f32> = Point { x: 10.5, y: 15.0 };
    let value = pair.get_x();

    println!("this value => {}", value);


    // If we used a concrete type implementation the previously method are only able to use for
    // this particular type.
    let new_pair: Point<i32> = Point::new(15, 20);
    let new_value = new_pair.get_x();
    let y_value = new_pair.get_y();

    println!("this value => {}", new_value);
    println!("y => {}", y_value);
}
