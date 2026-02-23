//! Kata 11: Enums + match.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Add(i32),
    Sub(i32),
    Reset,
}

pub fn apply(cmds: &[Command]) -> i32 {
    let mut acc = 0;
    for cmd in cmds {
        match cmd {
            Command::Add(n) => acc += n,
            Command::Sub(n) => acc -= n,
            Command::Reset => acc = 0,
        }
    }
    acc
}
