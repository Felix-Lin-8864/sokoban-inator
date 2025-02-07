use crate::components::*;
use crate::events::*;
use crate::Game;
use ggez::Context;

use std::collections::HashMap;

pub fn run_process_events(game: &mut Game, context: &mut Context) {
    let mut win_flag: bool = false;
    let mut new_events = Vec::new();
    {
        let world = &mut game.world;
        let events = {
            let mut query = world.query::<&mut EventQueue>();
            query
            .iter()
            .next()
            .unwrap()
            .1
            .events
            .drain(..)
            .collect::<Vec<_>>()
        };

        let mut query = world.query::<(&Position, &BoxSpot)>();
        let box_spots_by_position: HashMap<(u8, u8), &BoxSpot> = query
            .iter()
            .map(|(_, t)| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        let mut query = world.query::<&mut AudioStore>();
        let audio_store = query.iter().next().unwrap().1;
        
        for event in events {
            println!("New event: {:?}", event);

            match event {
                Event::EntityMoved(EntityMoved { entity }) => {
                    if let Ok(the_box) = world.get::<&Box>(entity) {
                        if let Ok(box_position) = world.get::<&Position>(entity) {
                            if let Some(box_spot) =
                                box_spots_by_position.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_spot.colour == the_box.colour || box_spot.colour == BoxColour::Grey || the_box.colour == BoxColour::Grey ),
                                }));
                            }
                        }
                    }
                }
                Event::PlayerHitObstacle => {
                    audio_store.play_sound(context, "wall");
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };
                    audio_store.play_sound(context, sound);
                }
                Event::GameWon => {
                    let mut query = world.query::<&mut Gameplay>();
                    let gameplay = query.iter().next().unwrap().1;
                    gameplay.state = GameplayState::Won;
                    win_flag = true;
                },
            }
        }
    }

    {
        let mut query = game.world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut new_events);
    }

    if win_flag {
        game.reinitialise_world(context);
        game.level += 1;
    }
}