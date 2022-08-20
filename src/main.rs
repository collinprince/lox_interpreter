use std::env;
use std::fs;
use std::io::stdin;
use std::io::Write;

pub mod error_handling;
pub mod lexer;

use crate::error_handling::{CLArgsError, Error};
use crate::lexer::cursor::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox_interpreter [script]");
        return Err(Box::new(CLArgsError::new(
            0,
            "Incorrect commandline args".to_string(),
        )));
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(s: &String) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(s).expect("Was not able to read in file {s}");
    let cursor = Cursor::new(contents.as_str());
    match run(cursor) {
        Err(x) => {
            println!("{}", x);
            Ok(()) // this should probably be an error, fix later
        }
        _ => Ok(()),
    }
}

fn run_prompt() -> Result<(), std::io::Error> {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("flush failed!");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        match buf.trim_end() {
            "" => break,
            input => {
                let cursor = Cursor::new(input);
                if let Err(x) = run(cursor) {
                    // report error if one occurred
                    println!("{}", x);
                }
            }
        }
    }
    Ok(())
}

fn run(_cursor: Cursor) -> Result<(), Box<dyn Error>> {
    println!("cursor");
    Err(Box::new(CLArgsError::new(1, "Test".to_string())))
    // Ok(())
}
