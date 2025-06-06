use crate::gameplay::ply::Turn;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Player(pub Turn);

impl Player {
    pub const fn chance() -> Self {
        Self(Turn::Chance)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self(Turn::Choice(0))
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Turn::Chance => write!(f, "??"),
            Turn::Choice(0) => write!(f, "P0"),
            Turn::Choice(_) => write!(f, "P1"),
            Turn::Terminal => write!(f, "END"),
        }
    }
}
