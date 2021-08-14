use serde_json::Value;
use std::fs::File;
use serde_json::from_reader;

pub fn read_json(file: String) -> Value {
    let file = File::open(file).expect("file should open read only");
    let json: serde_json::Result<Value> = from_reader(file);
    if json.is_err() {
        println!("{:?}", json);
        panic!("Failed to read JSON file");
    }
    json.unwrap()
}
