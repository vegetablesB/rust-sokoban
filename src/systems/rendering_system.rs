use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, TextFragment};
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

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            // let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let image =
                Image::from_path(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
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
