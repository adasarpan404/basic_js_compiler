#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i64),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    If,
    Else,
    EOF,
}

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let mut chars = text.chars().collect::<Vec<_>>();
        chars.push('\0');
        Lexer {
            text: chars.clone(),
            pos: 0,
            current_char: Some(chars[0]),
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char = if self.pos < self.text.len() {
            Some(self.text[self.pos])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn integer(&mut self) -> i64 {
        let mut result = String::new();
        while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        result.parse::<i64>().unwrap()
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if c.is_digit(10) {
                return Token::Integer(self.integer());
            }

            match c {
                '+' => {
                    self.advance();
                    return Token::Plus;
                }
                '-' => {
                    self.advance();
                    return Token::Minus;
                }
                '*' => {
                    self.advance();
                    return Token::Mul;
                }
                '/' => {
                    self.advance();
                    return Token::Div;
                }
                '(' => {
                    self.advance();
                    return Token::LParen;
                }
                ')' => {
                    self.advance();
                    return Token::RParen;
                }
                '{' => {
                    self.advance();
                    return Token::LBrace;
                }
                '}' => {
                    self.advance();
                    return Token::RBrace;
                }
                ';' => {
                    self.advance();
                    return Token::Semicolon;
                }
                'i' if self.text[self.pos] == 'i'
                    && self.pos + 1 < self.text.len()
                    && self.text[self.pos + 1] == 'f' =>
                {
                    self.advance();
                    self.advance();
                    return Token::If;
                }
                'e' if self.text[self.pos] == 'e'
                    && self.pos + 3 < self.text.len()
                    && self.text[self.pos + 1] == 'l'
                    && self.text[self.pos + 2] == 's'
                    && self.text[self.pos + 3] == 'e' =>
                {
                    self.advance();
                    self.advance();
                    self.advance();
                    self.advance();
                    return Token::Else;
                }
                '\0' => return Token::EOF,
                _ => panic!("Unknown character: {}", c),
            }
        }
        Token::EOF
    }
}
