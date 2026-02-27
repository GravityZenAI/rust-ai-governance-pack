//! Kata 02: Ownership vs borrowing.

/// Takes ownership of `s`, appends `suffix`, and returns the owned String.
pub fn push_suffix(mut s: String, suffix: &str) -> String {
    s.push_str(suffix);
    s
}

/// Appends `suffix` to `s` in place.
pub fn append_suffix_in_place(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

// --- Level 2: Deliberate challenge ---
// Common AI mistake: using a moved value after a conditional transfer.

/// If `prefix` is non-empty, prepend it to `base` and return the combined string.
/// Otherwise, return `base` unchanged.
///
/// Constraints:
/// - Must take ownership of `base` (no borrowing).
/// - Must NOT clone `base`.
pub fn maybe_prepend(base: String, prefix: &str) -> String {
    if prefix.is_empty() {
        base
    } else {
        let mut result = String::with_capacity(prefix.len() + base.len());
        result.push_str(prefix);
        result.push_str(&base);
        result
    }
}
