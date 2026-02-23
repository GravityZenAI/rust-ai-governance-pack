//! Kata 03: Result / Option.

/// Parse a positive i32 from string.
///
/// Rules:
/// - Return Err with a helpful message on parse failure.
/// - Return Err if the number is <= 0.
pub fn parse_positive_i32(s: &str) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|e| format!("parse error: {e}"))?;
    if n <= 0 {
        return Err(format!("expected positive, got {n}"));
    }
    Ok(n)
}

/// Safe integer division.
///
/// Returns None if `b == 0`.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
