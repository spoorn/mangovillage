use std::fmt::{Display, Formatter};
use bevy::prelude::Component;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}