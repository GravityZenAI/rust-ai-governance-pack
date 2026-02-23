//! Kata 09: Error propagation with `?`.

pub fn read_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse::<i32>()
}

pub fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(read_number(a)? + read_number(b)?)
}
