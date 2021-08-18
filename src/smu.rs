use std::time::Duration;
use crate::ols::Ols;
use std::{thread, time};

pub struct Smu {
    _ols: Ols
}

#[derive(Debug)]
pub enum Status {
    Bad,
    Ok,
    Failed,
    UnknownCmd,
    CmdRejectedPreReq,
    CmdRejectedBusy
}

pub const SMU_PCI_ADDR: u32 = 0x00000000;
pub const SMU_OFFSET_ADDR: u32 = 0xB8;
pub const SMU_OFFSET_DATA: u32 = 0xBC;

pub const PSMU_ADDR_MSG:u32 = 0x03B10A20;
pub const PSMU_ADDR_RSP:u32 = 0x03B10A80;
pub const PSMU_ADDR_ARG:u32 = 0x03B10A88;

fn convert_status(status: u32) -> Status {
    match status {
        0x0 => Status::Bad,
        0x1 => Status::Ok,
        0xFF => Status::Failed,
        0xFE => Status::UnknownCmd,
        0xFD => Status::CmdRejectedPreReq,
        0xFC => Status::CmdRejectedBusy,
        _ => unimplemented!()
    }
}
extern crate libloading as lib;

pub fn read_float(address: u32, offset: u32) -> f32 {
    let mut data = 0;
    get_phys_long(address + offset, &mut data);
    unsafe { return std::mem::transmute::<u32, f32>(data); }
}

pub fn get_phys_long(address: u32, data: &mut u32) -> bool {
    unsafe {
        let lib = lib::Library::new("inpoutx64.dll").unwrap();
        let func: lib::Symbol<unsafe extern "stdcall" fn(u32, *mut u32) -> bool> = lib.get(b"GetPhysLong").unwrap();
        return func(address, data);
    }
}

impl Smu {
    pub fn new(ols: Ols) -> Smu {
        Smu { _ols: ols}
    }

    pub fn read_reg(&self, addr:u32, value: &mut u32) -> bool {
        match self._ols.write_pci_config_dword_ex(SMU_PCI_ADDR, SMU_OFFSET_ADDR, addr) {
            1 => self._ols.read_pci_config_dword_ex(SMU_PCI_ADDR, SMU_OFFSET_DATA, value) == 1,
            _ => false
        }
    }

    pub fn write_reg(&self, addr: u32, data: u32) -> bool {
        match self._ols.write_pci_config_dword_ex(SMU_PCI_ADDR, SMU_OFFSET_ADDR, addr)  {
            1 => self._ols.write_pci_config_dword_ex(SMU_PCI_ADDR, SMU_OFFSET_DATA, data) == 1,
            _ => false
        }
    }

    //TODO: optimize this
    fn wait4rsp(&self, smu_addr_rsp: u32) -> bool {
        let mut result = false;
        let timeout = 1000;
        let mut data = 0;
        while (!result || data == 0) && --timeout > 0 {
            result = self.read_reg(smu_addr_rsp, &mut data);
            thread::sleep(time::Duration::from_millis(1));
        }
        if timeout == 0 || data != 1 { return false; }
        result
    }

    fn send_msg(&self, smu_addr_msg: u32, smu_addr_rsp: u32, smu_addr_arg: u32, message: u32, args: &mut Vec<u32>) -> Status {
        self.wait4rsp(smu_addr_rsp);
        let mut status = 0;
        self.write_reg(smu_addr_rsp, 0x0);
        for i in 0..6 {
            self.write_reg(smu_addr_arg + (i * 4) as u32, *args.get(i).unwrap_or(&0));
        }

        self.write_reg(smu_addr_msg, message);

        self.wait4rsp(smu_addr_rsp);

        for i in 0..6 {
            self.read_reg(smu_addr_arg + i * 4, args.get_mut(i as usize).unwrap());
        }

        self.read_reg(smu_addr_rsp, &mut status);

        return convert_status(status);
    }

    pub fn send_psmu(&self, message: u32, args: &mut Vec<u32>) -> Status {
        return self.send_msg(PSMU_ADDR_MSG, PSMU_ADDR_RSP, PSMU_ADDR_ARG, message, args);
    }

    fn uint_to_str(val: u32) -> String {
        match val {
            0 => "".to_owned(),
            _ => char::from_u32(val).unwrap().to_string()
        }
    }
    fn int_to_str(val: u32) -> String {
        let part1 = val & 0xFF;
        let part2 = val >> 8 & 0xFF;
        let part3 = val >> 16 & 0xFF;
        let part4 = val >> 24 & 0xFF;

        return format!("{}{}{}{}", Smu::uint_to_str(part1), Smu::uint_to_str(part2), Smu::uint_to_str(part3), Smu::uint_to_str(part4));
    }

    /*
    *  Gets the current version of the SMU table, which allows us to map 
    *  to the correct SMU offsets.
    *  {@param _do_not_use:Option<u8>}: DO NOT PROVIDE THIS PARAMETER, 
    *   ITS ONLY FOR WHEN THE FUNCTION IS CALLED ON THE FIRST RECURSIVE RETRY.
    */
    pub fn get_pmtable_version(&self,_do_not_use:Option<u8>) -> String {

        let mut smu_version_args: Vec<u32> = vec![0, 0, 0, 0, 0, 0];
        match self.send_psmu(0x6, &mut smu_version_args){
            Status::Ok => {
                return format!("{:X}",smu_version_args[0]);
            },
            Status::Failed => {
                print!("Failed to get PMU version");
            },
            Status::UnknownCmd => {
                print!("Unknown command");
            },
            Status::CmdRejectedPreReq => {
                print!("Command rejected because prerequisites not met");
            },
            Status::CmdRejectedBusy => {
                print!("Command rejected because the SMU is busy");
            },
            Status::Bad => {
                print!("Bad status when reading PMU version");
            }
        };
        if let Some(_do_not_use) = _do_not_use {
            panic!("Could not find pmtable version!");
        } else {
            thread::sleep(Duration::from_millis(100));
            self.get_pmtable_version(Some(1))
        }
    }

    pub fn cpu_name(&self) -> String {
        let mut name = String::new();
        let mut eax = 0;
        let mut ebx = 0;
        let mut ecx = 0;
        let mut edx = 0;

        if self._ols.cpuid(0x80000002, &mut eax, &mut ebx, &mut ecx, &mut edx) == 1 {
            name.push_str(Smu::int_to_str(eax).as_str());
            name.push_str(Smu::int_to_str(ebx).as_str());
            name.push_str(Smu::int_to_str(ecx).as_str());
            name.push_str(Smu::int_to_str(edx).as_str());
        }

        if self._ols.cpuid(0x80000003, &mut eax, &mut ebx, &mut ecx, &mut edx) == 1 {
            name.push_str(Smu::int_to_str(eax).as_str());
            name.push_str(Smu::int_to_str(ebx).as_str());
            name.push_str(Smu::int_to_str(ecx).as_str());
            name.push_str(Smu::int_to_str(edx).as_str());
        }

        if self._ols.cpuid(0x80000004, &mut eax, &mut ebx, &mut ecx, &mut edx) == 1 {
            name.push_str(Smu::int_to_str(eax).as_str());
            name.push_str(Smu::int_to_str(ebx).as_str());
            name.push_str(Smu::int_to_str(ecx).as_str());
            name.push_str(Smu::int_to_str(edx).as_str());
        }

        return name;
    }
}