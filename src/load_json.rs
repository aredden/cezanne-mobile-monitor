use serde_json::{from_reader, Value};
use std::fs::File;

pub fn read_json(file: String) -> Value {
    println!("Reading {}", file);
    let file = File::open(file).expect("file should open read only");
    let json: serde_json::Result<Value> = from_reader(file);
    if json.is_err() {
        println!("{:?}", json);
        panic!("Failed to read JSON file");
    }
    json.unwrap()
}
