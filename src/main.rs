mod ledmatrix;
mod resources;
mod alarm;

use alfred_rs::{tokio, AlfredModule};
use alfred_rs::connection::Connection;
use alfred_rs::error::Error;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio::sync::mpsc::Receiver;
use gpio_cdev::LineHandle;
use crate::resources::Resources;

const MODULE_NAME: &str = "ledmatrix";
const INPUT_TOPIC: &str = "ledmatrix";
//const PLAY_STOP_EVENT: &str = "play_stop";
const PLAY_START_EVENT: &str = "play_start";
const PLAY_END_EVENT: &str = "play_end";

struct LedMatrix {
    sda_handle: LineHandle,
    scl_handle: LineHandle,
    resources: Resources
}
impl LedMatrix {
    pub fn new(sda_pin: u32, scl_pin: u32) -> Self {
        let (sda_handle, scl_handle) = ledmatrix::init(sda_pin, scl_pin).expect("An error occurred while initializing GPIO");
        Self {
            sda_handle, scl_handle, resources: Resources::new()
        }
    }

    fn execute_resource(&self, resource_name: &str) {
        let frames = self.resources.get(resource_name)
            .expect("An error occurred while fetching resource");
        ledmatrix::show(&self.sda_handle, &self.scl_handle, frames);
    }

    pub async fn handle_alfred_incoming(&self, mut receiver: Receiver<Message>, connection: &Connection) {
        loop {
            let resource_name = receiver.recv().await.expect("An error occurred while receiving a message").text;
            let message = Message {
                text: resource_name.to_string(),
                message_type: MessageType::Text,
                ..Message::default()
            };
            connection.send_event(MODULE_NAME, PLAY_START_EVENT, &message).await.expect("An error occurred while sending start event");
            self.execute_resource(&resource_name);
            connection.send_event(MODULE_NAME, PLAY_END_EVENT, &message).await.expect("An error occurred while sending end event");
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut module = AlfredModule::new(MODULE_NAME, env!("CARGO_PKG_VERSION")).await.expect("Failed to create module");
    module.listen(INPUT_TOPIC).await.expect("An error occurred while listening");
    let sda_pin = module.config.get_module_value("sda")
        .map_or(17u32, |s| s.parse().expect("Failed to parse sda property"));
    let scl_pin = module.config.get_module_value("scl")
        .map_or(27u32, |s| s.parse().expect("Failed to parse sda property"));

    let led_matrix = LedMatrix::new(sda_pin, scl_pin);

    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    let connection = module.connection.clone();

    tokio::spawn(async move {
        led_matrix.handle_alfred_incoming(receiver, &connection).await;
    });

    loop {
        let (_, message) = module.receive().await.expect("An error occurred while receiving a message");
        sender.send(message).await.expect("An error occurred while sending a message");
    }
}

