use std::collections::HashMap;

use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, InstanceArray, TextFragment};
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};

use crate::components::*;
use crate::constants::TILE_WIDTH;
use crate::resources::GamePlay;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        let mut canvas = Canvas::from_frame(self.context, Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Group renderables by their image path
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();
        for (position, renderable) in rendering_data.iter() {
            rendering_batches
                .entry(position.z)
                .or_insert_with(HashMap::new)
                .entry(renderable.path.clone())
                .or_insert_with(Vec::new)
                .push(DrawParam::new().dest(Vec2::new(
                    position.x as f32 * TILE_WIDTH,
                    position.y as f32 * TILE_WIDTH,
                )));
        }

        // Sort the keys of renderable_groups to ensure consistent order
        // without .sorted_by(|a, b| a.0.cmp(b.0)) will cause image flickering, use this
        // sorted_by from itertools to ensure consistent rendering order

        // Draw each group
        for (_z, group) in rendering_batches.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            for (path, draw_param) in group {
                let image = Image::from_path(self.context, path).expect("expected image");
                let mut instance_array = InstanceArray::new(self.context, image);
                for param in draw_param.iter() {
                    instance_array.push(param.clone());
                }
                canvas.draw(&instance_array, DrawParam::default());
            }
        }

        // Render any text
        self.draw_text(&mut canvas, gameplay.state.to_string(), 525.0, 80.0, 44.0);
        self.draw_text(
            &mut canvas,
            gameplay.moves_count.to_string(),
            525.0,
            120.0,
            44.0,
        );
        let fps = format!("FPS: {:.0}", self.context.time.fps());
        self.draw_text(&mut canvas, fps, 525.0, 160.0, 44.0);

        // Finally, present the context, this will actually display everything
        // on the screen.
        canvas.finish(self.context).expect("expected to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(
        &mut self,
        canvas: &mut Canvas,
        text_string: String,
        x: f32,
        y: f32,
        font_size: f32,
    ) {
        let destination = Vec2::new(x, y);
        let color = Color::RED;
        let text = graphics::Text::new(TextFragment {
            text: text_string,
            color: Some(color),
            scale: Some(font_size.into()),
            font: None,
        });
        let draw_params = DrawParam::new().dest(destination);

        canvas.draw(&text, draw_params);
    }
}
