use ggez::{
    event::EventHandler,
    graphics::{self, Color},
    Context, GameResult,
};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

pub struct MainState {}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from(CLEAR_COLOR));
        graphics::present(ctx)
    }
}
