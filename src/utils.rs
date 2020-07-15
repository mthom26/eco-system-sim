use ggez::graphics::Image;
use specs::World;

use crate::{components::Position, entities::create_test};

pub fn test_level(world: &mut World, image: Image) {
    create_test(world, Position::new(50.0, 50.0), image)
}
