#![allow(dead_code)]

use std::{error::Error, ffi::c_void, fmt::Debug};
use minhook_sys::*;

pub struct MHStatus(MH_STATUS);

impl Error for MHStatus {}
impl Debug for MHStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MH_STATUS({})", self.to_string_friendly())
    }
}
impl std::fmt::Display for MHStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MH_STATUS({})", self.to_string_friendly())
    }
}

impl MHStatus {
    pub fn to_result(self) -> Result<(), Self> {
        if self.0 == MH_OK {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub fn to_string_friendly(&self) -> String {
        match self.0 {
            MH_UNKNOWN => "MH_UNKNOWN".to_string(),
            MH_OK => "MH_OK".to_string(),
            MH_ERROR_ALREADY_INITIALIZED => "MH_ERROR_ALREADY_INITIALIZED".to_string(),
            MH_ERROR_NOT_INITIALIZED => "MH_ERROR_NOT_INITIALIZED".to_string(),
            MH_ERROR_ALREADY_CREATED => "MH_ERROR_ALREADY_CREATED".to_string(),
            MH_ERROR_NOT_CREATED => "MH_ERROR_NOT_CREATED".to_string(),
            MH_ERROR_ENABLED => "MH_ERROR_ENABLED".to_string(),
            MH_ERROR_DISABLED => "MH_ERROR_DISABLED".to_string(),
            MH_ERROR_NOT_EXECUTABLE => "MH_ERROR_NOT_EXECUTABLE".to_string(),
            MH_ERROR_UNSUPPORTED_FUNCTION => "MH_ERROR_UNSUPPORTED_FUNCTION".to_string(),
            MH_ERROR_MEMORY_ALLOC => "MH_ERROR_MEMORY_ALLOC".to_string(),
            MH_ERROR_MEMORY_PROTECT => "MH_ERROR_MEMORY_PROTECT".to_string(),
            MH_ERROR_MODULE_NOT_FOUND => "MH_ERROR_MODULE_NOT_FOUND".to_string(),
            MH_ERROR_FUNCTION_NOT_FOUND => "MH_ERROR_FUNCTION_NOT_FOUND".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

macro_rules! test_mh_status {
    ($ret:expr) => {
        unsafe {
            MHStatus($ret).to_result()
        }
    };
}

pub fn initialize() -> Result<(), MHStatus> {
    test_mh_status!(MH_Initialize())
}

pub fn enable_hook(target: *mut c_void)  -> Result<(), MHStatus> {
    test_mh_status!(MH_EnableHook(target))
}

pub fn disable_hook(target: *mut c_void) -> Result<(), MHStatus> {
    test_mh_status!(MH_DisableHook(target))
}

pub fn remove_hook(target: *mut c_void) -> Result<(), MHStatus> {
    test_mh_status!(MH_RemoveHook(target))
}

pub fn enable_all() -> Result<(), MHStatus> {
    test_mh_status!(MH_EnableHook(std::ptr::null_mut()))
}

pub fn create_hook(target: *mut c_void, hook: *mut c_void) -> Result<*mut c_void, MH_STATUS> {
    unsafe {
        let mut original: *mut c_void = std::ptr::null_mut();
        let ret = MH_CreateHook(target, hook, &mut original);

        if ret == MH_OK {
            Ok(original)
        } else {
            Err(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test() {
        extern "C" fn placeholder(val: i32) -> i32 {
            val + 1
        }

        extern "C" fn hook(_: i32) -> i32 {
            1337
        }

        assert_eq!(placeholder(1), 2, "placeholder should return 2");

        initialize().expect("failed to initialize minhook");
        let original = create_hook(placeholder as *mut c_void, hook as *mut c_void).unwrap();
        enable_all().expect("failed to enable all hooks");
        let original = unsafe {
            std::mem::transmute::<*mut c_void, extern "C" fn(i32) -> i32>(original)
        };

        assert_eq!(placeholder(1), 1337, "hook should return 1337");
        assert_eq!(original(1), 2, "original should return 2");
        remove_hook(placeholder as *mut c_void).expect("failed to remove hook");
        assert_eq!(placeholder(1), 2, "placeholder should return 2");
    }
}