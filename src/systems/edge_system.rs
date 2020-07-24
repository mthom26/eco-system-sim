use specs::{Join, ReadStorage, System, WriteStorage};

use crate::{
    components::{Position, Velocity},
    HEIGHT, WIDTH,
};

pub struct EdgeSystem;

impl<'s> System<'s> for EdgeSystem {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut velocities) = data;

        for (pos, vel) in (&positions, &mut velocities).join() {
            if pos.x < 0.0 || pos.x > WIDTH {
                vel.x *= -1.0;
            }
            if pos.y < 0.0 || pos.y > HEIGHT {
                vel.y *= -1.0;
            }
        }
    }
}
