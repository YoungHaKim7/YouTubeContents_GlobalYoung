use bevy::prelude::*;

use crate::{
    game_color::GameColor,
    game_constants::{HEIGHT, TITLE, WIDTH},
    game_types::GameMap,
    state::GameState,
    systems::{rendering::draw_blocks, setup::setup_camera, spawning::spawn_initial_piece},
};

mod components;
mod game_color;
mod game_constants;
mod game_types;
mod piece_utils;
mod state;
mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(GameColor::Gray.into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIDTH as f32, HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameMap>()
        .insert_state(GameState::Playing)
        .add_systems(Startup, (setup_camera, spawn_initial_piece))
        .add_systems(Update, draw_blocks)
        .run();
}
