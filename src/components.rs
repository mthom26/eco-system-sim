use ggez::graphics::Image;
use specs::{Component, VecStorage, World, WorldExt};

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.1, y: 0.1 }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub image: Image,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Creature {
    // Timer to choose new velocity at random
    pub time: f32,
    current_time: f32,
}

impl Creature {
    pub fn new(time: f32) -> Self {
        Self {
            time,
            current_time: time,
        }
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();
    world.register::<Creature>();
}
