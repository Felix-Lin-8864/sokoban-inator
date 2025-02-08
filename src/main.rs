use std::path;

use ggez::{conf::{self, WindowMode}, event, winit::dpi::LogicalSize, ContextBuilder, GameResult};
use sokoban_inator::Game;

fn main() -> GameResult {
    let mut window_mode = WindowMode::default();
    window_mode.logical_size = Some(LogicalSize::new(800.0, 600.0));

    let context_builder = ContextBuilder::new("sokoban-inator", "FLIN8864")
    .window_setup(conf::WindowSetup::default().title("SOKOBAN-INATOR"))
    .window_mode(window_mode)
    .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    let game = Game::new(&mut context);
    event::run(context, event_loop, game)
}
