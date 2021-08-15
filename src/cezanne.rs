use crate::load_json::read_json;
use core::clone::Clone;
use serde_json::Value;
use std::collections::HashMap;

static mut JSON_DATA: Option<Value> = None;
pub fn get_cezanne_data(path: &str) -> HashMap<String, String> {
    let mut data = HashMap::new();
    unsafe {
        if JSON_DATA.is_none() {
            let data_read = read_json(path.to_owned());
            JSON_DATA = Some(data_read);
        }
        let mut read_data: Option<Value> = None;
        Clone::clone_from(&mut read_data, &JSON_DATA);
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
            }
            None => {
                panic!("SHIT");
            }
        }
    }
    data
}
