use std::{collections::HashMap, fmt::Display, time::Duration, boxed};

use ggez::{audio::{self, SoundSource}, Context};

use crate::events::Event;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub enum RenderableKind {
    Static,
    Animated,
}

pub struct Renderable {
    pub paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: &str) -> Self {
        Self {
            paths: vec![path.to_string()],
        }
    }

    pub fn new_animated(paths: Vec<&str>) -> Self {
        Self {
            paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        self.paths[path_index % self.paths.len()].clone()
    }
}

pub struct Wall {}

pub struct Player {}

pub struct Box {
    pub colour: BoxColour,
}

pub struct BoxSpot {
    pub colour: BoxColour,
}

pub struct Movable;
pub struct Immovable;

#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won!",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(PartialEq)]
pub enum BoxColour {
    Grey, // grey boxes are universal
    Red,
    Blue,
    Yellow,
    Lime,
}

impl Display for BoxColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColour::Grey => "grey",
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
            BoxColour::Yellow => "yellow",
            BoxColour::Lime => "lime",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, boxed::Box<audio::Source>>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &str) {
        if let Some(source) = self.sounds.get_mut(sound) {
            if source.play_detached(context).is_ok() {
                println!("Playing sound: {sound}");
            }
        }
    }
}