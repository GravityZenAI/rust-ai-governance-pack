//! Kata 08: Iterators.

pub fn squares(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * x).collect()
}

pub fn sum_even(v: &[i32]) -> i32 {
    v.iter().filter(|x| *x % 2 == 0).sum()
}
