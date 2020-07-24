use std::time::Duration;

use specs::World;

#[derive(Default)]
pub struct Time {
    pub delta: f32,
}

pub fn register_resources(world: &mut World) {
    world.insert(Time::default());
}
