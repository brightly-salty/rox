#![feature(result_cloned, const_option)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate anyhow;

mod ast;
mod environment;
mod interpreter;
mod parser;
mod scanner;
mod tokens;
mod value;

use anyhow::Result;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tokens::{Token, TokenType};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // Consume `rox`
    let interpreter = Interpreter::new();
    if let Some(filename) = args.next() {
        run_file(filename, interpreter)?;
    } else {
        run_prompt(&interpreter)?;
    }
    Ok(())
}

fn run_file<P: AsRef<Path>>(filename: P, interpreter: Interpreter) -> Result<()> {
    let contents = fs::read_to_string(filename)?;
    run(&contents, interpreter);
    if had_error() {
        panic!("There was an error running the file!")
    }
    Ok(())
}

fn run_prompt(interpreter: &Interpreter) -> Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        run(&input, interpreter.clone());
        set_had_error(false);
        input.clear();
    }
}

fn run(source: &str, mut interpreter: Interpreter) {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    if had_error() {
        return;
    }
    interpreter.interpret(&statements);
}

pub fn error(line: NonZeroUsize, message: &str) {
    report(line, "", message);
}

pub fn error_at_token(token: &Token, message: &str) {
    if token.type_ == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(
            token.line,
            &format!(" at '{}'", token.lexeme),
            message,
        );
    }
}

fn report(line: NonZeroUsize, where_: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, where_, message);
    set_had_error(true);
}

fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

fn set_had_error(b: bool) {
    HAD_ERROR.store(b, Ordering::Relaxed)
}
