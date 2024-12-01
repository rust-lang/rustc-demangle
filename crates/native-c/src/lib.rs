use std::ffi::{c_char, c_int};

/// struct demangle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDemangle {
    style: c_int,
    mangled: *const c_char,
    mangled_len: usize,
    elements: usize,
    // 32
    original: *const c_char,
    original_len: usize,
    suffix: *const c_char,
    suffix_len: usize,
}

impl CDemangle {
    /// Create an empty `struct demangle`
    pub fn zero() -> Self {
        Self {
            style: 0,
            mangled: core::ptr::null(),
            mangled_len: 0,
            elements: 0,
            original: core::ptr::null(),
            original_len: 0,
            suffix: core::ptr::null(),
            suffix_len: 0,
        }
    }
}

extern "C" {
    /// call rust_demangle_demangle
    pub fn rust_demangle_demangle(s: *const c_char, res: *mut CDemangle);
    /// call rust_demangle_display_demangle
    pub fn rust_demangle_display_demangle(
        res: *const CDemangle,
        out: *mut c_char,
        len: usize,
        alternate: bool,
    ) -> c_int;
}
