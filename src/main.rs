#[derive(Debug, PartialEq, Clone)]
enum Token {
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

struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(text: String) -> Self {
        let mut chars = text.chars().collect::<Vec<_>>();
        chars.push('\0');
        Lexer {
            text: chars.clone(),
            pos: 0,
            current_char: Some(chars[0]),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = if self.pos < self.text.len() {
            Some(self.text[self.pos])
        } else {
            None
        };
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

    fn get_next_token(&mut self) -> Token {
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

#[derive(Debug, Clone)]
enum AST {
    BinOp(Box<AST>, Token, Box<AST>),
    Num(i64),
    IfElse(Box<AST>, Box<AST>, Box<AST>),
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token_type: Token) {
        if self.current_token == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn factor(&mut self) -> AST {
        match &self.current_token {
            Token::Integer(value) => {
                let node = AST::Num(*value);
                self.eat(Token::Integer(*value));
                node
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let node = self.expr();
                self.eat(Token::RParen);
                node
            }
            _ => panic!("Invalid factor"),
        }
    }

    fn term(&mut self) -> AST {
        let mut node = self.factor();
        while self.current_token == Token::Mul || self.current_token == Token::Div {
            let token = self.current_token.clone();
            if token == Token::Mul {
                self.eat(Token::Mul);
            } else {
                self.eat(Token::Div);
            }
            node = AST::BinOp(Box::new(node), token, Box::new(self.factor()));
        }
        node
    }

    fn expr(&mut self) -> AST {
        let mut node = self.term();
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let token = self.current_token.clone();
            if token == Token::Plus {
                self.eat(Token::Plus);
            } else {
                self.eat(Token::Minus);
            }
            node = AST::BinOp(Box::new(node), token, Box::new(self.term()));
        }
        node
    }

    fn block(&mut self) -> AST {
        self.eat(Token::LBrace); // eat '{'
        let expr = self.expr(); // parse expression
        self.eat(Token::Semicolon); // expect ';'
        self.eat(Token::RBrace); // eat '}'
        expr
    }

    fn if_else_statement(&mut self) -> AST {
        self.eat(Token::If);
        self.eat(Token::LParen);
        let condition = self.expr();
        self.eat(Token::RParen);

        let true_branch = self.block();
        let false_branch = if self.current_token == Token::Else {
            self.eat(Token::Else);
            self.block()
        } else {
            AST::Num(0)
        };

        AST::IfElse(
            Box::new(condition),
            Box::new(true_branch),
            Box::new(false_branch),
        )
    }

    fn parse(&mut self) -> AST {
        if self.current_token == Token::If {
            self.if_else_statement()
        } else {
            self.expr()
        }
    }
}

struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    fn new(parser: Parser) -> Self {
        Interpreter { parser }
    }

    fn visit(&self, node: AST) -> i64 {
        match node {
            AST::BinOp(left, op, right) => {
                let left_val = self.visit(*left);
                let right_val = self.visit(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Mul => left_val * right_val,
                    Token::Div => left_val / right_val,
                    _ => panic!("Invalid operator"),
                }
            }
            AST::Num(value) => value,
            AST::IfElse(condition, true_branch, false_branch) => {
                let condition_val = self.visit(*condition);
                if condition_val != 0 {
                    self.visit(*true_branch)
                } else {
                    self.visit(*false_branch)
                }
            }
        }
    }

    fn interpret(&mut self) -> i64 {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}

fn main() {
    use std::env;
    use std::fs;
    use std::process;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lexer = Lexer::new(contents);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new(parser);

    let result = interpreter.interpret();
    println!("Result: {}", result);
}
