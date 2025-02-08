use std::{collections::HashMap, time::Duration};

use ggez::{
    glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment}, Context
};
use hecs::Entity;
use itertools::Itertools;

use crate::{components::*, Game};
use crate::constants::*;

pub fn get_frame(renderable: &Renderable, delta: Duration) -> String {
    let path_index = match renderable.kind() {
        RenderableKind::Static => 0,
        RenderableKind::Animated => ((delta.as_millis() % 1000) / FRAME_RATE) as usize,
    };

    renderable.path(path_index)
}

pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32, scale_factor: f32) {
    let text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x * scale_factor, y * scale_factor));
}

pub fn run_rendering(game: &Game, context: &mut Context) {
    let world = &game.world;
    let mut canvas =
        Canvas::from_frame(context, Color::from([0.95, 0.95, 0.95, 1.0]));
    
    let scale_factor = context.gfx.window().scale_factor() as f32;

    let mut query = world.query::<&Time>();
    let time = query.iter().next().unwrap().1;

    // get all renderables and sort by z for visual layering
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

    for (_, (position, renderable)) in rendering_data.iter() {
        let img = get_frame(renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH * scale_factor;
        let y = position.y as f32 * TILE_WIDTH * scale_factor;
        let z = position.z;

        let draw_param = DrawParam::new()
            .dest(Vec2::new(x, y))
            .scale(Vec2::new(scale_factor, scale_factor));

        rendering_batches
            .entry(z)
            .or_default()
            .entry(img)
            .or_default()
            .push(draw_param);
    }

    let fps = format!("FPS:  {:.0}", context.time.fps());
    
    for (_z, group) in rendering_batches
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    {
        for (image_path, draw_params) in group {
            let image = Image::from_path(context, image_path).unwrap();
            let mut mesh_batch = graphics::InstanceArray::new(context, Some(image));
            
            for draw_param in draw_params.iter() {
                mesh_batch.push(*draw_param);
            }
            
            canvas.draw(&mesh_batch, DrawParam::new());
        }
    }
    
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(
        &mut canvas, 
        &format!("Level: {}\nMoves: {}", game.level, gameplay.moves_count), 
        525.0, 
        10.0, 
        scale_factor
    );
    
    let instructions = "
        Use arrow keys to move\n
        Press R to restart\n
        Push boxes onto spots of
        matching colours\n
        Every colour matches with
        grey!
    ";
    draw_text(&mut canvas, instructions, 480.0, 140.0, scale_factor);
    draw_text(
        &mut canvas, 
        "Hint: try squishing your team on walls!\n(def not part of a bug I found cool)", 
        30.0, 
        545.0,
        scale_factor,
    );
    draw_text(&mut canvas, &fps, 525.0, 50.0, scale_factor);
    
    canvas.finish(context).expect("Expected to present");
}