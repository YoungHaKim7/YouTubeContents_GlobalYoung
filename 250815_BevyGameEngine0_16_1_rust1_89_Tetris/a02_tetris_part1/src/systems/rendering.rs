use bevy::prelude::*;

use crate::{
    components::{Piece, Position},
    game_constants::{HEIGHT, NUM_BLOCKS_X, NUM_BLOCKS_Y, TEXTURE_SIZE, WIDTH},
    game_types::{GameMap, Presence},
    piece_utils::get_block_matrix,
};

pub fn draw_blocks(
    mut commands: Commands,
    game_map: Res<GameMap>,
    query_piece: Query<(&Piece, &Position)>,
    query_existing_blocks: Query<Entity, With<Sprite>>,
) {
    for entity in query_existing_blocks.iter() {
        commands.entity(entity).despawn();
    }

    for y in 0..NUM_BLOCKS_Y {
        for x in 0..NUM_BLOCKS_X {
            if let Presence::Yes(color) = game_map.0[y][x] {
                commands.spawn((
                    Sprite {
                        color: color.into(),
                        custom_size: Some(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32)),
                        ..default()
                    },
                    Transform::from_xyz(
                        (x as f32 * TEXTURE_SIZE as f32) - (WIDTH as f32 / 2.0)
                            + (TEXTURE_SIZE as f32 / 2.0),
                        (HEIGHT as f32 / 2.0)
                            - (y as f32 * TEXTURE_SIZE as f32)
                            - (TEXTURE_SIZE as f32 / 2.0),
                        0.0,
                    ),
                    Visibility::Visible,
                ));
            }
        }
    }

    if let Ok((piece, position)) = query_piece.single() {
        let piece_matrix = get_block_matrix(piece.states[piece.current_state], piece.color);
        for (my, row) in piece_matrix.iter().enumerate() {
            for (mx, cell) in row.iter().enumerate() {
                if let Presence::Yes(color) = *cell {
                    commands.spawn((
                        Sprite {
                            color: color.into(),
                            custom_size: Some(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32)),
                            ..default()
                        },
                        Transform::from_xyz(
                            ((position.x + mx as isize) as f32 * TEXTURE_SIZE as f32)
                                - (WIDTH as f32 / 2.0)
                                + (TEXTURE_SIZE as f32 / 2.0),
                            (HEIGHT as f32 / 2.0)
                                - ((position.y + my as isize) as f32 * TEXTURE_SIZE as f32)
                                - (TEXTURE_SIZE as f32 / 2.0),
                            0.0,
                        ),
                        Visibility::Visible,
                    ));
                }
            }
        }
    }
}
