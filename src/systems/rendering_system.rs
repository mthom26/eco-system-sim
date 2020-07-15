use ggez::{
    graphics::{self, DrawParam},
    nalgebra::Point2,
    Context,
};
use specs::{Join, ReadStorage, System};

use crate::components::{Position, Renderable};

pub struct RenderingSystem<'s> {
    pub ctx: &'s mut Context,
}

impl<'s> System<'s> for RenderingSystem<'s> {
    type SystemData = (ReadStorage<'s, Position>, ReadStorage<'s, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        for (pos, renderable) in (&positions, &renderables).join() {
            let draw_params = DrawParam::new().dest(Point2::new(pos.x, pos.y));
            graphics::draw(self.ctx, &renderable.image, draw_params).expect("Could not render");
        }
    }
}
