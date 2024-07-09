// NOTE: use brings the module into scope. `crate` searches from the crate root which is
// src/main.rs in this case.
use crate::token::Token;
use std::string::String;

/// Lexer is used to analyse an input and turns them into tokens that can be used to evaluate the
/// input further.
pub struct Lexer {
    /// the entire input string
    input: String,
    /// current position in input
    pos: usize,
    /// leading position in input
    lead_pos: usize,
    /// current char under examination
    ch: Option<char>,
}

impl Lexer {
    /// Create a new Lexer.
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            pos: 0,
            lead_pos: 0,
            ch: None,
        };

        // init by reading the first character
        l.read_char();

        // implicit return makes my brain shortcircuit still so i am going to stick to explicit
        // return for now.
        return l;
    }

    /// Grab the next token. It will return Token::EOL if no more token to read.
    /// # Exmaple
    /// ````
    /// let lexer = Lexer::new(input);
    /// let token = lexer.next_token();
    /// ````
    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some(ch) => {
                if ch.is_alphabetic() {
                    return self.read_word();
                }
                Token::Illegal
            }
            None => Token::EOL,
        };
        self.read_char();
        tok
    }

    // Reads a word which can be a keyword for commands, or units.
    fn read_word(&mut self) -> Token {
        let pos = self.pos;
        while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
            self.read_char();
        }
        let identifier = &self.input[pos..self.pos];
        Token::Identifier(identifier.to_string())
    }

    /// Moves the input position one step.
    fn read_char(&mut self) {
        if self.lead_pos >= self.input.len() {
            self.ch = None
        } else {
            self.ch = Some(self.input.chars().nth(self.lead_pos).unwrap());
        }
        self.pos = self.lead_pos;
        self.lead_pos += 1;
    }
}
