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

fn fuzz(data: &[u8], alternate: bool) {
    let mut str_buf = String::with_capacity(16384);
    let mut buf = [0u8; 4096];
    let mut demangle = rustc_demangle_native_c::CDemangle::zero();
    // We want to allow for easy overflow checking. The C output
    // can be longer than the Rust output by a factor of up to *7/2,
    // since e.g. an 'α' (U+03b1) in a constant string will be encoded
    // as [b'\xce' b'\xb1'] in the C output (2 bytes) but as
    // [b'\\' b'u' b'{' b'3' b'b' b'1' '}'] (7 bytes). The
    // other factors are smaller than that (4 hex digits = 3 utf-8 bytes,
    // leading to a lower expansion factor of *8/3, and so on).
    //
    // Also, to make the fuzzer more easily encounter overflow conditions
    // in the C code, and since for most outputs the output lengths is the
    // same, starting with a similar output length makes it easier.
    let starting_buf_len = buf.len() / 4;
    let state;
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(cs) = CString::new(data) {
            unsafe {
                rustc_demangle_native_c::rust_demangle_demangle(cs.as_ptr(), &mut demangle);
                match rustc_demangle_native_c::rust_demangle_display_demangle(
                    &demangle,
                    buf.as_mut_ptr().cast(),
                    starting_buf_len,
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
                        max_len: starting_buf_len - 4,
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
                    if c_demangled.len() < starting_buf_len - 3 {
                        panic!(
                            "spurious overflow {} {:?} {:?} {:?} {}",
                            c_demangled.len(),
                            alternate,
                            asciify(&str_buf),
                            asciify(c_demangled),
                            starting_buf_len
                        )
                    }
                }
                State::Ok(demangled) => {
                    let fmt_buf = &mut FormatBuf {
                        buf: &mut str_buf,
                        max_len: starting_buf_len - 4,
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
}

fuzz_target!(|data: &[u8]| {
    // fuzz both normal and alternate modes.
    fuzz(data, false);
    fuzz(data, true);
});
