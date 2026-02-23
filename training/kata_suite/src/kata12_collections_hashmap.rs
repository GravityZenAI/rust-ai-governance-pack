//! Kata 12: Collections (HashMap).

use std::collections::HashMap;

pub fn word_frequencies(s: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in s.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}
