//! Kata 10: Modules.

mod inner {
    /// Multiply by 3.
    pub fn helper(x: i32) -> i32 {
        x * 3
    }
}

/// Public API should use the internal module.
pub fn public_api(x: i32) -> i32 {
    inner::helper(x)
}
