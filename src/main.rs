use std::path;

use ggez::{conf::{self, WindowMode}, event, winit::dpi::LogicalSize, ContextBuilder, GameResult};
use sokoban_inator::Game;

fn main() -> GameResult {
    let mut window_mode = WindowMode::default();
    window_mode.logical_size = Some(LogicalSize::new(800.0, 600.0));

    let mut exe_path = std::env::current_exe().expect("is an executable");
    let asset_path = if exe_path
        .parent().expect("parent is MacOS or release or src")
        .ends_with("Contents/MacOS")
    {
        exe_path.pop();
        exe_path.pop();
        exe_path.push("Resources");
        exe_path
    } else {
        path::PathBuf::from("./resources")
    };

    let context_builder = ContextBuilder::new("sokoban-inator", "FLIN8864")
    .window_setup(conf::WindowSetup::default().title("SOKOBAN-INATOR"))
    .window_mode(window_mode)
    .add_resource_path(asset_path);

    let (mut context, event_loop) = context_builder.build()?;
    let game = Game::new(&mut context);
    event::run(context, event_loop, game)
}
