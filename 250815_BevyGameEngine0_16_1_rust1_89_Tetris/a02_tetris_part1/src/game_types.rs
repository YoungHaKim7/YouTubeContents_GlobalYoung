use bevy::prelude::*;

use crate::{
    game_color::GameColor,
    game_constants::{NUM_BLOCKS_X, NUM_BLOCKS_Y},
};

#[rustfmt::skip]
pub type PieceMatrix = [
    [Presence; 4]; 4
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    L,
    J,
    S,
    Z,
    T,
    I,
    O,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Presence {
    No,
    Yes(GameColor),
}

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<Presence>>);

impl Default for GameMap {
    fn default() -> Self {
        GameMap(vec![vec![Presence::No; NUM_BLOCKS_X]; NUM_BLOCKS_Y])
    }
}

impl GameMap {}
