use std::collections::HashMap;
use std::str::Chars;

// using implementation of cursor based on cursor from rust's lexer
// (rust/compiler/rustc_lexer/src/cursor.rs)

pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
    #[cfg(debug_assertions)]
    prev: char,
    pub line: u32,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
            prev: EOF_CHAR,
            line: 1,
        }
    }

    // return last eaten symbol
    pub fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }
        #[cfg(not(debug_assertions))]
        {
            EOF_CHAR
        }
    }

    // get first char in chars without consuming
    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    // get second char in chars without consuming
    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    // return if we have reached end of file
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    // return numbers of bytes consumed so far
    pub fn len_consumed(&self) -> u32 {
        (self.initial_len - self.chars.as_str().len()) as u32
    }

    // reset the number of bytes consumed to 0
    pub fn reset_len_consumed(&mut self) {
        self.initial_len = self.chars.as_str().len()
    }

    // moves to next char acter
    pub fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }

        Some(c)
    }

    // eats symbols while predicate is true or until eof is reached
    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.advance();
        }
    }
}
