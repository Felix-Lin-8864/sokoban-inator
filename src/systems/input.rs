use ggez::{input::keyboard::KeyCode, Context};
use hecs::Entity;

use std::collections::HashMap;

use crate::components::*;
use crate::constants::*;
use crate::events::EntityMoved;
use crate::events::Event;
use crate::Game;

pub fn run_input(game: &mut Game, context: &mut Context) {
    // restart game without carrying over players
    if context.keyboard.is_key_just_pressed(KeyCode::R) {
        game.world = Game::init(context, None, 0);
        game.level = 0;
        return;
    }

    let world = &game.world;
    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Movable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect();

    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Immovable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect();

    // Vec of entities to be moved in KeyCode direction
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if context.keyboard.is_key_repeated() {
            continue;
        }

        let key = if context.keyboard.is_key_just_pressed(KeyCode::Up) {
            KeyCode::Up
        } else if context.keyboard.is_key_just_pressed(KeyCode::Down) {
            KeyCode::Down
        } else if context.keyboard.is_key_just_pressed(KeyCode::Left) {
            KeyCode::Left
        } else if context.keyboard.is_key_just_pressed(KeyCode::Right) {
            KeyCode::Right
        } else {
            continue;
        };
        
        // start is beginning coord of the chain, end is absolute end (map bounds in KeyCode direction),
        // and is_x tells us if it is a horizontal or vertical chain on the map
        let (start, end, is_x) = match key {
            KeyCode::Up => (position.y, 0, false),
            KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
            KeyCode::Left => (position.x, 0, true),
            KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
            _ => continue,
        };

        // build the chain
        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        // iterate over the chain, checking for any invalid 
        for x_or_y in range {
            // get position
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };

            match mov.get(&pos) {
                // if there is a movable entity, add it to to_move
                Some(entity) => to_move.push((*entity, key)),
                None => {
                    match immov.get(&pos) {
                        // if there is an immovable entity, the movement is invalid, and nothing should move
                        Some(_id) => {
                            to_move.clear();
                            events.push(Event::PlayerHitObstacle {});
                            break;
                        },
                        // if its not movable or immovable (empty/floor/boxspot), the move is valid
                        None => break,
                    }
                    // *** note the above breaks are what makes team squishing sometimes possible,
                    // depending on the order the players are iterated over. Simple fix was just to
                    // switch the breaks to continues, and have separate to_move vecs for each
                    // iteratio, but I wanted to leave it in since its a cool bug :D
                }
            }
        }
    }

    if !to_move.is_empty() {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.moves_count += 1;
    }

    for (entity, key) in to_move {
        let mut position = world.get::<&mut Position>(entity).unwrap();
        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => (),
        }

        events.push(Event::EntityMoved(EntityMoved { entity }));
    }

    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut events);
    }
}