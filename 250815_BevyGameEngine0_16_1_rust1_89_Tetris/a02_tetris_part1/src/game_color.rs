use bevy::prelude::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum GameColor {
    #[default]
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Purple,
    Gray,
    Pink,
}

impl From<GameColor> for Color {
    fn from(game_color: GameColor) -> Self {
        match game_color {
            GameColor::Red => Color::srgb_u8(255, 0, 0),
            GameColor::Green => Color::srgb_u8(0, 255, 0),
            GameColor::Blue => Color::srgb_u8(0, 0, 255),
            GameColor::Yellow => Color::srgb_u8(255, 255, 0),
            GameColor::Cyan => Color::srgb_u8(0, 255, 255),
            GameColor::Orange => Color::srgb_u8(255, 165, 0),
            GameColor::Purple => Color::srgb_u8(128, 0, 128),
            GameColor::Gray => Color::srgb_u8(128, 128, 128),
            GameColor::Pink => Color::srgb_u8(255, 192, 203),
        }
    }
}
