//! Kata 06: Generics.

pub fn max_of<T: Ord + Copy>(a: T, b: T) -> T {
    todo!("return max")
}

/// Given a sorted slice, return a new Vec with consecutive duplicates removed.
pub fn dedup_sorted<T: PartialEq + Copy>(v: &[T]) -> Vec<T> {
    todo!("dedup consecutive duplicates")
}
