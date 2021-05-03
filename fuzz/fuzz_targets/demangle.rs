#![no_main]
use libfuzzer_sys::fuzz_target;
use std::fmt::Write;

fuzz_target!(|data: &str| {
    let mut s = String::new();
    let sym = rustc_demangle::demangle(data);
    drop(write!(s, "{}", sym));
    s.truncate(0);

    if let Ok(sym) = rustc_demangle::try_demangle(data) {
        drop(write!(s, "{}", sym));
    }
});
