#![no_main]

use std::ffi::{CStr, CString};
use std::fmt::Write;

use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
enum State<'a> {
    Ok(&'a str),
    Overflow,
}

pub struct FormatBuf<'a> {
    buf: &'a mut String,
    max_len: usize,
}

impl<'a> Write for FormatBuf<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        if self.buf.len() + s.len() > self.max_len {
            return Err(std::fmt::Error);
        }
        self.buf.push_str(s);
        Ok(())
    }
}

fn asciify(x: &str) -> String {
    let mut result = String::with_capacity(x.len() * 4);
    for ch in x.chars() {
        if ch.is_ascii() {
            result.push(ch);
        } else {
            write!(&mut result, "\\u{{{:x}}}", ch as u32).ok();
        }
    }
    result
}

fuzz_target!(|data: &[u8]| {
    if data.len() == 0 {
        return;
    }
    let alternate = data[0] % 2 == 0;
    let data = &data[1..];
    let mut str_buf = String::with_capacity(16384);
    let mut buf = [0u8; 4096];
    let mut demangle = rustc_demangle_native_c::CDemangle::zero();
    let state;
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(cs) = CString::new(data) {
            unsafe {
                rustc_demangle_native_c::rust_demangle_demangle(cs.as_ptr(), &mut demangle);
                match rustc_demangle_native_c::rust_demangle_display_demangle(
                    &demangle,
                    buf.as_mut_ptr().cast(),
                    buf.len() / 4,
                    alternate,
                ) {
                    0 => {
                        state = State::Ok(
                            CStr::from_bytes_until_nul(&buf[..])
                                .expect("nul")
                                .to_str()
                                .expect("utf-8"),
                        );
                    }
                    _ => {
                        state = State::Overflow;
                    }
                };
            }
            let rdemangle = rustc_demangle::demangle(s);
            match state {
                State::Overflow => {
                    str_buf.clear();
                    let fmt_buf = &mut FormatBuf {
                        buf: &mut str_buf,
                        max_len: buf.len() / 4 - 4,
                    };
                    let rust_overflowed = if alternate {
                        write!(fmt_buf, "{:#}", rdemangle)
                    } else {
                        write!(fmt_buf, "{}", rdemangle)
                    };
                    if rust_overflowed.is_err() {
                        return; // rust overflowed as well, OK
                    }
                    // call C again with larger buffer. If it fits in an 1020-byte Rust buffer, it will fit in a 4096-byte C buffer
                    let c_demangled = unsafe {
                        match rustc_demangle_native_c::rust_demangle_display_demangle(
                            &demangle,
                            buf.as_mut_ptr().cast(),
                            buf.len(),
                            alternate,
                        ) {
                            0 => CStr::from_bytes_until_nul(&buf[..])
                                .expect("nul")
                                .to_str()
                                .expect("utf-8"),
                            _ => {
                                panic!("overflow again");
                            }
                        }
                    };
                    assert_eq!(asciify(&str_buf), asciify(c_demangled));
                    if c_demangled.len() < buf.len() / 4 - 3 {
                        panic!(
                            "spurious overflow {} {:?} {:?} {:?} {}",
                            c_demangled.len(),
                            alternate,
                            asciify(&str_buf),
                            asciify(c_demangled),
                            buf.len() / 4
                        )
                    }
                }
                State::Ok(demangled) => {
                    let fmt_buf = &mut FormatBuf {
                        buf: &mut str_buf,
                        max_len: buf.len() / 4 - 4,
                    };
                    let rust_overflowed = if alternate {
                        write!(fmt_buf, "{:#}", rdemangle)
                    } else {
                        write!(fmt_buf, "{}", rdemangle)
                    };
                    if rust_overflowed.is_err() {
                        panic!("rust overflowed 1020 but C output is <1024");
                    }
                    assert_eq!(
                        (alternate, asciify(&str_buf)),
                        (alternate, asciify(demangled))
                    );
                }
            }
        }
    }
});
