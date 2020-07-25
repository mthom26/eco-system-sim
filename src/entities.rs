use ggez::graphics::Image;
use specs::{Builder, World, WorldExt};

use crate::components::{Creature, Position, Renderable, Velocity};

pub fn create_test(world: &mut World, pos: Position, image: Image) {
    world
        .create_entity()
        .with(pos)
        .with(Renderable { image })
        .build();
}

pub fn create_food(world: &mut World, pos: Position, image: Image) {
    world
        .create_entity()
        .with(pos)
        .with(Renderable { image })
        .build();
}

pub fn create_creature(world: &mut World, pos: Position, vel: Velocity, image: Image) {
    world
        .create_entity()
        .with(pos)
        .with(vel)
        .with(Creature::new(5.0, 50.0))
        .with(Renderable { image })
        .build();
}
