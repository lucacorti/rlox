mod scanner;

use std::{env, fs, io::{self, BufRead, Write}, process::exit};
use scanner::Scanner;

enum SysExits {
    Ok = 0,
    Usage = 64,
    IOErr = 74
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _n => {
            println!("Usage: rlox [script]");
            exit(SysExits::Usage as i32);
        }
    }
}

fn run_file(file: &String) {
    let data = fs::read_to_string(file).unwrap();
    run(&data);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        match handle.read_line(&mut buffer) {
            Ok(0) => exit(SysExits::Ok as i32),
            Ok(_n) => {
                run(&buffer);
                buffer.clear();
            },
            Err(error) => {
                println!("{error}");
                exit(SysExits::IOErr as i32)
            }
        }
    }
}

fn run(data: &String) {
    let mut scanner = Scanner::new(data);
    scanner.scan_tokens();
}
