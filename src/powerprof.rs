#[cfg(windows)] extern crate winapi;
use winapi::{
    shared::minwindef::{BOOL, DWORD, FALSE, TRUE, PUINT},
    um::{powrprof}
};

fn get_power_information() -> Result<(u32, u32), &'static str> {
    let mut d:PUINT = 0 as PUINT;
    let result = powrprof::GetActivePwrScheme(&d);
    if result == FALSE {
        panic!("GetActivePwrScheme failed");
    } else {

    }
}