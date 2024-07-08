use std::thread;
use winapi::shared::minwindef::{self, HINSTANCE, DWORD, LPVOID, BOOL};

pub(crate) mod hook;

struct ModuleHandle(HINSTANCE);
unsafe impl Send for ModuleHandle {}

fn entry(module_handle: ModuleHandle) {
    let _ = module_handle.0;
}

#[dllmain_rs::entry]
fn dllmain() {
    let module_handle = ModuleHandle(dll_module);

    thread::spawn(move || {
        entry(module_handle);
    });
}