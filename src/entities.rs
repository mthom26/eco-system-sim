use ggez::graphics::Image;
use specs::{Builder, World, WorldExt};

use crate::components::{Position, Renderable};

pub fn create_test(world: &mut World, pos: Position, image: Image) {
    world
        .create_entity()
        .with(Position { ..pos })
        .with(Renderable { image })
        .build();
}
