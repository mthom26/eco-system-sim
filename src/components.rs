use ggez::graphics::Image;
use rand::{thread_rng, Rng};
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

impl Velocity {
    // Should probably normalize this
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        Self::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)).normalized()
    }

    pub fn normalized(self) -> Self {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        Velocity {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

// Currently the Renderable component is drawn with its top left corner at the
// origin point of the position. Need to add an offset (radius) to shift it so
// it is centered
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
    pub current_time: f32,
    // Magnitude of creatures velocity
    pub velocity: f32,
}

impl Creature {
    pub fn new(time: f32, velocity: f32) -> Self {
        Self {
            time,
            current_time: time,
            velocity,
        }
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();
    world.register::<Creature>();
}
