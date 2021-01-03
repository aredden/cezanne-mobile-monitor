#![feature(assoc_char_funcs)]

use std::{thread, time};
mod smu;
mod ols;

use crate::ols::Ols; //, smu};
use crate::smu::{Smu, read_float};
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
    println!("init={}", init);
    println!("cpu_name={}", smu.cpu_name());

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

    let mut items = vec![
        MonitoringItem::new("STAPM LIMIT".to_owned(), Unit::Watt, 0x0),
        MonitoringItem::new("STAPM VALUE".to_owned(), Unit::Watt, 0x4),

        MonitoringItem::new("PPT LIMIT FAST".to_owned(), Unit::Watt, 0x8),
        MonitoringItem::new("PPT VALUE FAST".to_owned(), Unit::Watt, 0xC),
        MonitoringItem::new("PPT LIMIT SLOW".to_owned(), Unit::Watt, 0x10),
        MonitoringItem::new("PPT VALUE SLOW".to_owned(), Unit::Watt, 0x14),

        MonitoringItem::new("THM LIMIT CORE".to_owned(), Unit::Watt, 0x40),
        MonitoringItem::new("THM VALUE CORE".to_owned(), Unit::Watt, 0x44),

        MonitoringItem::new("CORE FREQ 0".to_owned(), Unit::Mhz, 0x3BC),
        MonitoringItem::new("CORE FREQ 1".to_owned(), Unit::Mhz, 0x3C0),
        MonitoringItem::new("CORE FREQ 2".to_owned(), Unit::Mhz, 0x3C4),
        MonitoringItem::new("CORE FREQ 3".to_owned(), Unit::Mhz, 0x3C8),
        MonitoringItem::new("CORE FREQ 4".to_owned(), Unit::Mhz, 0x3CC),
        MonitoringItem::new("CORE FREQ 5".to_owned(), Unit::Mhz, 0x3D0),
        MonitoringItem::new("CORE FREQ 6".to_owned(), Unit::Mhz, 0x3D4),
        MonitoringItem::new("CORE FREQ 7".to_owned(), Unit::Mhz, 0x3D8),

        MonitoringItem::new("CORE TEMP 0".to_owned(), Unit::Celsius, 860),
        MonitoringItem::new("CORE TEMP 1".to_owned(), Unit::Celsius, 864),
        MonitoringItem::new("CORE TEMP 2".to_owned(), Unit::Celsius, 868),
        MonitoringItem::new("CORE TEMP 3".to_owned(), Unit::Celsius, 872),
        MonitoringItem::new("CORE TEMP 4".to_owned(), Unit::Celsius, 876),
        MonitoringItem::new("CORE TEMP 5".to_owned(), Unit::Celsius, 880),
        MonitoringItem::new("CORE TEMP 6".to_owned(), Unit::Celsius, 884),
        MonitoringItem::new("CORE TEMP 7".to_owned(), Unit::Celsius, 888),

        MonitoringItem::new("StapmTimeConstant".to_owned(), Unit::Celsius, 2204),
        MonitoringItem::new("SlowPPTTimeConstant".to_owned(), Unit::Celsius, 2208)
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
