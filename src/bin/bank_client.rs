use std::io;
use chrono::prelude::*;
use terminal_menu::{menu, label, button, run, mut_menu};
use uuid::Uuid;
use std::{thread, time};


use muko_bank::kafka::{producer, DEFAULT_TOPIC, DEFAULT_BROKER};
use muko_bank::projector;
use muko_bank::domain::{Withdraw, Account, Open, Deposit};

use redis::Commands;

fn main() {
    println!("Welcome to MukoBank");
    loop {


        let menu = menu(vec![
            // label:
            //  not selectable, usefule as a title, separator, etc...
            label("----------------------"),
            label("Select Operation"),
            label("-----------------------"),

            // button:
            //  exit the menu
            button("Open"),
            button("Withdraw"),
            button("Deposit"),
        ]);
        run(&menu);

        let operation = mut_menu(&menu).selected_item_name().to_string();

        println!("operation {}", operation);

        match operation.as_str() {
            "Open" => handle_open(),
            "Withdraw" => handle_withdraw(),
            "Deposit" =>  handle_deposit(),
            _ => println!("Invalid operation"),
        }
    }
}

fn handle_deposit () {
    println!("Please input account_id.");

    let mut account_id = String::new();

    io::stdin()
        .read_line(&mut account_id)
        .expect("Failed to read line");

    let account_id = account_id.trim();

    let account = projector::project(account_id);

    if account.is_none() {
        println!("Invalid Account");
        return;
    }



    let account = account.unwrap();

    let mut amount: Option<u32> = None;

    while amount.is_none() {
        let mut input = String::new();

        println!("Amout: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        if let Ok(value) = input.trim().parse::<u32>() {
            amount = Some(value);
        }
    }

    let amount = amount.unwrap();

    let deposit = Deposit {
        id: Uuid::new_v4().to_string(),
        account_id: account_id.to_string(),
        amount,
        time: Utc::now().to_string(),
        sequence: account.sequence,
    };

    if let Err(e) = producer::produce_message(deposit.as_json().as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }


    let mut counter = 0;

    loop {
        counter += 1;

        let redis_address = format!("muko_bank::{}", account_id);

        let account_stored = get_account(&redis_address);

        let account_parsed_result = serde_json::from_str::<Account>(&account_stored);

        let account = if let Ok(account_parsed) = account_parsed_result {
            Some(account_parsed)
        } else {
            None
        };

        if let Some(account) = account  {
            if account.sequence == deposit.sequence + 1 {
                println!("Deposit Success");
                break;
            }
        }

        if counter == 3  {
            println!("Deposit Failed");
            break;
        }

        thread::sleep(time::Duration::from_secs(2));
    };
}

fn handle_withdraw () {
    println!("Please input account_id.");

    let mut account_id = String::new();

    io::stdin()
        .read_line(&mut account_id)
        .expect("Failed to read line");

    let account_id = account_id.trim();

    let account = projector::project(account_id);

    if account.is_none() {
        println!("Invalid Account");
        return;
    }

    let account = account.unwrap();

    let mut amount: Option<u32> = None;

    while amount.is_none() {
        let mut input = String::new();

        println!("Amout: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        if let Ok(value) = input.trim().parse::<u32>() {
            amount = Some(value);
        }
    }

    let amount = amount.unwrap();

    let withdraw = Withdraw {
        id: Uuid::new_v4().to_string(),
        account_id: account_id.to_string(),
        amount,
        time: Utc::now().to_string(),
        sequence: account.sequence,
    };

    if let Err(e) = producer::produce_message(withdraw.as_json().as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }


    let mut counter = 0;

    loop {
        counter += 1;

        let redis_address = format!("muko_bank::{}", account_id);

        let account_stored = get_account(&redis_address);

        let account_parsed_result = serde_json::from_str::<Account>(&account_stored);

        let account = if let Ok(account_parsed) = account_parsed_result {
            Some(account_parsed)
        } else {
            None
        };

        if let Some(account) = account  {
            if account.sequence == withdraw.sequence + 1 {
                println!("Withdraw Success");
                break
            }
        }

        if counter == 3  {
            println!("Withdraw Failed");
            break;
        }

        thread::sleep(time::Duration::from_secs(2));
    };
}

fn handle_open () {
    let id = Uuid::new_v4().to_string();

    let mut name: Option<String> = None;

    while name.is_none() {
        let mut input = String::new();

        println!("name: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        name = Some(input.trim().to_string())
    }

    let name = name.unwrap();

    let data = Open {
        id: Uuid::new_v4().to_string(),
        account_id: id.to_string(),
        name,
        time: Utc::now().to_string(),
        balance: 0
    }.as_json();

    if let Err(e) = producer::produce_message(data.as_bytes(), DEFAULT_TOPIC, vec![DEFAULT_BROKER.to_string()]) {
        println!("Failed producing messages: {}", e);
    }

    let mut account: Option<Account> = None;

    let mut counter = 0;

    loop {
        counter += 1;

        let redis_address = format!("muko_bank::{}", id);

        let account_stored = get_account(&redis_address);

        let account_parsed_result = serde_json::from_str::<Account>(&account_stored);

        if let Ok(account_parsed) = account_parsed_result {
            account = Some(account_parsed)
        }

        if counter == 3 || account.is_some() {
            break
        }

        thread::sleep(time::Duration::from_secs(2));
    };

    if account.is_some() {
        println!("Account {} Created", id);
    } else {
        println!("Error Creating account")
    }

}

fn get_account(account_id: &str) -> String {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1:6380").unwrap();
    let mut con = client.get_connection().unwrap();
    let result = con.get::<&str, String>(account_id);

    match result {
        Ok(value) =>  value,
        _ => "".to_string()
    }
}
