mod ledmatrix;
mod resources;
mod alarm;

use alfred_rs::{tokio, AlfredModule};
use alfred_rs::connection::{Receiver, Sender};
use alfred_rs::error::Error;
use alfred_rs::message::{Message, MessageType};
use gpio_cdev::LineHandle;
use crate::resources::get_resource;

const MODULE_NAME: &str = "ledmatrix";
const INPUT_TOPIC: &str = "ledmatrix";
//const PLAY_STOP_EVENT: &str = "play_stop";
const PLAY_START_EVENT: &str = "play_start";
const PLAY_END_EVENT: &str = "play_end";

async fn execute_resource(resource_name: &str, sda_handle: &LineHandle, scl_handle: &LineHandle, module: &mut AlfredModule) {
    let frames = get_resource(resource_name)
        .expect("An error occurred while fetching resource");
    let message = Message {
        text: resource_name.to_string(),
        message_type: MessageType::Text,
            ..Message::default()
    };
    module.send_event(MODULE_NAME, PLAY_START_EVENT, &message).await.expect("An error occurred while sending start event");
    ledmatrix::show(sda_handle, scl_handle, frames);
    module.send_event(MODULE_NAME, PLAY_END_EVENT, &message).await.expect("An error occurred while sending end event");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut module = AlfredModule::new(MODULE_NAME).await.expect("Failed to create module");
    module.listen(INPUT_TOPIC).await.expect("An error occurred while listening");
    let sda_pin = module.config.get_module_value("sda")
        .map_or(17u32, |s| s.parse().expect("Failed to parse sda property"));
    let scl_pin = module.config.get_module_value("scl")
        .map_or(27u32, |s| s.parse().expect("Failed to parse sda property"));
    let (sda_handle, scl_handle) = ledmatrix::init(sda_pin, scl_pin).expect("An error occurred while initializing GPIO");
    execute_resource("heart", &sda_handle, &scl_handle, &mut module).await;
    execute_resource("pacman", &sda_handle, &scl_handle, &mut module).await;
    execute_resource("skater", &sda_handle, &scl_handle, &mut module).await;
    execute_resource("space", &sda_handle, &scl_handle, &mut module).await;
    execute_resource("wait", &sda_handle, &scl_handle, &mut module).await;
    //ledmatrix::test().unwrap();
    //ledmatrix::show(&sda_handle, &scl_handle, vec![frame]);
    Ok(())
}

