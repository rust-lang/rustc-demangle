fn main() {
    cc::Build::new()
        .file("src/demangle.c")
        .include("include")
        .compile("demangle_native_c");
    println!("cargo::rerun-if-changed=src/demangle.c");
    println!("cargo::rerun-if-changed=include/demangle.h");
}
