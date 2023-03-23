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

    let mut output = Vec::new();
    drop(rustc_demangle::demangle_stream(
        &mut s.as_bytes(),
        &mut output,
        true,
    ));
    output.clear();
    drop(rustc_demangle::demangle_stream(
        &mut s.as_bytes(),
        &mut output,
        false,
    ));
});
