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
