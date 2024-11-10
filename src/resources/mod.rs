use std::error::Error;
use crate::resources::clock::Clock;
use crate::resources::heart::Heart;
use crate::resources::pacman::Pacman;
use crate::resources::skater::Skater;
use crate::resources::space::Space;
use crate::resources::wait::Wait;

trait Resource {
    fn new() -> Self;
    fn get_resource(&mut self) -> Vec<[u32; 17]>;
}

pub struct Resources {
    heart: Heart,
    pacman: Pacman,
    skater: Skater,
    space: Space,
    wait: Wait,
    clock: Clock,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            heart: Heart::new(),
            pacman: Pacman::new(),
            skater: Skater::new(),
            space: Space::new(),
            wait: Wait::new(),
            clock: Clock::new(),
        }
    }

    pub fn get(&mut self, resource_name: &str) -> Result<Vec<[u32; 17]>, Box<dyn Error>> {
        match resource_name {
            "heart" => Ok(self.heart.get_resource()),
            "pacman" => Ok(self.pacman.get_resource()),
            "skater" => Ok(self.skater.get_resource()),
            "space" => Ok(self.space.get_resource()),
            "wait" => Ok(self.wait.get_resource()),
            "clock" => Ok(self.clock.get_resource()),
            _ => Err("Unknown resource name".into())
        }
    }

    pub fn update_tick(&mut self) {
        self.clock.update_tick();
    }
}

mod chars;
mod clock;
mod heart;
mod pacman;
mod space;
mod skater;
mod wait;