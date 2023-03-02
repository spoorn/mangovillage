use bevy::prelude::Component;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}