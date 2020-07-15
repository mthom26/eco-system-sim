use std::path::PathBuf;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};

mod main_state;
use main_state::MainState;

fn main() {
    let ctx_builder = ContextBuilder::new("eco-system-sim", "me")
        .window_setup(WindowSetup::default())
        .window_mode(WindowMode::default().dimensions(1280.0, 720.0))
        .add_resource_path(PathBuf::from("./resources"));

    let (mut ctx, mut event_loop) = ctx_builder.build().expect("Could not build Context.");

    let mut state = MainState {};

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error: {}", e),
    }
}
