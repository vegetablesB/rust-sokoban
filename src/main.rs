use std::path;

use ggez::{conf, Context, GameResult};
use ggez::event::{self};
use ggez::input::keyboard::KeyInput;
use specs::{RunNow, World, WorldExt};

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

// Struct to hold all the game state
struct Game {
    world: World,
}

// Main event loop
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context: _ctx };
            rs.run_now(&self.world);
        }

        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        println!("Key pressed: {:?}", input.keycode);
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys.push(input);
        Ok(())
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };

    // Run the main event loop
    event::run(context, event_loop, game);
}
