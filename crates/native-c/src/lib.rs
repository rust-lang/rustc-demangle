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

#[test]
fn smoke_test() {
    fn test_single(input: &str, expected: &str, alternate: bool) {
        use std::ffi::{CStr, CString};

        let mut buf = [0u8; 4096];
        unsafe {
            let mut demangle = CDemangle::zero();
            let cs = CString::new(input).unwrap();
            for output_len in 0..4096 {
                rust_demangle_demangle(cs.as_ptr(), &mut demangle);
                if rust_demangle_display_demangle(
                    &demangle,
                    buf.as_mut_ptr().cast(),
                    output_len,
                    alternate,
                ) != 0
                {
                    continue; // buffer is not big enough
                }
                let output = CStr::from_bytes_until_nul(&buf[..])
                    .expect("nul")
                    .to_str()
                    .expect("utf-8");
                assert_eq!(output, expected);
                // test overflow margin
                assert_eq!(output_len, output.len() + 4);
                return;
            }
            panic!("overflow");
        }
    }
    for (input, normal, alternate) in [
        // test empty string
        ("", "", ""),
        // just a path
        ("_RNvC6_123foo3bar", "123foo::bar", "123foo::bar"),
        // more complex paths
        ("_RNCNCNgCs6DXkGYLi8lr_2cc5spawn00B5_", "cc[4d6468d6c9fd4bb3]::spawn::{closure#0}::{closure#0}", "cc::spawn::{closure#0}::{closure#0}"),
        ("_RINbNbCskIICzLVDPPb_5alloc5alloc8box_freeDINbNiB4_5boxed5FnBoxuEp6OutputuEL_ECs1iopQbuBiw2_3std", "alloc[f15a878b47eb696b]::alloc::box_free::<dyn alloc[f15a878b47eb696b]::boxed::FnBox<(), Output = ()>>", "alloc::alloc::box_free::<dyn alloc::boxed::FnBox<(), Output = ()>>"),
        ("_RMC0INtC8arrayvec8ArrayVechKj7b_E", "<arrayvec::ArrayVec<u8, 123usize>>", "<arrayvec::ArrayVec<u8, 123>>"),
        // punycode
        ("_RNqCs4fqI2P2rA04_11utf8_identsu30____7hkackfecea1cbdathfdh9hlq6y", "utf8_idents[317d481089b8c8fe]::საჭმელად_გემრიელი_სადილი", "utf8_idents::საჭმელად_გემრიელი_სადილი"),
        // string with non-utf8 characters
        ("_RIC0Kef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E",
        "::<{*\"\\u{1f40a}\\u{1f988}\\u{1f986}\\u{1f42e} \\u{a7} \\u{1f436}\\u{1f452}\\u{2615}\\u{1f525} \\u{a7} \\u{1f9e1}\\u{1f49b}\\u{1f49a}\\u{1f499}\\u{1f49c}\"}>",
        "::<{*\"\\u{1f40a}\\u{1f988}\\u{1f986}\\u{1f42e} \\u{a7} \\u{1f436}\\u{1f452}\\u{2615}\\u{1f525} \\u{a7} \\u{1f9e1}\\u{1f49b}\\u{1f49a}\\u{1f499}\\u{1f49c}\"}>"
        ),
        // invalid syntax via backref
        ("_RNvNvB0_1x1y", "{invalid syntax}::x::y", "{invalid syntax}::x::y"),
        // overflow via backref
        ("_RNvNvB1_1x1y",
        "{recursion limit reached}::?::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::y",
        "{recursion limit reached}::?::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::x::y",
        ),
        // native
        ("_ZN9backtrace3foo17hbb467fcdaea5d79bE.llvm.A5310EB9", "backtrace::foo::hbb467fcdaea5d79b", "backtrace::foo"),
        // LLVM suffix
        ("_RNvC6_123foo3bar.llvm.A5310EB9", "123foo::bar", "123foo::bar"),
        ("_ZN9backtrace3foo17hbb467fcdaea5d79bE.llvm.A5310EB9", "backtrace::foo::hbb467fcdaea5d79b", "backtrace::foo"),
        // other suffix
        ("_RNvC6_123foo3bar.i", "123foo::bar.i", "123foo::bar.i"),
        ("_ZN9backtrace3foo17hbb467fcdaea5d79bE.i", "backtrace::foo::hbb467fcdaea5d79b.i", "backtrace::foo.i"),
    ] {
        test_single(input, normal, false);
        test_single(input, alternate, true);
    }
}
