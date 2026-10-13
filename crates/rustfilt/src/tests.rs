#![cfg(test)]
use super::demangle_stream;
///! Tests demangle_stream, based upon the assumption rustc_demangle works properly
use rustc_demangle::demangle;

static MANGLED_NAMES: &'static [&'static str] = &[
    "_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h510ed05e72307174E",
    "_ZN7example4main17h0db00b8b32acffd5E",
    "_ZN3std2io5stdio6_print17he48522be5b0a80d9E",
    "_ZN3foo17h05af221e174051e9E",
    "_ZN3foo20h05af221e174051e9abcE",
    "_ZN3foo5h05afE",
    "_ZN109_$LT$core..str..pattern..CharSearcher$LT$$u27$a$GT$$u20$as$u20$core..str..pattern..Searcher$LT$$u27$a$GT$$GT$10next_match17h9c8d80a58da7cd74E",
    "_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h98ea4751a6975428E",
    "_ZN51_$LT$serde_json..read..IteratorRead$LT$Iter$GT$$GT$15parse_str_bytes17h8199b7867f1a334fE",
    "_ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17h1bc0dbd302b9f01bE",

    // RFC2603 v0 mangled names
    "_RNvNtNtCs1234_7mycrate3foo3bar3baz",
    "_RNvNvMCs1234_7mycrateINtCs1234_7mycrate3FoopE3bar4QUUX",
    "_RNvNvXCs1234_7mycrateINtCs1234_7mycrate3FoopENtNtC3std5clone5Clone5clone4QUUX",
    "_RNvNvCs1234_7mycrate4QUUX3FOO",
];

fn demangle_line(s: &str, include_hash: bool) -> std::borrow::Cow<str> {
    let mut out = Vec::new();
    demangle_stream(&mut s.as_bytes(), &mut out, include_hash).unwrap();
    std::borrow::Cow::Owned(String::from_utf8(out).unwrap())
}

#[test]
fn ignores_text() {
    for text in &[
        "boom de yada\tboom de yada\n",
        "bananas are fun for everyone",
    ] {
        assert_eq!(demangle_line(text, false), *text);
        assert_eq!(demangle_line(text, true), *text);
    }
}

#[test]
fn standalone_demangles() {
    for name in MANGLED_NAMES {
        assert_eq!(
            demangle_line(name, true).as_ref(),
            &demangle(name).to_string()
        );
    }
}

#[test]
fn not_noop_demangles() {
    for name in MANGLED_NAMES {
        assert_ne!(demangle_line(name, false).as_ref(), *name);
    }
}

#[test]
fn standalone_demangles_nohash() {
    for name in MANGLED_NAMES {
        assert_eq!(
            demangle_line(name, false).as_ref(),
            &format!("{:#}", demangle(name))
        );
    }
}

fn test_embedded_line_demangle<F1, F2>(line_demangler: F1, demangler: F2)
where
    F1: Fn(&str) -> String,
    F2: Fn(&str) -> String,
{
    for name in MANGLED_NAMES {
        macro_rules! test_context {
            ($context:expr) => {
                assert_eq!(
                    line_demangler(&format!($context, name)),
                    format!($context, demangler(name))
                )
            };
        }
        // x86 ASM
        test_context!("        lea     rax, [rip + {}]");
        test_context!("        call    {}@PLT");
        // perf script --no-demangle
        test_context!(
            "                  1a680e {} (/home/user/git/steven-rust/target/debug/steven)"
        );
        test_context!(
            "                  20039f {} (/home/user/git/steven-rust/target/debug/steven)"
        );
        test_context!(
            "                  1dade8 {} (/home/user/git/steven-rust/target/debug/steven)"
        );
        // Random unicode symbols
        test_context!("J∆ƒƒ∆Ǥ{}∆ʓ∆ɲI∆ɳ");
        // https://xkcd.com/1638/
        test_context!(r#"cat out.txt | grep -o "\\\[[(].*\\\[{}\])][^)\]]*$""#);
        test_context!(r"\\\\\\\\{}\\\\\\\\"); // Backslash to end all other text (twice)
    }
}

#[test]
fn embedded_demangle() {
    test_embedded_line_demangle(
        |line| demangle_line(line, false).into_owned(),
        |name| format!("{:#}", demangle(name)),
    );
}

#[test]
fn embedded_demangle_nohash() {
    test_embedded_line_demangle(
        |line| demangle_line(line, true).into_owned(),
        |name| demangle(name).to_string(),
    );
}

#[test]
fn stream_without_newlines() {
    let tab_mangled = MANGLED_NAMES.join("\t");

    let mut stream_demangled: Vec<u8> = vec![];
    demangle_stream(&mut tab_mangled.as_bytes(), &mut stream_demangled, true).unwrap();

    let split_demangled: Vec<_> = stream_demangled.split(|&b| b == b'\t').collect();
    assert_eq!(MANGLED_NAMES.len(), split_demangled.len());

    for (mangled, demangled) in MANGLED_NAMES.iter().zip(split_demangled) {
        assert_eq!(demangled, demangle(mangled).to_string().as_bytes());
    }
}
