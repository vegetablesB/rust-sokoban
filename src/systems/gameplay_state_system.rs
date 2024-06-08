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
        // Collect and sort positions of boxes
        let mut box_positions = (&boxes, &positions)
            .join()
            .map(|(_, pos)| pos)
            .collect::<Vec<_>>();
        box_positions.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

        // Collect and sort positions of box spots
        let mut box_spot_positions = (&box_spots, &positions)
            .join()
            .map(|(_, pos)| pos)
            .collect::<Vec<_>>();
        box_spot_positions.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

        // Check if all boxes are at the box spots
        let mut won = box_positions.len() == box_spot_positions.len();

        for (box_position, box_spot_position) in box_positions.iter().zip(box_spot_positions.iter())
        {
            if box_position.x != box_spot_position.x || box_position.y != box_spot_position.y {
                won = false;
                break;
            }
        }

        if won {
            game_play.state = GamePlayState::Won;
        } else {
            game_play.state = GamePlayState::Playing;
        }
    }
}
