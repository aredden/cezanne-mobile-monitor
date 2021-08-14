extern crate libloading;

type FnInitialize = unsafe extern "stdcall" fn() -> i32;
type FnCpuid = unsafe extern "stdcall" fn(u32, &mut u32, &mut u32, &mut u32, &mut u32) -> i32;
type FnRead = unsafe extern "stdcall" fn(u32, u32, *mut u32) -> i32;
type FnWrite = unsafe extern "stdcall" fn(u32, u32, u32) -> i32;


pub struct Ols {
    _lib: libloading::Library
}

#[derive(Debug)]
pub enum Error {
    DllNotFound,
    DllIncorrectVersion,
    DllInitializeError
}


const READ_PCI_CONFIG_DWORD_EX: &'static [u8] = b"ReadPciConfigDwordEx";
const WRITE_PCI_CONFIG_DWORD_EX: &'static [u8] = b"WritePciConfigDwordEx";
const INIT_OLS: &'static [u8] = b"InitializeOls";
const CPUID: &'static [u8] = b"Cpuid";

impl Ols {
    pub fn new() -> Result<Ols, Error> {
        let lib = unsafe {
            match libloading::Library::new("WinRing0x64.dll") {
                Ok(val) => val,
                Err(_e) => return Err(Error::DllNotFound)
            }
        };

        let (_func_initialize, _func_cpuid, _func_read, _func_write) = unsafe {
            let func_read: libloading::Symbol<FnRead> = lib.get(READ_PCI_CONFIG_DWORD_EX).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_write: libloading::Symbol<FnWrite> = lib.get(WRITE_PCI_CONFIG_DWORD_EX).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_initialize: libloading::Symbol<FnInitialize> = lib.get(INIT_OLS).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_cpuid: libloading::Symbol<FnCpuid> = lib.get(CPUID).map_err(|_err| Error::DllIncorrectVersion)?;


            Ok((func_initialize, func_cpuid, func_read, func_write))
        }?;

        let ols = Ols { _lib: lib};

        if ols.init() == 0 {
            return Err(Error::DllInitializeError);
        }

        Ok(ols)
    }

    pub fn init(&self) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnInitialize> = self._lib.get(INIT_OLS).unwrap();
            fnc()
        }
    }

    pub fn write_pci_config_dword_ex(&self, pci_address: u32, reg_address: u32, value: u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnWrite> = self._lib.get(WRITE_PCI_CONFIG_DWORD_EX).unwrap();
            fnc(pci_address, reg_address, value)
        }
    }

    pub fn read_pci_config_dword_ex(&self, pci_address: u32, reg_address: u32, value: &mut u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnRead> = self._lib.get(READ_PCI_CONFIG_DWORD_EX).unwrap();
            fnc(pci_address, reg_address, value)
        }
    }

    pub fn cpuid(&self, index: u32, eax: &mut u32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnCpuid> = self._lib.get(CPUID).unwrap();
            fnc(index, eax, ebx, ecx, edx)
        }
    }
}


