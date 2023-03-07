use std::fmt::{Display, Formatter};
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}