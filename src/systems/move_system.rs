use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{Position, Velocity};

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (WriteStorage<'s, Position>, ReadStorage<'s, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, velocities) = data;

        for (pos, vel) in (&mut positions, &velocities).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}
