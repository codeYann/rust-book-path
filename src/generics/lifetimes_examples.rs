#[derive(Debug)]
pub struct Node<'a, T> {
    pub key: T,
    pub next: Option<&'a mut Node<'a, T>>,
}

impl<'a, T> Node<'a, T> {
    pub fn new(key: T) -> Self {
        return Self { key, next: None };
    }
}

#[derive(Debug)]
pub struct LinkedList<'a, T> {
    pub head: Option<&'a mut Node<'a, T>>,
    pub tail: Option<&'a mut Node<'a, T>>,
    pub size: i32,
}


impl<'a, T> LinkedList<'a,T> {
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            size: 0
        }
    }

    pub fn append(&mut self, key: T) -> Result<T, ()> {
    }
    // add code here
}
