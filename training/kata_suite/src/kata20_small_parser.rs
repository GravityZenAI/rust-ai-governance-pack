//! Kata 20: Small parser.

/// Parse and sum an expression like "1 + 2 + 3".
///
/// Rules:
/// - Only '+' operator.
/// - Whitespace allowed.
/// - Error on empty input, invalid integer, or malformed syntax.
pub fn sum_expr(expr: &str) -> Result<i32, String> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Err("empty input".to_string());
    }
    let mut total = 0i32;
    for part in trimmed.split('+') {
        let part = part.trim();
        if part.is_empty() {
            return Err("malformed expression: empty segment".to_string());
        }
        let n: i32 = part
            .parse()
            .map_err(|e| format!("invalid integer '{part}': {e}"))?;
        total += n;
    }
    Ok(total)
}
