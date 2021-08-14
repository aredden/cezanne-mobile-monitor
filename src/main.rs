use std::{thread, time};
mod smu;
mod ols;
mod load_json;
mod cezanne;
use regex::Regex;
use lazy_static::{lazy_static};
use crate::ols::Ols; //, smu};
use crate::smu::{Smu, read_float};
use crate::cezanne::get_cezanne_data;

use std::time::Duration;

#[macro_use]
extern crate json;
use json::JsonValue;

#[derive(Debug)]
enum Unit {
    Celsius,
    Watt,
    Mhz,
}

#[derive(Debug)]
struct MonitoringItem {
    description: String,
    unit: Unit,
    offset: u32,
    value: f32
}

fn is_cezanne(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"5\d00").unwrap();
    }
    RE.is_match(text)
}

impl MonitoringItem {
    pub fn new(description: String, unit: Unit, offset: u32) -> MonitoringItem {
        MonitoringItem { description, unit, offset, value: 0.0}
    }

    pub fn update(&mut self, addr: u32) {
        self.value = read_float(addr, self.offset);
    }
}

fn main() {

    let ols = match Ols::new() {
        Ok(val) => val,
        Err(e) => panic!("Error happened:{:?}", e)
    };

    let smu = Smu::new(ols);
    let init = smu.write_reg(crate::smu::PSMU_ADDR_RSP, 0x1); //Initialize
    let cpu_name = smu.cpu_name();

    if !is_cezanne(&cpu_name) {
        panic!("CPU is not cezanne!");
    }

    println!("init={}", init);
    println!("cpu_name={}", &cpu_name);
    
    let mut args: Vec<u32> = vec![0, 0, 0, 0, 0, 0];
    smu.send_psmu(0x66, &mut args);
    let address = args[0];
    args[0] = 0;
    println!("address={}", address);

    /*smu.send_psmu(0x65, &mut args);
    thread::sleep(Duration::from_millis(100));

    let table_value = smu::read_float(address, 768);
    let tableVersion:u32 = match table_value {
        0.0 => 0x00370005,
        _ => 0x00370004
    };*/
    
    fn build_monit_item(name: String, unit: Unit) -> MonitoringItem {
        let cdata = get_cezanne_data();    

        let offset = cdata.get(&name);
        if offset.is_none(){
            panic!("{} not found", &name);
        }
        let offset_int = offset.unwrap().parse::<u32>().unwrap();
        MonitoringItem::new(name,unit,offset_int)
    }

    let mut items = vec![
        build_monit_item(String::from("STAPM_LIMIT"), Unit::Watt),
        build_monit_item(String::from("STAPM_VALUE"), Unit::Watt),

        build_monit_item(String::from("PPT_LIMIT_FAST"), Unit::Watt),
        build_monit_item(String::from("PPT_VALUE_FAST"), Unit::Watt),
        build_monit_item(String::from("PPT_LIMIT_SLOW"), Unit::Watt),
        build_monit_item(String::from("PPT_VALUE_SLOW"), Unit::Watt),
        
        build_monit_item(String::from("THM_LIMIT_CORE"), Unit::Celsius),
        build_monit_item(String::from("THM_VALUE_CORE"), Unit::Celsius),

        build_monit_item(String::from("CORE_FREQ_0"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_1"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_2"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_3"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_4"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_5"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_6"), Unit::Mhz),
        build_monit_item(String::from("CORE_FREQ_7"), Unit::Mhz),
        
        build_monit_item(String::from("CORE_TEMP_0"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_1"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_2"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_3"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_4"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_5"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_6"), Unit::Celsius),
        build_monit_item(String::from("CORE_TEMP_7"), Unit::Celsius),

        
        build_monit_item(String::from("CORE_TEMP_6"), Unit::Mhz),
        build_monit_item(String::from("CORE_TEMP_7"), Unit::Mhz),

        build_monit_item(String::from("StapmTimeConstant"), Unit::Celsius),
        build_monit_item(String::from("SlowPPTTimeConstant"), Unit::Celsius),

    ];

    loop {
        smu.send_psmu(0x65, &mut args);
        thread::sleep(Duration::from_millis(100));
        let mut other_arr: Vec<JsonValue> = Vec::new();
        for item in &mut items {

            item.update(address);
            let data = object!{
                description: item.description.to_string(),
                offset: item.offset.to_string(),
                value: item.value.to_string()
            };
            // let stri = json::stringify(data).to_owned();
            other_arr.push(data.clone());
        }
        let json_stringable = object!{
            values: other_arr
        };
        let jsonval = json::stringify(json_stringable);
        println!("{}",&jsonval);
        thread::sleep(time::Duration::from_secs(1));
    }
}
