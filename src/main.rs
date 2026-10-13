//! Minimal implementation of rustc-demangle as a binary.
//!
//! This is not intended to be a feature-rich binary, e.g., argument parsing is unlikely to be
//! added. However it's usable for most use cases and as of writing is ~2x faster than rustfilt.

use std::io;
use std::process::ExitCode;

#[cfg(not(feature = "std"))]
compile_error!("Currently the std feature is required for building the binary");

fn main() -> ExitCode {
    if std::env::args_os().count() > 1 {
        eprintln!("No arguments expected. Pass input on stdin.");
        return ExitCode::FAILURE;
    }

    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();

    let Err(e) = rustc_demangle::demangle_stream(&mut input, &mut output, false) else {
        return ExitCode::SUCCESS;
    };

    // Don't print any extra output if we hit BrokenPipe, just exit.
    if e.kind() == std::io::ErrorKind::BrokenPipe {
        // FIXME: This isn't quite the same as a SIGPIPE failure, but it's probably close enough for
        // our purposes.
        return ExitCode::FAILURE;
    }

    eprintln!("Failed: {}", e);
    ExitCode::FAILURE
}
