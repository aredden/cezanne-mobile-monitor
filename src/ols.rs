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

const ReadPciConfigDwordEx: &'static [u8] = b"ReadPciConfigDwordEx";
const WritePciConfigDwordEx: &'static [u8] = b"WritePciConfigDwordEx";
const InitializeOls: &'static [u8] = b"InitializeOls";
const Cpuid: &'static [u8] = b"Cpuid";

impl Ols {
    pub fn new() -> Result<Ols, Error> {
        let lib = unsafe {
            match libloading::Library::new("WinRing0x64.dll") {
                Ok(val) => val,
                Err(e) => return Err(Error::DllNotFound)
            }
        };

        let (func_initialize, func_cpuid, func_read, func_write) = unsafe {
            let func_read: libloading::Symbol<FnRead> = lib.get(ReadPciConfigDwordEx).map_err(|err| Error::DllIncorrectVersion)?;
            let func_write: libloading::Symbol<FnWrite> = lib.get(WritePciConfigDwordEx).map_err(|err| Error::DllIncorrectVersion)?;
            let func_initialize: libloading::Symbol<FnInitialize> = lib.get(InitializeOls).map_err(|err| Error::DllIncorrectVersion)?;
            let func_cpuid: libloading::Symbol<FnCpuid> = lib.get(Cpuid).map_err(|err| Error::DllIncorrectVersion)?;


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
            let fnc:libloading::Symbol<FnInitialize> = self._lib.get(InitializeOls).unwrap();
            fnc()
        }
    }

    pub fn WritePciConfigDwordEx(&self, pciAddress: u32, regAddress: u32, value: u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnWrite> = self._lib.get(WritePciConfigDwordEx).unwrap();
            fnc(pciAddress, regAddress, value)
        }
    }

    pub fn ReadPciConfigDwordEx(&self, pciAddress: u32, regAddress: u32, value: &mut u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnRead> = self._lib.get(ReadPciConfigDwordEx).unwrap();
            fnc(pciAddress, regAddress, value)
        }
    }

    pub fn Cpuid(&self, index: u32, eax: &mut u32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32) -> i32 {
        unsafe {
            let fnc:libloading::Symbol<FnCpuid> = self._lib.get(Cpuid).unwrap();
            fnc(index, eax, ebx, ecx, edx)
        }
    }
}


