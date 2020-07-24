use ggez::{
    event::EventHandler,
    graphics::{self, Color, Image},
    timer, Context, GameResult,
};
use specs::{RunNow, World, WorldExt};

use crate::{
    components::register_components,
    resources::{register_resources, Time},
    systems::{CreatureUpdate, EdgeSystem, MoveSystem, RenderingSystem},
    utils::test_level,
};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

pub struct MainState {
    specs_world: World,
    assets: Assets,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let assets = Assets::new(ctx);

        let mut specs_world = World::new();
        register_resources(&mut specs_world);
        register_components(&mut specs_world);

        test_level(&mut specs_world, &assets);

        Self {
            assets,
            specs_world,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut ms = MoveSystem {};
        ms.run_now(&self.specs_world);
        let mut es = EdgeSystem {};
        es.run_now(&self.specs_world);
        let mut cs = CreatureUpdate {};
        cs.run_now(&self.specs_world);

        let mut time = self.specs_world.write_resource::<Time>();
        time.delta = timer::delta(ctx).as_secs_f32();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from(CLEAR_COLOR));
        let mut rs = RenderingSystem { ctx };
        rs.run_now(&self.specs_world);
        graphics::present(ctx)
    }
}

pub struct Assets {
    pub test_image: Image,
    pub food_image: Image,
    pub creature_image: Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> Self {
        let test_image = graphics::Image::new(ctx, "/assets/images/test.png").unwrap();
        let food_image = graphics::Image::new(ctx, "/assets/images/food.png").unwrap();
        let creature_image = graphics::Image::new(ctx, "/assets/images/creature.png").unwrap();

        Self {
            test_image,
            food_image,
            creature_image,
        }
    }
}
