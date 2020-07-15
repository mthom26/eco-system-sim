use ggez::{
    event::EventHandler,
    graphics::{self, Color, Image},
    Context, GameResult,
};
use specs::{RunNow, World, WorldExt};

use crate::{components::register_components, systems::RenderingSystem, utils::test_level};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

pub struct MainState {
    specs_world: World,
    assets: Assets,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let assets = Assets::new(ctx);

        let mut specs_world = World::new();
        register_components(&mut specs_world);

        test_level(&mut specs_world, assets.test_image.clone());

        Self {
            assets,
            specs_world,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from(CLEAR_COLOR));
        let mut rs = RenderingSystem { ctx };
        rs.run_now(&self.specs_world);
        graphics::present(ctx)
    }
}

struct Assets {
    test_image: Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> Self {
        let test_image = graphics::Image::new(ctx, "/assets/images/test.png").unwrap();

        Self { test_image }
    }
}
