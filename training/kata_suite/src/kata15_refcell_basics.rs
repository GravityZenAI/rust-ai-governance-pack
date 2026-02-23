//! Kata 15: RefCell (interior mutability).

use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Bag {
    items: RefCell<Vec<i32>>,
}

impl Bag {
    pub fn new() -> Self {
        todo!("create empty bag")
    }

    pub fn push(&self, x: i32) {
        todo!("push into inner vec")
    }

    pub fn sum(&self) -> i32 {
        todo!("sum items without violating borrow rules")
    }
}
