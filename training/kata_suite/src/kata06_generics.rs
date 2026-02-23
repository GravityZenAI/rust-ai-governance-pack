//! Kata 06: Generics.

pub fn max_of<T: Ord + Copy>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

/// Given a sorted slice, return a new Vec with consecutive duplicates removed.
pub fn dedup_sorted<T: PartialEq + Copy>(v: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for &item in v {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    result
}
