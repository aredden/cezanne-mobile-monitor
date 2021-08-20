extern crate libloading;

type FnInitialize = unsafe extern "stdcall" fn() -> i32;
type FnCpuid = unsafe extern "stdcall" fn(u32, &mut u32, &mut u32, &mut u32, &mut u32) -> i32;
type FnRead = unsafe extern "stdcall" fn(u32, u32, *mut u32) -> i32;
type FnWrite = unsafe extern "stdcall" fn(u32, u32, u32) -> i32;
type FnDllStatus = unsafe extern "stdcall" fn() -> u32;
type FnDeinitialize = unsafe extern "stdcall" fn() -> ();



pub struct Ols {
    _lib: libloading::Library
}

impl Drop for Ols {
    fn drop(&mut self) {
        unsafe {
            let fnc:libloading::Symbol<FnDeinitialize> = self._lib.get(DEINIT_OLS).unwrap();
            fnc();
        }
    }
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
const DLL_STATUS:&'static [u8] = b"GetDllStatus";
const DEINIT_OLS:&'static [u8] = b"DeinitializeOls";

impl Ols {
    pub fn new() -> Result<Ols, Error> {
        let lib = unsafe {
            match libloading::Library::new("WinRing0x64.dll") {
                Ok(val) => val,
                Err(_e) => return Err(Error::DllNotFound)
            }
        };

        let (_func_initialize, _func_cpuid, _func_read, _func_write, _func_deinitialize, _func_dll_status) = unsafe {
            let func_read: libloading::Symbol<FnRead> = lib.get(READ_PCI_CONFIG_DWORD_EX).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_write: libloading::Symbol<FnWrite> = lib.get(WRITE_PCI_CONFIG_DWORD_EX).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_initialize: libloading::Symbol<FnInitialize> = lib.get(INIT_OLS).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_cpuid: libloading::Symbol<FnCpuid> = lib.get(CPUID).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_deinitialize:libloading::Symbol<FnDeinitialize> = lib.get(DEINIT_OLS).map_err(|_err| Error::DllIncorrectVersion)?;
            let func_dll_status: libloading::Symbol<FnDllStatus> = lib.get(DLL_STATUS).map_err(|_err| Error::DllIncorrectVersion)?;
            Ok((func_initialize, func_cpuid, func_read, func_write, func_deinitialize, func_dll_status))
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

    #[allow(dead_code)]
    pub fn deinit(&self) -> () {
        unsafe {
            let fnc:libloading::Symbol<FnDeinitialize> = self._lib.get(DEINIT_OLS).unwrap();
            fnc()
        }
    }

    #[allow(dead_code)]
    pub fn get_dll_status(&self) -> u32 {
        unsafe {
            let fnc:libloading::Symbol<FnDllStatus> = self._lib.get(DLL_STATUS).unwrap();
            let result = fnc();
            print!("DLL Status: {:?}", &result);
            result
        }
    }
}


