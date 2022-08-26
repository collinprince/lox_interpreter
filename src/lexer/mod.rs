pub mod cursor;
use cursor::Cursor;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Literal {
    Str { val: String, terminated: bool },
    Num { val: f64 },
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u32) -> Token {
        Token {
            kind,
            lexeme,
            line,
            literal: None,
        }
    }

    pub fn literal(self, literal: Literal) -> Token {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            line: self.line,
            literal: Some(literal),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.kind,
            self.lexeme,
            match &self.literal {
                Some(x) => {
                    match x {
                        Literal::Str {
                            val: s,
                            terminated: t,
                        } => format!("{} {}", if *t { "terminated" } else { "unterminated" }, s),
                        Literal::Num { val: n } => n.to_string(),
                    }
                }
                None => {
                    "".to_string()
                }
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // semantically unimportant lexemes that will be filtered out
    Comment,
    BlockComment,
    Whitespace,
    Newline,

    // unknown token, we will report this error later
    Unknown,

    // eof
    EOF,
}

// don't bother implementing print for every enum value,
// just pipe debug output to display
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn scan_tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.scan_token())
        }
    })
}

impl Cursor<'_> {
    fn advance_if_next(&mut self, c: char) -> bool {
        if self.is_eof() {
            false
        } else {
            if c == self.first() {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn scan_token(&mut self) -> Token {
        use TokenKind::*;
        let first_char = self.advance().unwrap();
        match first_char {
            // single char lexemes
            '(' => Token::new(LeftParen, "(".to_string(), self.line),
            ')' => Token::new(RightParen, ")".to_string(), self.line),
            '{' => Token::new(LeftBrace, "{".to_string(), self.line),
            '}' => Token::new(RightBrace, "}".to_string(), self.line),
            ',' => Token::new(Comma, ",".to_string(), self.line),
            '.' => Token::new(Dot, ".".to_string(), self.line),
            '-' => Token::new(Minus, "-".to_string(), self.line),
            '+' => Token::new(Plus, "+".to_string(), self.line),
            '*' => Token::new(Star, "*".to_string(), self.line),
            ';' => Token::new(Semicolon, ";".to_string(), self.line),

            // optionally two char lexemes
            '!' => {
                if self.advance_if_next('=') {
                    Token::new(BangEqual, "!=".to_string(), self.line)
                } else {
                    Token::new(Bang, "!".to_string(), self.line)
                }
            }
            '=' => {
                if self.advance_if_next('=') {
                    Token::new(EqualEqual, "==".to_string(), self.line)
                } else {
                    Token::new(Equal, "=".to_string(), self.line)
                }
            }
            '<' => {
                if self.advance_if_next('=') {
                    Token::new(LessEqual, "<=".to_string(), self.line)
                } else {
                    Token::new(Less, "<".to_string(), self.line)
                }
            }
            '>' => {
                if self.advance_if_next('=') {
                    Token::new(GreaterEqual, ">=".to_string(), self.line)
                } else {
                    Token::new(Greater, ">".to_string(), self.line)
                }
            }

            // potential multi char comment
            '/' => {
                if self.advance_if_next('/') {
                    self.eat_while(|c| c != '\n');
                    Token::new(Comment, "".to_string(), self.line)
                } else if self.advance_if_next('*') {
                    let literal = std::string::String::from("/*");
                    self.block_comment(literal)
                } else {
                    Token::new(Slash, "/".to_string(), self.line)
                }
            }
            ' ' | '\r' | '\t' => Token::new(Whitespace, "".to_string(), self.line),
            '\n' => {
                // technically newline is on self.line, not self.line + 1
                // but this token will be filtered anyway, so it's ok
                self.line += 1;
                Token::new(Newline, "".to_string(), self.line)
            }
            '"' => self.string(),
            d if is_digit(d) => self.number(std::string::String::from(d)),
            a if is_alpha(a) => self.identifer_or_keyword(a.to_string()),
            x => Token::new(Unknown, x.to_string(), self.line),
        }
    }

    pub fn string(&mut self) -> Token {
        let mut literal: String = String::from("");
        let mut line = self.line;
        self.eat_while(|c| {
            if c != '"' {
                if c == '\n' {
                    line += 1;
                }
                literal.push(c);
                true
            } else {
                false
            }
        });
        if self.is_eof() {
            Token::new(TokenKind::String, format!("\"{}", literal), self.line).literal(
                Literal::Str {
                    val: literal,
                    terminated: false,
                },
            )
        } else {
            // advance to new line count in case this was a multiline string
            self.line = line;
            // advance past closing quote
            self.advance();
            Token::new(
                TokenKind::String,
                format!("\"{}\"", literal).to_string(),
                self.line,
            )
            .literal(Literal::Str {
                val: literal,
                terminated: true,
            })
        }
    }

    pub fn number(&mut self, mut literal: String) -> Token {
        self.eat_while(|c| {
            if is_digit(c) {
                literal.push(c);
                true
            } else {
                false
            }
        });

        if self.advance_if_next('.') && is_digit(self.second()) {
            // consume fractional part as well
            literal.push('.');
            self.eat_while(|c| {
                if is_digit(c) {
                    literal.push(c);
                    true
                } else {
                    false
                }
            })
        }

        Token::new(TokenKind::Number, literal.clone(), self.line).literal(Literal::Num {
            val: literal.parse::<f64>().unwrap(),
        })
    }

    pub fn identifer_or_keyword(&mut self, mut literal: String) -> Token {
        self.eat_while(|c| {
            if is_alpha_numeric(c) {
                literal.push(c);
                true
            } else {
                false
            }
        });
        Token::new(get_text_type(&literal), literal, self.line)
    }

    pub fn block_comment(&mut self, mut literal: String) -> Token {
        let start_line = self.line;
        let mut finish_line = start_line;
        // make prev_c space to begin instead of '*'
        // to avoid incorrectly lexing /*/ as a block comment
        let mut prev_c = ' ';
        self.eat_while(|c| {
            literal.push(c);
            if prev_c == '*' && c == '/' {
                false
            } else if c == '\n' {
                prev_c = c;
                finish_line += 1;
                true
            } else {
                prev_c = c;
                true
            }
        });
        // unterminated block comment
        if self.is_eof() {
            return Token::new(TokenKind::BlockComment, literal.clone(), start_line).literal(
                Literal::Str {
                    val: literal,
                    terminated: false,
                },
            );
        }
        // advance past / to end block comment
        self.advance();
        self.line = finish_line;
        Token::new(TokenKind::BlockComment, literal.clone(), start_line).literal(Literal::Str {
            val: literal,
            terminated: true,
        })
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

thread_local! { static KEYWORD_MAP: HashMap<&'static str, TokenKind> =
    HashMap::from([
        ("and", TokenKind::And),
        ("class", TokenKind::Class),
        ("else", TokenKind::Else),
        ("false", TokenKind::False),
        ("for", TokenKind::For),
        ("fun", TokenKind::Fun),
        ("if", TokenKind::If),
        ("nil", TokenKind::Nil),
        ("or", TokenKind::Or),
        ("print", TokenKind::Print),
        ("return", TokenKind::Return),
        ("super", TokenKind::Super),
        ("this", TokenKind::This),
        ("true", TokenKind::True),
        ("var", TokenKind::Var),
        ("while", TokenKind::While),
    ]);
}
// not sure that there is a way to have a static
// hashmap in rust using std, so instead will
// implement a thread safe global variable
fn get_text_type(s: &String) -> TokenKind {
    // implement a singleton
    KEYWORD_MAP.with(|map_cell| match map_cell.get(s.as_str()) {
        Some(&token_kind) => token_kind,
        None => TokenKind::Identifier,
    })
}

/*
after this sequence of tokens has been created, need to
pass through and check for error tokens
if there are error tokens present, we will report them
and stop program, else we'll continue
*/
