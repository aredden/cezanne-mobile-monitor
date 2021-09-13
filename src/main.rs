use crate::load_json::get_smu_offsets_path;
use serde_json::{to_string, to_value, Map, Value};
use std::{thread, time};
mod cezanne;
mod load_json;
mod ols;
mod smu;
mod cli;
use crate::cli::{cli,CliOptions};
use crate::cezanne::get_offset_data;
use crate::ols::Ols;
use crate::smu::{read_float, Smu};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::Duration;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
enum Unit {
    Celsius,
    Watt,
    Mhz,
}

#[derive(Debug, Deserialize, Serialize)]
struct MonitoringItem {
    description: String,
    unit: Unit,
    offset: u32,
    value: f32,
}

fn is_cezanne(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"5\d00").unwrap();
    }
    RE.is_match(text)
}

impl MonitoringItem {
    pub fn new(description: String, unit: Unit, offset: u32) -> MonitoringItem {
        MonitoringItem {
            description,
            unit,
            offset,
            value: 0.0,
        }
    }

    pub fn update(&mut self, addr: u32) {
        self.value = read_float(addr, self.offset);
    }
}

static mut TABLE_JSON_PATH: Option<String> = None;
static mut RUN_TYPE: CliOptions = CliOptions::Run;
fn initols()->Ols {
    let ols = match Ols::new() {
        Ok(val) => val,
        Err(e) => panic!("Error happened:{:?}", e),
    };
    ols
}

fn main() {
    let run_type = cli();
    unsafe {
        RUN_TYPE = run_type.clone();
    }
    let ols = initols();
    let smu = Smu::new(ols);
    let init = smu.write_reg(crate::smu::PSMU_ADDR_RSP, 0x1); //Initialize
    let cpu_name = smu.cpu_name();

    if !is_cezanne(&cpu_name) {
        panic!("CPU is not cezanne!");
    }

    if run_type.clone() == CliOptions::Run {
        println!("init={}", init);
        println!("cpu_name={}", &cpu_name);
    }

    let mut args: Vec<u32> = vec![0, 0, 0, 0, 0, 0];

    smu.send_psmu(0x66, &mut args);

    let smu_base_addr = args[0];

    args[0] = 0;

    if run_type.clone() == CliOptions::Run {
        println!("address={:X}", &smu_base_addr);
    }

    let smu_version = smu.get_pmtable_version(None);

    if run_type.clone() == CliOptions::Table {
        println!("version={}", &smu_version);
        return;
    } else if run_type.clone() == CliOptions::Query{
        println!("version={}", &smu_version);
    }

    let path = get_smu_offsets_path(&smu_version.as_str());
    if run_type.clone() != CliOptions::Query {
        println!("jsonpath={}",&path);
    }
    unsafe {
        TABLE_JSON_PATH = Some(path);

    };
    fn build_monit_item(name: String, unit: Unit) -> MonitoringItem {
        let mut tjpath = "".to_owned();
        unsafe {
            match &TABLE_JSON_PATH {
                Some(path) => {
                    tjpath.clone_from(&path);
                    let silent = RUN_TYPE.clone() != CliOptions::Query;
                    let cdata = get_offset_data(&tjpath, silent);
                    let offset = cdata.get(&name);
                    if offset.is_none() {
                        panic!("{} not found", &name);
                    }
                    let offset_int = offset.unwrap().parse::<u32>().unwrap();
                    MonitoringItem::new(name, unit, offset_int)
                }
                None => {
                    panic!("Usage: --path <path>");
                }
            }
        }
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
        build_monit_item(String::from("StapmTimeConstant"), Unit::Celsius),
        build_monit_item(String::from("SlowPPTTimeConstant"), Unit::Celsius),
    ];

    loop {
        let mut json_map = Map::new();
        smu.send_psmu(0x65, &mut args);
        let mut items_list = vec![];
        thread::sleep(Duration::from_millis(100));
        for item in &mut items {
            item.update(smu_base_addr);
            items_list.push(to_value(&item).unwrap());
        }
        json_map.insert(String::from("values"), Value::from(items_list));
        println!("{}", to_string(&json_map).unwrap());
        if run_type.clone() == CliOptions::Query {
            return;
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
