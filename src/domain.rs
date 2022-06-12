use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Deposit {
    pub id: String,
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}

impl Deposit {
    pub fn as_json(&self) -> String {
        json!({ "Command": { "Deposit": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Deposited {
    pub id: String,
    pub deposit_id: String,
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}

impl Deposited {
    pub fn as_json(&self) -> String {
        json!({
            "Event": { "Deposited": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Withdraw {
    pub id: String,
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}


impl Withdraw {
    pub fn as_json(&self) -> String {
        json!({ "Command": { "Withdraw": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawInsufficientFunds {
    pub id: String,
    pub withdraw_id: String,
    pub time: String,
    pub sequence: u32,
}

impl WithdrawInsufficientFunds {
    pub fn as_json(&self) -> String {
        json!({ "Event": { "WithdrawInsufficientFunds": self } }).to_string()
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Open {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub balance: u32,
    pub time: String
}

impl Open {
    pub fn as_json(&self) -> String {
        json!({ "Command": { "Open": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum Command {
    Open(Open),
    Deposit(Deposit),
    Withdraw(Withdraw),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Withdrawn {
    pub id: String,
    pub withdraw_id: String,
    pub account_id: String,
    pub amount: u32,
    pub time: String,
    pub sequence: u32,
}

impl Withdrawn {
    pub fn as_json(&self) -> String {
        json!({
            "Event": { "Withdrawn": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Opened {
    pub id: String,
    pub open_id: String,
    pub account_id: String,
    pub name: String,
    pub balance: u32,
    pub time: String,
    pub sequence: u32,
}

impl Opened {
    pub fn as_json(&self) -> String {
        json!({
            "Event": { "Opened": self } }).to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    Opened(Opened),
    Deposited(Deposited),
    Withdrawn(Withdrawn),
    WithdrawInsufficientFunds(WithdrawInsufficientFunds),
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Event(Event),
    Command(Command),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: u32,
    pub created_at: String,
    pub sequence: u32
}
