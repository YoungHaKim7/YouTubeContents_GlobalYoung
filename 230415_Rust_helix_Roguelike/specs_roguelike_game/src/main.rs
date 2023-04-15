use specs::{
    world::Builder, Component, Read, ReadStorage, System, SystemData, VecStorage, World,
    WriteStorage,
};
use tcod::colors;
use tcod::console::*;
use tcod::input::{Key, KeyCode};

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;
const LIMIT_FPS: i32 = 20;

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct CharacterGlyph {
    glyph: char,
}

impl Component for CharacterGlyph {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
struct PrintMeTag;

impl Component for PrintMeTag {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug, Default)]
struct PlayerController;

impl Component for PlayerController {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug, Default)]
struct GameState {
    end: bool,
    keyPress: Option<KeyCode>,
}

struct PrintingSystem;
impl<'a> System<'a> for PrintingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, PrintMeTag>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (position, print_me, game_state) = data;
        println!("{:?}", *game_state);
        for (pos, _) in (&position, &print_me).join() {
            println!("{:?}", pos);
        }
    }
}

struct PlayerMove;
impl<'a> System<'a> for PlayerMove {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        use KeyCode::*;
        let (mut position, player_controlled, game_state) = data;
        if let Some(key) = game_state.keyPress {
            for (pos, _) in (&mut position, &player_controlled).join() {
                match key {
                    Up => pos.y -= 1,
                    Down => pos.y += 1,
                    Left => pos.x -= 1,
                    Right => pos.x += 1,
                    _ => {}
                }
            }
        }
    }
}

struct TcodSystem {
    window: Root,
}

impl<'a> System<'a> for TcodSystem {
    type SystemData = (
        ReadStorage<'a, CharacterGlyph>,
        ReadStorage<'a, Position>,
        specs::Write<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        use KeyCode::*;

        let root = &mut self.window;

        let (sprites, positions, mut game_state) = data;

        root.clear();
        for (sprite, pos) in (&sprites, &positions).join() {
            root.put_char(pos.x, pos.y, sprite.glyph, BackgroundFlag::None);
        }
        root.flush();
        let key = root.wait_for_keypress(false);

        let key_press = match key {
            Key { code: Escape, .. } => {
                game_state.end = true;
                None
            }
            Key { code: Up, .. } => Some(Up),
            Key { code: Down, .. } => Some(Down),
            Key { code: Left, .. } => Some(Left),
            Key { code: Right, .. } => Some(Right),
            _ => None,
        };

        game_state.keyPress = key_press;
        game_state.end = game_state.end || root.window_closed();
    }
}

fn main() {
    let mut root = Root::initializer()
        .font("terminal16x16_gs_ro.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike using specs")
        .init();

    root.set_default_foreground(colors::WHITE);

    tcod::system::set_fps(LIMIT_FPS);

    let mut world = World::new();
    world.add_resource(GameState::default());

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .with(PlayerMove, "player_move", &[])
        .with_thread_local(TcodSystem { window: root })
        .build();

    dispatcher.setup(&mut world.res);

    world
        .create_entity()
        .with(Position { x: 10, y: 10 })
        .with(CharacterGlyph { glyph: 'y' })
        .build();

    world
        .create_entity()
        .with(Position { x: 23, y: 10 })
        .with(CharacterGlyph { glyph: 'x' })
        .build();
    world
        .create_entity()
        .with(Position { x: 1, y: 3 })
        .with(CharacterGlyph { glyph: 'A' })
        .build();
    world
        .create_entity()
        .with(Position { x: 11, y: 22 })
        .with(CharacterGlyph { glyph: '@' })
        .with(PlayerController {})
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        println!("press key");
        {
            let game_state = world.read_resource::<GameState>();
            if game_state.end {
                break;
            }
        }
        world.maintain();
    }
}
