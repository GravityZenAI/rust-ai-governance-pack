//! Kata 14: Parsing.

/// Split a CSV line by ',' and trim whitespace around fields.
pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.trim().to_string()).collect()
}

/// Parse "a:b" into (a, b).
pub fn parse_pair(line: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("expected 'a:b', got '{line}'"));
    }
    let a: i32 = parts[0]
        .trim()
        .parse()
        .map_err(|e| format!("bad first number: {e}"))?;
    let b: i32 = parts[1]
        .trim()
        .parse()
        .map_err(|e| format!("bad second number: {e}"))?;
    Ok((a, b))
}
