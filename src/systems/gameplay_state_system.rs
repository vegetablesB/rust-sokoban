use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::components::{Box, BoxSpot, Position};
use crate::resources::{GamePlay, GamePlayState};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_play, positions, boxes, box_spots) = data;
        // Check if the game is won: check the position of the boxes and box spots
        let boxes_by_position: HashMap<(u8, u8), ()> = (&boxes, &positions)
            .join()
            .map(|(_, position)| ((position.x, position.y), ()))
            .collect();

        // Loop all box spots and check if there is a box on it
        for (position, _) in (&positions, &box_spots).join() {
            if boxes_by_position.contains_key(&(position.x, position.y)) {
                continue;
            } else {
                game_play.state = GamePlayState::Playing;
                return;
            }
        }

        game_play.state = GamePlayState::Won;
    }
}
