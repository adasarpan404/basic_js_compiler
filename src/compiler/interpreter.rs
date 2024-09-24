use super::{
    parser::{Parser, AST},
    token::Token,
};

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new(parser: Parser) -> Self {
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

    pub fn interpret(&mut self) -> i64 {
        let tree = self.parser.parse();
        self.visit(tree)
    }
}
