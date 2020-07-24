use specs::{Join, Read, System, WriteStorage};

use crate::{
    components::{Creature, Velocity},
    resources::Time,
};

pub struct CreatureUpdate;

// Handle each creatures internal logic (update velocity)
impl<'s> System<'s> for CreatureUpdate {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Creature>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut creatures, mut velocities) = data;

        for (creature, _vel) in (&mut creatures, &mut velocities).join() {
            creature.current_time -= time.delta;
            if creature.current_time < 0.0 {
                // TODO - Get new random velocity
                println!("Update creature velocity");
                creature.current_time = creature.time;
            }
        }
    }
}
