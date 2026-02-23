//! Kata 13: Slices & Strings.

pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

/// ASCII palindrome check (case-insensitive, alphanumeric only).
pub fn is_ascii_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}
