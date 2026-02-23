//! Kata 04: Structs and methods.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn add(&mut self, delta: i32) {
        self.value += delta;
    }

    pub fn get(&self) -> i32 {
        self.value
    }
}
