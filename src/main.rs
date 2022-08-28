use std::env;
use std::fs;
use std::io::stdin;
use std::io::Write;

pub mod error_handling;
pub mod lexer;
pub mod syntax;

use crate::error_handling::{CLArgsError, Error, IOError, LexError};
use crate::lexer::scan_tokens;
use crate::lexer::token::{Token, TokenKind};
use crate::syntax::expr::*;

fn main() -> Result<(), Box<dyn Error>> {
    // let args: Vec<String> = env::args().collect();
    // if args.len() > 2 {
    //     println!("Usage: lox_interpreter [script]");
    //     return Err(Box::new(CLArgsError::new(
    //         0,
    //         "Incorrect commandline args".to_string(),
    //     )));
    // } else if args.len() == 2 {
    //     run_file(&args[1])?;
    // } else {
    //     run_prompt()?;
    // }

    let expr = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token::new(TokenKind::Minus, "-".to_string(), 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 123.0 },
            })),
        })),
        operator: Token::new(TokenKind::Star, "*".to_string(), 1),
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 45.67 },
            })),
        })),
    });

    let printer = AstPrinter {};
    println!("{}", printer.print(&expr));

    let expr_2 = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 1.0 },
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 2.0 },
            })),
        })),
        operator: Token::new(TokenKind::Star, "*".to_string(), 1),
        right: Box::new(Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 4.0 },
            })),
            operator: Token::new(TokenKind::Minus, "-".to_string(), 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Literal::Num { val: 3.0 },
            })),
        })),
    });

    let rp_printer = crate::syntax::expr::ReversePolishPrinter {};
    println!("{}", rp_printer.print(&expr_2));

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
                    print!("{}", x);
                }
            }
        }
    }
    Ok(())
}

fn run(input: &str) -> Result<(), Box<dyn Error>> {
    use crate::lexer::token::TokenKind;
    let mut errors = vec![];
    for x in scan_tokens(input) {
        match x.kind {
            TokenKind::Unknown => {
                errors.push(LexError::new("Unknown token".to_string(), x.line));
            }
            TokenKind::String => match x.literal.clone() {
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
            print!("{}", e);
        }
    }
    Ok(())
}
