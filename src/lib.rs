use winapi::{
    shared::minwindef::{BOOL, DWORD, FALSE, HMODULE, LPVOID, TRUE},
    um::{libloaderapi::DisableThreadLibraryCalls, winnt::DLL_PROCESS_ATTACH},
};

#[macro_use]
extern crate lazy_static;
extern crate winapi;

mod helveta;
use crate::helveta::helveta::*;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    module: HMODULE,
    reason_for_call: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if reason_for_call != DLL_PROCESS_ATTACH {
        return FALSE;
    }

    unsafe {
        DisableThreadLibraryCalls(module);

        let instrument = Context::new();
        instrument.run();
    }

    //	Succesful injection
    TRUE
}
