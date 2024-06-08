use ggez::input::keyboard::KeyInput;
use specs::World;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys: Vec<KeyInput>,
}

// Register resources with the world
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
