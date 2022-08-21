pub mod cursor;

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
    pub literal: Option<String>,
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

    pub fn literal(self, literal: Option<String>) -> Token {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            line: self.line,
            literal,
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
            self.literal.clone().unwrap_or("".to_string())
        )
    }
}

#[derive(Debug)]
pub enum TokenKind {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    PLus,
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

    EOF,
}

// don't bother implementing print for every enum value,
// just pipe debug output to display
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
