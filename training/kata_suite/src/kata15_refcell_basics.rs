//! Kata 15: RefCell (interior mutability).

use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct Bag {
    items: RefCell<Vec<i32>>,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, x: i32) {
        self.items.borrow_mut().push(x);
    }

    pub fn sum(&self) -> i32 {
        self.items.borrow().iter().sum()
    }
}
