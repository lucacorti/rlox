mod scanner;

use scanner::Scanner;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::{env, fs};

pub enum SysExits {
    Ok = 0,
    Usage = 64,
    IOErr = 74,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => exit(run_repl() as i32),
        2 => exit(run_file(&args[1]) as i32),
        _n => {
            println!("Usage: rlox [script]");
            exit(SysExits::Usage as i32);
        }
    }
}

pub fn run(data: &String) -> SysExits {
    let mut scanner = Scanner::new(data);
    scanner.scan_tokens();
    return SysExits::Ok;
}

pub fn run_file(file: &String) -> SysExits {
    let data = fs::read_to_string(file).unwrap();
    return run(&data);
}

fn run_repl() -> SysExits {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        match handle.read_line(&mut buffer) {
            Ok(0) => return SysExits::Ok,
            Ok(_n) => {
                run(&buffer);
                buffer.clear();
            }
            Err(error) => {
                println!("{error}");
                return SysExits::IOErr;
            }
        }
    }
}
