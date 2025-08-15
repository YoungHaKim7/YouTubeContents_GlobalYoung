use bevy::prelude::*;

use crate::{
    components::{Piece, Position},
    game_constants::NUM_BLOCKS_X,
    game_types::GameMap,
    state::GameState,
};

pub fn spawn_piece(
    commands: &mut Commands,
    game_map: &GameMap,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    let new_piece = Piece::random();
    let initial_position = Position {
        x: NUM_BLOCKS_X as isize / 2 - 1,
        y: 0,
    };

    commands.spawn((new_piece, initial_position));
}

pub fn spawn_initial_piece(
    mut commands: Commands,
    game_map: Res<GameMap>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    spawn_piece(&mut commands, &game_map, &mut game_state);
}
