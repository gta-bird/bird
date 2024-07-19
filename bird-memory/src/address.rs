//! Memory addresses

use std::ffi::c_void;

/// A memory address
/// 
/// # Examples
/// 
/// ```rust
/// use bird_memory::address::Address;
/// 
/// let address = Address::new(0x12345678);
/// ```
pub struct Address {
    pub raw_address: usize,
}

impl Address {
    pub fn new(address: usize) -> Self {
        Self {
            raw_address: address
        }
    }

    pub fn from_ptr(address: *const c_void) -> Self {
        Self::new(address as usize)
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.raw_address as *const c_void
    }
}

impl Into<usize> for Address {
    fn into(self) -> usize {
        self.raw_address
    }
}

impl Into<*const c_void> for Address {
    fn into(self) -> *const c_void {
        self.as_ptr()
    }
}

impl Into<*mut c_void> for Address {
    fn into(self) -> *mut c_void {
        self.as_ptr() as *mut c_void
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address({:#x})", self.raw_address)
    }
}
