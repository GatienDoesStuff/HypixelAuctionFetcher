use std::{fs, path::Path};
use rusqlite::{Connection, params};
use serde_json::Value;


pub fn init() {

    match Path::new("./data").exists() {
        true => (),
        false => fs::create_dir("./data").expect("Couldn't create data directory. Check if you have permissions to write in the current directory")
    }

    let conn = Connection::open("./data/auctions.db").expect("Something went wrong trying to open/create the database");

    conn.execute(
        "create table if not exists active_auctions (
             auction_uuid text primary key not null,
             ends integer,
             item_id text,
             item_name text,
             item_bytes text,
             price integer
         )",
        [],
    ).expect("Something went wrong, please report upstream");

    conn.execute(
        "create table if not exists ended_auctions (
             auction_uuid text primary key not null,
             timestamp integer,
             item_id text,
             item_name text,
             item_bytes text,
             sold_for integer
         )",
        [],
    ).expect("Something went wrong, please report upstream");
}



pub fn append(auctions:&Vec<Value>) {

    let mut conn = Connection::open("./data/auctions.db").expect("Something went wrong trying to open the database");

    let tx = conn.transaction().expect("Couldn't create transaction");

    println!("doing database stuff");
    for page in auctions {
        for sale in page["auctions"].as_array().expect("The json provided by the API is borked") {
            let _ = tx.execute(
                "insert into active_auctions (auction_uuid, ends, item_id, item_name, item_bytes, price) values (?1, ?2, ?3, ?4, ?5, ?6)",
                params![sale["uuid"].as_str().unwrap(),
                        sale["end"].as_u64().unwrap(),
                        sale["extra"].as_str().unwrap(),
                        sale["item_name"].as_str().unwrap(),
                        sale["item_bytes"].as_str().unwrap(),
                        sale["starting_bid"].as_u64().unwrap()],);
            }
    }
    tx.commit().unwrap(); //If this returns an error, blame me
}
