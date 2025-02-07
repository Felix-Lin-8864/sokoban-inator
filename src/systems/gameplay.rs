use crate::{components::*, events::Event};
use hecs::World;

use std::collections::HashMap;

pub fn run_gameplay_state(world: &World) {
    // get all boxes
    let mut query = world.query::<(&Position, &Box)>();
    let boxes_by_position: HashMap<(u8, u8), &Box> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect();

    // iterate over all box_spots and check if there is a box on it (same position)
    let boxes_out_of_position: usize = world
        .query::<(&Position, &BoxSpot)>()
        .iter()
        .map(|(_, (position, box_spot))| {
            // if the correct colour box is on the boxspot, get 0, else 1
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if box_spot.colour == the_box.colour || box_spot.colour == BoxColour::Grey || the_box.colour == BoxColour::Grey {
                    0
                } else {
                    1
                }
            } else {
                1
            }
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .sum();

    // 0 would mean there were no box_spots without a box on it
    if boxes_out_of_position == 0 {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.push(Event::GameWon);
    }
}