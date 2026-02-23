//! Kata 04: Structs and methods.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        todo!("construct a new counter with value=0")
    }

    pub fn inc(&mut self) {
        todo!("increment by 1")
    }

    pub fn add(&mut self, delta: i32) {
        todo!("add delta")
    }

    pub fn get(&self) -> i32 {
        todo!("return current value")
    }
}
