#[derive(Debug, Clone)]
pub enum Literal {
    Str { val: String, terminated: bool },
    Num { val: f64 },
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
