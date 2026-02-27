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

// --- Level 2: Deliberate challenge ---
// The AI must process the string twice without conflicting borrows.
// Common AI mistake: trying to hold an iterator while also iterating.

/// Return the longest word AND the total character count (no whitespace).
///
/// Constraints:
/// - Must return a slice (`&str`) into the input (no allocation for the word).
/// - Must NOT clone or allocate the input string.
pub fn longest_word_and_char_count(s: &str) -> (&str, usize) {
    let char_count = s.chars().filter(|c| !c.is_ascii_whitespace()).count();
    let longest = s
        .split_ascii_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("");
    (longest, char_count)
}
