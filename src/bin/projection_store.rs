use redis::Commands;

use muko_bank::domain::{Account, Message, Event};
use muko_bank::kafka::{self, PROJECTIONS_GROUP};

fn main () {
    loop {
        consume();
    }


}

pub fn consume() {
    let message_handler = |message| {
        if let Message::Event(event) = message {
            match event {
                Event::Deposited(deposited) => {

                    let account_json: String = get_account(&deposited.account_id);
                    let mut account: Account  = serde_json::from_str(&account_json).unwrap();

                    account.balance += deposited.amount;
                    account.sequence = deposited.sequence;

                    let account_json = serde_json::to_string(&account).unwrap();

                    set_account(&account.id, &account_json);

                },
                Event::Withdrawn(w) => {

                    let account_json: String = get_account(&w.account_id);
                    let mut account: Account  = serde_json::from_str(&account_json).unwrap();

                    account.balance -= w.amount;
                    account.sequence = w.sequence;

                    let account_json = serde_json::to_string(&account).unwrap();

                    set_account(&account.id, &account_json);

                },
                Event::Opened(o) => {
                    let account = Account {
                        id: o.account_id,
                        balance: o.balance,
                        name: o.name,
                        created_at: o.time,
                        sequence: o.sequence,
                    };

                    let account_json = serde_json::to_string(&account).unwrap();

                    set_account(&account.id, &account_json);
                },
                _ => {}
            }
        }
    };

    if let Err(e) = kafka::consumer::consume_messages(message_handler, PROJECTIONS_GROUP) {
        println!("Failed consuming messages: {}", e);
    }
}

fn set_account(account_id: &str, data: &str) {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1:6380").unwrap();
    let mut con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail

    let redis_address = format!("muko_bank::{}", account_id);

    let _ : () = con.set(redis_address, data).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    // con.get()
}

fn get_account(account_id: &str) -> String {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1:6380").unwrap();
    let mut con = client.get_connection().unwrap();
    let redis_address = format!("muko_bank::{}", account_id);
    con.get(redis_address).unwrap()
}
