//! Kata 01: Borrowing basics.

/// Return the first word of `s` (split by ASCII whitespace).
///
/// Constraints:
/// - Must return a slice (`&str`) into the input (no allocation).
pub fn first_word(s: &str) -> &str {
    s.split_ascii_whitespace().next().unwrap_or("")
}

/// Count words in `s` using whitespace splitting.
pub fn count_words(s: &str) -> usize {
    s.split_ascii_whitespace().count()
}
