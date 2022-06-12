mod command_handler;

use muko_bank::kafka::{consumer, DEFAULT_GROUP};

fn main() {
    if let Err(e) = consumer::consume_messages(command_handler::handler, DEFAULT_GROUP) {
        println!("Failed consuming messages: {}", e);
    }
}
