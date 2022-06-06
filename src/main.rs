mod command_handler;

use muko_bank::{domain::{Open}};

use chrono::prelude::*;

use muko_bank::kafka::{producer, consumer, DEFAULT_TOPIC, DEFAULT_BROKER};

fn main() {
    let data = Open {
        account_id: "123".to_string(),
        name: "123".to_string(),
        time: Utc::now().to_string(),
        balance: 100
    }.as_json();

    if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }

    if let Err(e) = consumer::consume_messages(command_handler::handler) {
        println!("Failed consuming messages: {}", e);
    }
}
