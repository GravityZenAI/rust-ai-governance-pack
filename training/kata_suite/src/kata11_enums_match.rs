//! Kata 11: Enums + match.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Add(i32),
    Sub(i32),
    Reset,
}

pub fn apply(cmds: &[Command]) -> i32 {
    todo!("apply commands starting from 0")
}
