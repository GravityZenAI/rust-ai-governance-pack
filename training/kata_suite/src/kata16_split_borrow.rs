//! Kata 16: Split borrows (avoid E0499).

/// If both indices are in-bounds and distinct, return the sum of the two values.
/// Also increment both elements by 1.
///
/// Return None if indices are out of range or equal.
pub fn sum_and_bump_two(v: &mut [i32], i: usize, j: usize) -> Option<i32> {
    todo!("use split_at_mut to get two &mut safely")
}
