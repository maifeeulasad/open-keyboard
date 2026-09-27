//! `okb` — a small command-line harness for the open-keyboard engine.
//!
//! It exists so the linguistic core can be exercised and demonstrated long before
//! the IBus/Fcitx5 system integration exists.
//!
//! ## Usage
//!
//! ```text
//! okb amar sonar bangla      # transliterate the arguments
//! echo "ami tomay valobasi" | okb   # or read lines from stdin
//! okb --help
//! ```

use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use okb_engine::{PhoneticScheme, Scheme};

const HELP: &str = "\
okb — open-keyboard transliteration harness

USAGE:
    okb [TEXT]...        Transliterate the given text (English → Bengali)
    okb                  Read lines from stdin and transliterate each

OPTIONS:
    -h, --help           Show this help
    -V, --version        Show version

EXAMPLES:
    okb amar sonar bangla
    echo \"ami bangla likhi\" | okb
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{HELP}");
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("okb {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    let scheme = PhoneticScheme::bengali();

    if args.is_empty() {
        run_stdin(&scheme)
    } else {
        println!("{}", scheme.transliterate(&args.join(" ")));
        ExitCode::SUCCESS
    }
}

/// Transliterate stdin line by line, so the tool composes in pipelines.
fn run_stdin(scheme: &PhoneticScheme) -> ExitCode {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            eprintln!("okb: failed to read input");
            return ExitCode::FAILURE;
        };
        if writeln!(out, "{}", scheme.transliterate(&line)).is_err() {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
