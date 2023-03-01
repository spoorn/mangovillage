use serde::{Deserialize, Serialize};

pub mod components;
pub mod util;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}