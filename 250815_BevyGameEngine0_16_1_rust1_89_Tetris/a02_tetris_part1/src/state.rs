use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}
