//! Kata 03: Result / Option.

/// Parse a positive i32 from string.
///
/// Rules:
/// - Return Err with a helpful message on parse failure.
/// - Return Err if the number is <= 0.
pub fn parse_positive_i32(s: &str) -> Result<i32, String> {
    todo!("parse and validate")
}

/// Safe integer division.
///
/// Returns None if `b == 0`.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    todo!("implement safe_div")
}
