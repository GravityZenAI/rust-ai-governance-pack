//! Kata 05: Traits.

pub trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub r: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
