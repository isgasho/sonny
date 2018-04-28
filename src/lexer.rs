use std::fs::File;
use std::io::Read;

static KEYWORDS: &[&'static str] = &[
    "time",
    "sample_rate",
    "sin",
    "cos",
    "ceil",
    "floor",
    "abs",
    "end",
    "out",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Operator,
    Id,
    Num,
    NoteString,
    Keyword,
    Delimeter,
    Misc,
    Done,
    Unknown,
    Empty,
}

use self::TokenType::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(pub TokenType, pub String);

macro_rules! tok {
    ($t:expr, $s:expr) => {
        Token($t, ($s).to_string())
    };
}

#[derive(Debug)]
pub struct Lexer {
    lineno: usize,
    was_put_back: bool,
    c: [u8; 1],
    file: File,
}

impl Lexer {
    pub fn new(file: &str) -> Lexer {
        Lexer {
            lineno: 1,
            was_put_back: false,
            c: [0],
            file: File::open(file).expect(&format!("Unable to open file \"{}\"", file)),
        }
    }
    pub fn lineno(&self) -> usize {
        self.lineno
    }
    fn get_char(&mut self) -> Option<char> {
        if self.was_put_back {
            self.was_put_back = false;
            Some(self.c[0] as char)
        } else {
            if self.file.read_exact(&mut self.c).is_ok() {
                Some(self.c[0] as char)
            } else {
                None
            }
        }
    }
    fn put_back(&mut self) {
        self.was_put_back = true;
    }
    pub fn lex(&mut self) -> Token {
        // Begin reading a token
        while let Some(c) = self.get_char() {
            let mut token = String::new();

            // Check for tokens that start with alpha or us
            if c.is_alphabetic() || c == '_' {
                token.push(c);
                // Check for ids, keywords, built_ins and notes
                while let Some(c) = self.get_char() {
                    if c.is_alphanumeric() || c == '_' {
                        token.push(c);
                    } else {
                        self.put_back();
                        // Check for pi
                        if token == "pi" {
                            return Token(Num, token);
                        }
                        // Check for keywords
                        if KEYWORDS.iter().find(|&k| k == &token).is_some() {
                            return Token(Keyword, token);
                        }
                        // Check for notes
                        let bytes: Vec<u8> = token.chars().map(|cc| cc as u8).collect();
                        let mut i = 0;
                        if bytes[i] >= 'A' as u8 && bytes[i] <= 'F' as u8 {
                            if bytes.len() == 1 {
                                return Token(NoteString, token);
                            } else {
                                i += 1;
                                if bytes[i] == 'b' as u8 || bytes[i] == '#' as u8 {
                                    if bytes.len() == 2 {
                                        return Token(NoteString, token);
                                    }
                                    i += 1;
                                }
                                while i < bytes.len() {
                                    if !(bytes[i] as char).is_digit(10) {
                                        return Token(Id, token);
                                    }
                                    i += 1;
                                }
                                return Token(NoteString, token);
                            }
                        }

                        return Token(Id, token);
                    }
                }
            }
            // Check for valid num tokens
            else if c.is_digit(10) {
                token.push(c);
                while let Some(c) = self.get_char() {
                    if c.is_digit(10) {
                        token.push(c);
                    } else {
                        self.put_back();
                        return Token(Num, token);
                    }
                }
            }
            // Check for newlines
            else if c.is_whitespace() {
                if c == '\n' {
                    self.lineno += 1;
                }
            }
            // Check for operators
            else {
                token.push(c);
                match c {
                    '(' | ')' | '{' | '}' | '[' | ']' | ',' => return Token(Delimeter, token),
                    ':' => {
                        if let Some(c) = self.get_char() {
                            if c == ':' {
                                token.push(c);
                            } else {
                                self.put_back();
                            }
                            return Token(Delimeter, token);
                        }
                    }
                    '!' | '.' => return Token(Misc, token),
                    '+' | '*' | '%' | '^' => return Token(Operator, token),
                    '-' => {
                        if let Some(c) = self.get_char() {
                            if c == '>' {
                                token.push(c);
                                return Token(Delimeter, token);
                            } else {
                                self.put_back();
                                return Token(Operator, token);
                            }
                        }
                    }
                    '/' => {
                        if let Some(c) = self.get_char() {
                            match c {
                                '/' => {
                                    while let Some(c) = self.get_char() {
                                        if c == '\n' {
                                            break;
                                        }
                                    }
                                }
                                _ => {
                                    self.put_back();
                                    return Token(Operator, token);
                                }
                            }
                        }
                    }
                    _ => {
                        token.push(c);
                        return Token(Unknown, token);
                    }
                }
            }
        }
        Token(Done, String::new())
    }
}
