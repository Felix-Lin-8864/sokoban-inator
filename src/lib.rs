use components::Time;
use entities::{create_audio_store, create_event_queue, create_gameplay, create_time};
use ggez::{event, Context, GameError};
use hecs::World;
use map::{gen_map, load_sounds};
use systems::{events::run_process_events, gameplay::run_gameplay_state, input::run_input, rendering::run_rendering};

mod components;
mod constants;
mod entities;
mod map;
mod systems;
mod events;

pub struct Game {
    pub world: World,
    pub level: u32,
}

impl Game {
    pub fn init(context: &mut Context, old_world: Option<&mut World>, level: u32) -> World {
        let mut world = World::new();
        {
            let world = &mut world;
            create_gameplay(world);
            create_time(world);
            create_event_queue(world);
            create_audio_store(world);
            load_sounds(world, context);
            gen_map(world, old_world, level);
        }
        world
    }

    pub fn new(context: &mut Context) -> Self {
        Self {
            world: Self::init(context, None, 0),
            level: 0,
        }
    }

    pub fn reinitialise_world(&mut self, context: &mut Context) {
        self.world = Self::init(context, Some(&mut self.world), self.level);
    }
}


impl event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        {
            run_input(self, ctx);
        }

        {
            run_gameplay_state(&self.world);
        }
        
        {
            let mut query = self.world.query::<&mut Time>();
            let time = query.iter().next().unwrap().1;
            time.delta += ctx.time.delta();
        }

        {
            run_process_events(self, ctx);
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        {
            run_rendering(self, ctx);
        }
        
        Ok(())
    }
}