//! Kata 17: From / Into.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kilometers(pub u32);

impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        todo!("1 km = 1000 m")
    }
}
