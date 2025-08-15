use crate::{
    components::Piece,
    game_color::GameColor,
    game_types::{PieceMatrix, PieceType, Presence},
};

use rand::{Rng, rng};

pub fn get_block_matrix(num: u16, color: GameColor) -> PieceMatrix {
    let mut res = [[Presence::No; 4]; 4];
    for i in 0..16 {
        if num & (1_u16 << (15 - i)) > 0 {
            res[i / 4][i % 4] = Presence::Yes(color)
        }
    }
    res
}

impl From<PieceType> for Piece {
    fn from(piece_type: PieceType) -> Self {
        use PieceType::*;

        let def = Piece::default();

        match piece_type {
            L => Piece {
                states: [17504, 1856, 1570, 736],
                color: GameColor::Orange,
                ..def
            },
            J => Piece {
                states: [8800, 1136, 1604, 3616],
                color: GameColor::Blue,
                ..def
            },
            S => Piece {
                states: [17952, 1728, 17952, 1728],
                color: GameColor::Green,
                ..def
            },
            Z => Piece {
                states: [9792, 3168, 9792, 3168],
                color: GameColor::Red,
                ..def
            },
            T => Piece {
                states: [17984, 3648, 19520, 19968],
                color: GameColor::Purple,
                ..def
            },
            I => Piece {
                states: [17476, 3840, 17476, 3840],
                color: GameColor::Cyan,
                ..def
            },
            O => Piece {
                states: [1632, 1632, 1632, 1632],
                color: GameColor::Yellow,
                ..def
            },
        }
    }
}

impl Piece {
    pub fn random() -> Self {
        let mut rng = rng();
        let piece_type = match rng.random_range(0..7) {
            0 => PieceType::L,
            1 => PieceType::J,
            2 => PieceType::S,
            3 => PieceType::Z,
            4 => PieceType::T,
            5 => PieceType::I,
            _ => PieceType::O,
        };
        Piece::from(piece_type)
    }
}
