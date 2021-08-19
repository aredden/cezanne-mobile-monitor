use serde_json::{from_reader, Value};
use std::fs::File;
use glob;


pub fn get_smu_offsets_path(smu_version: &str) -> String {
    let mut result_value:Option<String> = None;
    let map_paths = format!("offset_maps/*{}.json",smu_version);
    for entry in glob::glob(&map_paths).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let pbufstr = path.to_str().unwrap().to_owned();
                result_value = Some(pbufstr);
                break;
            },
            Err(e) => {
                panic!("Couldn't read file at path! {:?}",e);
            },
        }
    }
    match result_value {
        Some(value) => {
            value
        },
        None => {
            panic!("Couldn't find a matching offsets file for your smu table version!")
        }
    }
}

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
