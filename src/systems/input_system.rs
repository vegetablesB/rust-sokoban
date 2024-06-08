use std::collections::HashMap;

use ggez::input::keyboard::KeyCode;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use specs::world::Index;

use crate::components::*;
use crate::constants::*;
use crate::resources::{GamePlay, InputQueue};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, GamePlay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut gameplay, entities, mut positions, players, movables, immovables) =
            data;
        let mut to_move = Vec::new();
        for (position, _player) in (&positions, &players).join() {
            if let Some(input) = input_queue.keys.pop() {
                // Get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|(e, _, p)| ((p.x, p.y), e.id()))
                    .collect();
                let imm: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|(e, _, p)| ((p.x, p.y), e.id()))
                    .collect();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to be moved

                let (start, end, is_x) = match input.keycode {
                    Some(KeyCode::Up) => (position.y, 0, false),
                    Some(KeyCode::Down) => (position.y, MAP_HEIGHT, false),
                    Some(KeyCode::Left) => (position.x, 0, true),
                    Some(KeyCode::Right) => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // Check if there is a movable object
                    match mov.get(&pos) {
                        Some(id) => to_move.push((input.keycode, id.clone())),
                        None => match imm.get(&pos) {
                            Some(_) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }
        // Now add the steps to the game play
        if to_move.len() > 0 {
            gameplay.moves_count += 1;
        }

        for (key, id) in to_move {
            if let Some(position) = positions.get_mut(entities.entity(id)) {
                match key {
                    Some(KeyCode::Up) => position.y -= 1,
                    Some(KeyCode::Down) => position.y += 1,
                    Some(KeyCode::Left) => position.x -= 1,
                    Some(KeyCode::Right) => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}
