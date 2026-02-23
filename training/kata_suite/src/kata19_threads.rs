//! Kata 19: Threads (basic concurrency).

use std::thread;

/// Sum numbers using up to `n_threads` threads.
///
/// Constraints:
/// - Must not use global mutable state.
/// - Must be deterministic.
pub fn parallel_sum(nums: Vec<i32>, n_threads: usize) -> i32 {
    if nums.is_empty() || n_threads == 0 {
        return 0;
    }
    let chunk_size = nums.len().div_ceil(n_threads);
    let mut handles = Vec::new();
    for chunk in nums.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<i32>()));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
