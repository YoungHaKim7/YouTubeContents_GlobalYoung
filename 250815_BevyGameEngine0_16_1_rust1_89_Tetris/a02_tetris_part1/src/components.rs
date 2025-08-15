use bevy::ecs::component::Component;

use crate::game_color::GameColor;

#[derive(Component, Default, Copy, Clone)]
pub struct Piece {
    pub states: [u16; 4],
    pub color: GameColor,
    pub current_state: usize,
}

#[derive(Component, Default, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}
