use std::path;

use ggez::{conf, event, ContextBuilder, GameResult};
use sokoban_inator::Game;

fn main() -> GameResult {
    let context_builder = ContextBuilder::new("sokoban-inator", "sokoban")
    .window_setup(conf::WindowSetup::default().title("Sokoban-Inator"))
    .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
    .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    let game = Game::new(&mut context);
    event::run(context, event_loop, game)
}
