use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{Creature, Position, Velocity},
    resources::Time,
};

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Creature>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut positions, velocities, creatures) = data;

        for (pos, vel, creature) in (&mut positions, &velocities, &creatures).join() {
            pos.x += vel.x * time.delta * creature.velocity;
            pos.y += vel.y * time.delta * creature.velocity;
        }
    }
}
