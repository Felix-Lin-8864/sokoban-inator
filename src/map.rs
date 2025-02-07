use crate::components::{AudioStore, BoxColour, Player, Position};
use crate::constants::{COLOUR_TOKENS, MAP_HEIGHT, MAP_WIDTH};
use crate::entities::*;
use ggez::audio::Source;
use ggez::Context;
use hecs::World;
use rand::rngs::ThreadRng;
use rand::{rng, Rng};

fn parse_colour(c: &str) -> BoxColour {
    match &c.chars().next().unwrap() {
        'G' => BoxColour::Grey,
        'B' => BoxColour::Blue,
        'R' => BoxColour::Red,
        'Y' => BoxColour::Yellow,
        'L' => BoxColour::Lime,
        _ => panic!("unrecognized map item {}", c),
    }
}

fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we get the z from the factory functions
            };

            create_floor(world, position);

            match *column {
                "W" => {
                    create_wall(world, position);
                }
                "P" => {
                    create_player(world, position);
                }
                "N" | "." => (),
                c => {
                    let colour = parse_colour(c);
                    // if Box { "GB", "BB", "RB" }
                    if c.ends_with("B") {
                        create_box(world, position, colour);
                    // if BoxSpot { "GS", "BS", "RS" }
                    } else if c.ends_with("S") {
                        create_box_spot(world, position, colour);
                    } else {
                        panic!("unrecognized map item {}", c);
                    }
                }
            }
        }
    }
}

fn gen_colour_token(random: &mut ThreadRng, colour: Option<char>) -> char {
    let rand_colour = COLOUR_TOKENS[random.random_range(0..=3) as usize];
    match colour {
        // if we already have a non-'G' colour, return the same colour token
        Some(colour) => {
            if colour == 'G' {
                rand_colour
            } else {
                colour
            }
        },
        // if we do not have a colour yet, 15% chance to get 'G' otherwise random
        None => {
            if random.random_bool(0.15) {
                'G'
            } else {
                rand_colour
            }
        },
    }
}

fn random_empty_pos(map: &[Vec<String>], random: &mut ThreadRng) -> (usize, usize) {
    loop {
        // *** just ensure they don't spawn on edge for now, but fix later
        // once map validation function completed ***
        let row = random.random_range(2..(MAP_HEIGHT as usize - 2));
        let col = random.random_range(2..(MAP_WIDTH as usize - 2));
        if map[row][col] == "." {
            return (row, col);
        }
    }
}

pub fn gen_map(world: &mut World, old_world: Option<&mut World>, level: u32) {
    let mut random = rng();

    // initialise empty enclosed map
    let mut map = vec![vec![String::from("W"); MAP_WIDTH as usize]; MAP_HEIGHT as usize];
    for row in 1..(MAP_HEIGHT as usize - 1) {
        for col in 1..(MAP_WIDTH as usize - 1) {
            map[row][col] = String::from(".");
        }
    }

    let (row, col) = random_empty_pos(&map, &mut random);
    map[row][col] = String::from("P");

    // number of boxes generally scales with level (number of players)
    // for increased control difficulty
    for _ in 0..random.random_range((level+1)..=(level+3)) {
        let box_colour = gen_colour_token(&mut random, None);
        let (row, col) = random_empty_pos(&map, &mut random);
        map[row][col] = format!("{box_colour}B");

        let box_spot_colour = gen_colour_token(&mut random, Some(box_colour));
        let (row, col) = random_empty_pos(&map, &mut random);
        map[row][col] = format!("{box_spot_colour}S");
    }

    // have players carry over if they don't overlap with a block or player
    if let Some(old_world) = old_world {
        let mut query = old_world.query::<(&Position, &Player)>();
        for (_, (p_pos, _player)) in query.iter() {
            let curr = &map[p_pos.y as usize][p_pos.x as usize];
            if !(curr.ends_with("B") || curr == "P") {
                map[p_pos.y as usize][p_pos.x as usize] = String::from("P");
            }
        }
    }

    let map_string = map.into_iter()
        .map(|row| row.join(" "))
        .collect::<Vec<String>>()
        .join("\n");

    load_map(world, map_string);
}

pub fn load_sounds(world: &mut World, context: &mut Context) {
    let mut query = world.query::<&mut AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    for sound in ["correct", "incorrect", "wall"] {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{sound_name}.wav");
        let sound_source = Source::new(context, sound_path).expect("Expected sound loaded");

        audio_store.sounds.insert(sound_name, Box::new(sound_source));
    }
}