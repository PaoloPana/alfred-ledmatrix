mod ledmatrix;
mod resources;

use alfred_rs::{tokio, AlfredModule};
use alfred_rs::connection::Receiver;
use alfred_rs::error::Error;
use gpio_cdev::LineHandle;
use crate::resources::resources::get_resource;

const MODULE_NAME: &str = "ledmatrix";
const INPUT_TOPIC: &str = "ledmatrix";
const PLAY_STOP_EVENT: &str = "play_stop";
const PLAY_START_EVENT: &str = "play_start";
const PLAY_END_EVENT: &str = "play_end";

fn execute_resource(resource_name: &str, sda_handle: &LineHandle, scl_handle: &LineHandle) {
    let frames = get_resource(resource_name)
        .expect("An error occurred while fetching resource");
    ledmatrix::show(&sda_handle, &scl_handle, frames);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut module = AlfredModule::new(MODULE_NAME).await.expect("Failed to create module");
    module.listen(INPUT_TOPIC).await.expect("An error occurred while listening");
    let sda_pin = module.config.get_module_value("sda")
        .map_or(17u32, |s| s.parse().unwrap());
    let scl_pin = module.config.get_module_value("scl")
        .map_or(27u32, |s| s.parse().unwrap());
    println!("sda: {sda_pin} scl: {scl_pin}");
    println!("Hello, world!");
    let (sda_handle, scl_handle) = ledmatrix::init().unwrap();
    execute_resource("heart", &sda_handle, &scl_handle);
    execute_resource("pacman", &sda_handle, &scl_handle);
    execute_resource("skater", &sda_handle, &scl_handle);
    execute_resource("space", &sda_handle, &scl_handle);
    execute_resource("wait", &sda_handle, &scl_handle);
    //ledmatrix::test().unwrap();
    //ledmatrix::show(&sda_handle, &scl_handle, vec![frame]);

    println!("2 Hello, world!");

    Ok(())
}

