use std::fmt::Display;

use ggez::input::keyboard::KeyInput;
use specs::World;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys: Vec<KeyInput>,
}

#[derive(Default)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub moves_count: u32,
}

pub enum GamePlayState {
    Playing,
    Won,
}

impl Default for GamePlayState {
    fn default() -> Self {
        GamePlayState::Playing
    }
}

impl Display for GamePlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GamePlayState::Playing => write!(f, "Playing"),
            GamePlayState::Won => write!(f, "Won"),
        }
    }
}

// Register resources with the world
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(GamePlay::default());
}
