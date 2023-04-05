use std::{str::Chars, process, iter};

use phf::phf_map;


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Float(f64),
    Identifier(String),
    String(String),
    Plus,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Eq,
    Semi,
    Let,
    Hatch,
    Build,
    Push,
    Top,

    Axe,
    Chicken,
    Add,
    Fox,
    Rooster,
    Cmp,
    Pick,
    Peck,
    Fr,
    Bbq,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,
    "build" => Token::Build,
    "hatch" => Token::Hatch,
    "push" => Token::Push,
    "TOP" => Token::Top,
    
    "axe" => Token::Axe,
    "chicken" => Token::Chicken,
    "add" => Token::Add,
    "fox" => Token::Fox,
    "rooster" => Token::Rooster,
    "compare" => Token::Cmp,
    "pick" => Token::Pick,
    "peck" => Token::Peck,
    "fr" => Token::Fr,
    "bbq" => Token::Bbq
};

pub struct Lexer<'a> {
    input: &'a String,
    chars: Chars<'a>,
    pub cur_char: Option<char>,
    pub line: usize,
    pub col: usize,
    pub pos: usize,
    pub lookahead: Option<Token>,
}

pub struct LexerIterator<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl Iterator for LexerIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.lex_token()
    }
}

impl<'a> IntoIterator for &'a mut Lexer<'a> {
    type Item = Token;

    type IntoIter = LexerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIterator { lexer: self }
    }
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.chars(),
            cur_char: None,
            line: 1,
            col: 0,
            pos: 0,
            lookahead: None,
        };
        lexer.cur_char = lexer.chars.next();
        lexer.lookahead = lexer.lex_token();
        lexer
    }

    pub fn match_token(&mut self, expected: Token) -> bool {
        if self.lookahead == Some(expected.clone()) {
            self.lookahead = self.lex_token();
            true
        } else {
            println!(
                "Expected {:?} on line {} column {}, got {}",
                expected,
                self.line,
                self.col,
                match &self.lookahead {
                    Some(t) => format!("{:?}", t),
                    None => "None".to_owned()
                }
            );
            self.point_error()
        }
    }

    pub fn step_token(&mut self) {
        self.lookahead = self.lex_token();
    }

    pub fn point_error(&self) -> ! {
        let line_end = if let Some(n) = self.input[self.pos..].find('\n') {
            self.pos + n - 1
        } else {
            self.input.len() - 1
        };

        println!("\n{}", &self.input[self.pos - self.col + 1..=line_end]);
        println!("{}^", iter::repeat(' ').take(self.col - 2).collect::<String>());
        process::exit(1)
    }

    fn step_chr(&mut self) {
        self.pos += 1;
        self.col += 1;
        self.cur_char = self.chars.next();
        if matches!(self.cur_char, Some(ch) if ch == '\n') {
            self.line += 1;
            self.col = 0;
        }
    }

    fn lex_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.cur_char {
            Some('+') => {
                self.step_chr();
                Some(Token::Plus)
            }
            Some('-') => {
                self.step_chr();
                Some(Token::Sub)
            }
            Some('*') => {
                self.step_chr();
                Some(Token::Mul)
            }
            Some('/') => {
                self.step_chr();
                Some(Token::Div)
            }
            Some('(') => {
                self.step_chr();
                Some(Token::LParen)
            }
            Some(')') => {
                self.step_chr();
                Some(Token::RParen)
            }
            Some('[') => {
                self.step_chr();
                Some(Token::LBracket)
            }
            Some(']') => {
                self.step_chr();
                Some(Token::RBracket)
            }
            Some('{') => {
                self.step_chr();
                Some(Token::LBrace)
            }
            Some('}') => {
                self.step_chr();
                Some(Token::RBrace)
            }
            Some(',') => {
                self.step_chr();
                Some(Token::Comma)
            }
            Some('=') => {
                self.step_chr();
                Some(Token::Eq)
            }
            Some(';') => {
                self.step_chr();
                Some(Token::Semi)
            }
            Some('"') => Some(self.lex_string()),
            Some(ch) if ch.is_digit(10) => Some(self.lex_number()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => Some(self.lex_ident()),
            Some(ch) => {
                println!("Invalid character '{}' on line {} column {}", ch, self.line, self.col);
                self.point_error();
            }
            None => None,
        }
    }

    fn lex_string(&mut self) -> Token {
        Token::String("".to_owned());
        let mut ret = String::new();
        self.consume_char('"');

        loop {
            let start = self.pos;
            self.consume_while(|c| c != '\\' && c != '"');

            ret.push_str(&self.input[start..self.pos]);
            if self.consume_char('\\') {
                match self.cur_char {
                    Some('n') => ret.push('\n'),
                    Some('t') => ret.push('\t'),
                    Some('"') => ret.push('"'),
                    Some(c) => {
                        println!("Invalid escape sequence '\\{}' on line {} column {}", c, self.line, self.col);
                        self.point_error();
                    },
                    None => {
                        println!("Unexpected end of input while parsing string on line {} column {}", self.line, self.col);
                        self.point_error();
                    },
                }
                self.step_chr();
            } else if self.consume_char('"') {
                break
            }
        }

        Token::String(ret)
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        self.consume_digits();
        
        if self.consume_char('.') {
            self.consume_digits();
            Token::Float(self.input[start..self.pos].parse().unwrap())
        } else {
            Token::Int(self.input[start..self.pos].parse().unwrap())
        }
    }

    fn lex_ident(&mut self) -> Token {
        let start = self.pos;

        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
        let ret = &self.input[start..self.pos];

        match KEYWORDS.get(ret) {
            Some(tok) => tok.clone(),
            None => Token::Identifier(ret.to_owned()),
        }
    }

    fn consume_while<F>(&mut self, pred: F)
    where F: Fn(char) -> bool
    {
        while matches!(self.cur_char, Some(ch) if pred(ch)) {
            self.step_chr();
        }
    }

    fn consume_char(&mut self, c: char) -> bool {
        if matches!(self.cur_char, Some(ch) if ch == c) {
            self.step_chr();
            true
        } else {
            false
        }
    }

    fn consume_digits(&mut self) {
        self.consume_while(|c| c.is_digit(10))
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace())
    }
}

