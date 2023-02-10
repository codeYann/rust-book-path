#[derive(Debug)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    // Self is alias for the type Rectangle is this particular case
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}
