mod scanner;
mod tokens;

use anyhow::Result;
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // Consume `rox`
    if let Some(filename) = args.next() {
        run_file(filename)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file<P: AsRef<Path>>(filename: P) -> Result<()> {
    let contents = fs::read_to_string(filename)?;
    run(&contents);
    if get_had_error() {
        panic!("There was an error running the file!")
    }
    Ok(())
}

fn run_prompt() -> Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        run(&input);
        set_had_error(false);
        input.clear();
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan_tokens();
    for token in &tokens {
        println!("{:?}", token)
    }
}

pub fn error(line: NonZeroUsize, message: &str) {
    report(line, String::new(), message.to_owned());
}

fn report(line: NonZeroUsize, where_: String, message: String) {
    println!("[line {}] Error{}: {}", line, where_, message);
    set_had_error(true);
}

fn get_had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

fn set_had_error(b: bool) {
    HAD_ERROR.store(b, Ordering::Relaxed)
}
