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
        self.skip_whitespaces();
        let tok = match self.ch {
            // this matches negative numbers or command arguments
            // this matches commands
            Some('\\') => {
                // consume the backslash
                self.read_char();
                let word = self.read_word();
                self.match_command(word)
            }
            // this matches general identifiers or keywords such as units
            Some(ch) => {
                if ch.is_digit(10) {
                    return self.read_number();
                } else if ch.is_alphabetic() {
                    let word = self.read_word();
                    return self.match_unit_or_operator(word);
                }
                Token::Illegal
            }
            None => Token::EOL,
        };
        self.read_char();
        tok
    }

    // Check if we have reached the end of the input
    pub fn is_end(&self) -> bool {
        match self.ch {
            Some(_) => false,
            None => true,
        }
    }

    // Reads a word which can be a keyword for commands, or units.
    fn read_word(&mut self) -> String {
        let pos = self.pos;
        if self.ch.eq(&Some('-')) {
            // consume - sign
            self.read_char();
        }
        while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
            self.read_char();
        }
        self.input[pos..self.pos].to_string()
    }

    /// Matches the read word from self.read_word to a keyword or an arbitrary identifier.
    /// # Example
    /// ````
    /// let word = self.read_word();
    /// let token = self.match_unit_or_operator(word);
    /// ````
    fn match_unit_or_operator(&mut self, word: String) -> Token {
        match word.to_lowercase().as_str() {
            "mm" => Token::Millimeter,
            "cm" => Token::Centimeter,
            "m" => Token::Meter,
            "km" => Token::Kilometer,
            "c" => Token::Celcius,
            "f" => Token::Fahrenheit,
            "to" => Token::To,
            "in" => Token::In,
            "as" => Token::As,
            _ => Token::Illegal,
        }
    }

    /// Matches the read word from self.read_word to a command or an illegal token.
    /// # Example
    /// ````
    /// let word = self.read_word();
    /// let token = self.match_command(word);
    /// ````
    fn match_command(&self, word: String) -> Token {
        match word.to_lowercase().as_str() {
            "cp" => Token::CopyCmd,
            "help" => Token::HelpCmd,
            "h" => Token::HelpCmd, // shorthand
            _ => Token::Illegal,
        }
    }

    /// Reads a number which can be an integer or a float.
    fn read_number(&mut self) -> Token {
        let pos = self.pos;
        if self.ch.eq(&Some('-')) {
            // consume - sign
            self.read_char();
        }
        // read part of the number as integer
        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
            self.read_char();
        }
        // potential double
        let lead_ch = self.peek_char();
        if self.ch.is_some()
            && self.ch.eq(&Some('.'))
            && lead_ch.is_some()
            && lead_ch.unwrap().is_digit(10)
        {
            // consume the period
            self.read_char();
            while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
                self.read_char();
            }
        }

        Token::Number(self.input[pos..self.pos].parse().unwrap())
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

    /// Peeks at the character where the self.lead_pos is pointing to.
    fn peek_char(&self) -> Option<char> {
        if self.lead_pos >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.lead_pos)
        }
    }

    /// Skips whitespaces. Yes, just that.
    fn skip_whitespaces(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}
