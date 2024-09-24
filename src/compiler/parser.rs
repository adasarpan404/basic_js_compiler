use super::token::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum AST {
    BinOp(Box<AST>, Token, Box<AST>),
    Num(i64),
    IfElse(Box<AST>, Box<AST>, Box<AST>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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
            panic!("Unexpected token: {:?}", self.current_token)
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
        self.eat(Token::LBrace);
        let expr = self.expr();
        self.eat(Token::Semicolon);
        self.eat(Token::RBrace);
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

    pub fn parse(&mut self) -> AST {
        if self.current_token == Token::If {
            self.if_else_statement()
        } else {
            self.expr()
        }
    }
}
