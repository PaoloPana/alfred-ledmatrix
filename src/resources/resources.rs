use std::error::Error;
use crate::resources::heart::Heart;
use crate::resources::pacman::Pacman;
use crate::resources::Resource;
use crate::resources::skater::Skater;
use crate::resources::space::Space;
use crate::resources::wait::Wait;

pub fn get_resource(resource_name: &str) -> Result<Vec<[u32; 17]>, Box<dyn Error>> {
    match resource_name {
        "heart" => Ok(Heart::get_resource()),
        "pacman" => Ok(Pacman::get_resource()),
        "skater" => Ok(Skater::get_resource()),
        "space" => Ok(Space::get_resource()),
        "wait" => Ok(Wait::get_resource()),
        _ => Err("Unknown resource name".into())
    }
}