use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Withdraw {
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}

impl Withdraw {
    pub fn as_json(&self) -> String {
        json!({
            "Command": {
                "Withdraw": {
                    "account_id": self.account_id,
                    "amount": self.amount,
                    "time": self.time,
                    "sequence": self.sequence,
                }
            }
        }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Open {
    pub account_id: String,
    pub name: String,
    pub balance: u32,
    pub time: String
}

impl Open {
    pub fn as_json(&self) -> String {
        json!({
            "Command": {
                "Open": {
                    "account_id": self.account_id,
                    "name": self.name,
                    "balance": self.balance,
                    "time": self.time
                }
            }
        }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Command {
    Open(Open),
    Withdraw(Withdraw)
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Withdrawn {
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}

impl Withdrawn {
    pub fn as_json(&self) -> String {
        json!({
            "Event": {
                "Withdrawn": {
                    "account_id": self.account_id,
                    "amount": self.amount,
                    "time": self.time,
                    "sequence": self.sequence
                }
            }
        }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Opened {
    pub account_id: String,
    pub name: String,
    pub balance: u32,
    pub time: String,
    pub sequence: u32,
}

impl Opened {
    pub fn as_json(&self) -> String {
        json!({
            "Event": {
                "Opened": {
                    "account_id": self.account_id,
                    "name": self.name,
                    "balance": self.balance,
                    "time": self.time,
                    "sequence": self.sequence
                }
            }
        }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    Opened(Opened),
    Withdrawn(Withdrawn)
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Event(Event),
    Command(Command)
}

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: u32,
    pub created_at: String,
    pub sequence: u32
}
