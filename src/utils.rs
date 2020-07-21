use rand::{thread_rng, Rng};
use specs::World;

use crate::{
    components::Position,
    entities::{create_food, create_test},
    main_state::Assets,
    HEIGHT, WIDTH,
};

pub fn test_level(world: &mut World, assets: &Assets) {
    let mut rng = thread_rng();

    let margin = 20.0; // Keep food away from map edges

    for _ in 0..50 {
        let rand_x = rng.gen_range(0.0 + margin, WIDTH - margin);
        let rand_y = rng.gen_range(0.0 + margin, HEIGHT - margin);
        create_food(
            world,
            Position::new(rand_x, rand_y),
            assets.food_image.clone(),
        );
    }

    create_test(world, Position::new(50.0, 50.0), assets.test_image.clone());
}
