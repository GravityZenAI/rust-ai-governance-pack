//! Kata 16: Split borrows (avoid E0499).

/// If both indices are in-bounds and distinct, return the sum of the two values.
/// Also increment both elements by 1.
///
/// Return None if indices are out of range or equal.
pub fn sum_and_bump_two(v: &mut [i32], i: usize, j: usize) -> Option<i32> {
    if i == j || i >= v.len() || j >= v.len() {
        return None;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let (left, right) = v.split_at_mut(hi);
    let sum = left[lo] + right[0];
    left[lo] += 1;
    right[0] += 1;
    Some(sum)
}
