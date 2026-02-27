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

// --- Level 2: Deliberate challenge ---
// Common AI mistake: using unwrap() instead of proper error propagation.

/// Parse two strings as i32, add them, and divide by a third parsed string.
/// Must propagate ALL errors without unwrap/expect.
///
/// Returns Err if any parse fails or if divisor is zero.
pub fn parse_add_divide(a: &str, b: &str, divisor: &str) -> Result<i32, String> {
    let va: i32 = a.parse().map_err(|e| format!("parse a: {e}"))?;
    let vb: i32 = b.parse().map_err(|e| format!("parse b: {e}"))?;
    let vd: i32 = divisor.parse().map_err(|e| format!("parse divisor: {e}"))?;
    if vd == 0 {
        return Err("division by zero".to_string());
    }
    Ok((va + vb) / vd)
}
