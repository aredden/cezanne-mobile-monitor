use crate::load_json::read_json;
use std::collections;
use serde_json::{Value};
use core::clone::Clone;

static mut JSON_DATA: Option<Value> = None;
pub fn get_cezanne_data() -> collections::HashMap<String, String> {
    let mut data = collections::HashMap::new();
    unsafe {
        if JSON_DATA.is_none() {
            let data_read = read_json("INSERT/YOUR/JSON/PATH/HERE!!".to_owned());
            JSON_DATA = Some(data_read);
        }
        let mut read_data: Option<Value> = None;
        Clone::clone_from(&mut read_data,&JSON_DATA);
        match read_data {
            Some(data_read) => {
                let d = Value::from(data_read);
                let datas = d.as_object();
                for (k, v) in datas.unwrap() {
                    if v.is_string() {
                        let z = v.as_str().unwrap();
                        data.insert(k.to_owned() as String, z.to_owned());
                    }
                }
                
            },
            None => {
                panic!("SHIT");
            }
        }
    }
    data
}