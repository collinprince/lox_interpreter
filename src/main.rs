use std::env;
use std::fs;
use std::io::stdin;
use std::io::Write;

pub mod error_handling;
pub mod lexer;

use crate::error_handling::{CLArgsError, Error, IOError, LexError};
use crate::lexer::cursor::Cursor;
use crate::lexer::scan_tokens;

fn main() -> Result<(), Box<dyn Error>> {
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

fn run_file(s: &String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(s).expect("Was not able to read in file {s}");
    // let cursor = Cursor::new(contents.as_str());
    run(contents.as_str())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut line = 0;
    loop {
        line += 1;
        print!("> ");
        std::io::stdout().flush().expect("flush failed!");
        let mut buf = String::new();
        if let Err(x) = stdin().read_line(&mut buf) {
            return Err(Box::new(IOError::new(line, x)));
        }
        match buf.trim_end() {
            "" => break,
            input => {
                if let Err(x) = run(input) {
                    // report error if one occurred
                    println!("{}", x);
                }
            }
        }
    }
    Ok(())
}

fn run(input: &str) -> Result<(), Box<dyn Error>> {
    use crate::lexer::Literal;
    let mut errors = vec![];
    for x in scan_tokens(input) {
        match x.kind {
            lexer::TokenKind::Unknown => {
                errors.push(LexError::new("Unknown token".to_string(), x.line));
            }
            lexer::TokenKind::String => match x.literal.clone() {
                Some(l) => match l {
                    Literal::Str { terminated: t, .. } => {
                        if !t {
                            errors.push(LexError::new("Unterminated string".to_string(), x.line));
                        } else {
                            println!("{}", x);
                        }
                    }
                    _ => {
                        println!("{}", x);
                    }
                },
                _ => {
                    println!("{}", x);
                }
            },
            _ => {
                println!("{}", x);
            }
        }
    }
    if errors.len() > 0 {
        for e in errors {
            println!("{}", e);
        }
    }
    Ok(())
}
