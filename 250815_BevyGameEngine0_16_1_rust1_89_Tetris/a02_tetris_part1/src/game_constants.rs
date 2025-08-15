pub const TITLE: &str = "Tetris";

pub const NUM_BLOCKS_X: usize = 10;
pub const NUM_BLOCKS_Y: usize = 20;

pub const TEXTURE_SIZE: u32 = 32;

pub const WIDTH: u32 = NUM_BLOCKS_X as u32 * TEXTURE_SIZE;
pub const HEIGHT: u32 = NUM_BLOCKS_Y as u32 * TEXTURE_SIZE;

pub const NUM_LEVELS: usize = 10;
pub const LEVEL_TIMES: [usize; NUM_LEVELS] = [1000, 850, 800, 700, 600, 500, 400, 300, 200, 100];
