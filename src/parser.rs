use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    StmtSeq(Vec<Stmt>),
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
    Push(Expr),
    #[allow(dead_code)]
    Ass(String, Expr)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    BinOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnOp {
        op: Token,
        operand: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Variable(String),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser {
        Parser {
            lexer,
        }
    }

    pub fn parse(mut self) -> Stmt {
        self.parse_stmt_seq()
    }

    fn parse_stmt_seq(&mut self) -> Stmt {
        let mut stmts = vec![];
        loop {
            match self.parse_stmt() {
                Some(stmt) => stmts.push(stmt),
                None => break,
            }
            self.lexer.match_token(Token::Semi);
        }
        Stmt::StmtSeq(stmts)
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.lexer.lookahead {
            Some(Token::Axe) => {
                self.lexer.step_token();
                Some(Stmt::Axe)
            }
            Some(Token::Chicken) => {
                self.lexer.step_token();
                Some(Stmt::Chicken)
            }
            Some(Token::Add) => {
                self.lexer.step_token();
                Some(Stmt::Add)
            }
            Some(Token::Fox) => {
                self.lexer.step_token();
                Some(Stmt::Fox)
            }
            Some(Token::Rooster) => {
                self.lexer.step_token();
                Some(Stmt::Rooster)
            }
            Some(Token::Cmp) => {
                self.lexer.step_token();
                Some(Stmt::Cmp)
            }
            Some(Token::Pick) => {
                self.lexer.step_token();
                Some(Stmt::Pick)
            }
            Some(Token::Peck) => {
                self.lexer.step_token();
                Some(Stmt::Peck)
            }
            Some(Token::Fr) => {
                self.lexer.step_token();
                Some(Stmt::Fr)
            }
            Some(Token::Bbq) => {
                self.lexer.step_token();
                Some(Stmt::Bbq)
            }
            Some(Token::Push) => {
                self.lexer.step_token();
                Some(Stmt::Push(self.parse_expr()))
            }
            None => None,
            _ => {
                println!("Unexpected token on line {} column {}", self.lexer.line, self.lexer.col);
                println!("char: {:?}", self.lexer.cur_char);
                self.lexer.point_error()
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let left = self.parse_term();
        self.parse_expr_tail(left)
    }

    fn parse_expr_tail(&mut self, left: Expr) -> Expr {
        match self.lexer.lookahead {
            Some(Token::Plus) => {
                self.lexer.match_token(Token::Plus);
                let right = self.parse_term();
                self.parse_expr_tail(Expr::BinOp {
                    op: Token::Plus,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Some(Token::Sub) => {
                self.lexer.match_token(Token::Sub);
                let right = self.parse_term();
                self.parse_expr_tail(Expr::BinOp {
                    op: Token::Sub,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => left,
        }
    }

    fn parse_term(&mut self) -> Expr {
        let left = self.parse_factor();
        self.parse_term_tail(left)
    }

    fn parse_term_tail(&mut self, left: Expr) -> Expr {
        match self.lexer.lookahead {
            Some(Token::Mul) => {
                self.lexer.match_token(Token::Mul);
                let right = self.parse_factor();
                self.parse_term_tail(Expr::BinOp {
                    op: Token::Mul,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Some(Token::Div) => {
                self.lexer.match_token(Token::Div);
                let right = self.parse_factor();
                self.parse_term_tail(Expr::BinOp {
                    op: Token::Div,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => left,
        }
    }

    fn parse_factor(&mut self) -> Expr {
        match self.lexer.lookahead.clone() {
            Some(Token::Int(num)) => {
                self.lexer.step_token();
                Expr::Int(num)
            }
            Some(op @ Token::Sub) | Some(op @ Token::Plus) => {
                self.lexer.step_token();
                Expr::UnOp { op, operand: Box::new(self.parse_factor()) }
            }
            Some(Token::Float(num)) => {
                self.lexer.step_token();
                Expr::Float(num)
            }
            Some(Token::LParen) => {
                self.lexer.step_token();
                let expr = self.parse_expr();
                self.lexer.match_token(Token::RParen);
                expr
            }
            Some(Token::Identifier(name)) => {
                self.lexer.step_token();
                if let Some(Token::LParen) = self.lexer.lookahead {
                    self.lexer.step_token();
                    let args = self.parse_argument_list();
                    self.lexer.match_token(Token::RParen);
                    Expr::FunctionCall { name: name.clone(), args }
                } else {
                    Expr::Variable(name.clone())
                }
            }
            _ => {
                println!(
                    "\n\n\nExpected an expression on line {} column {}, got {}:",
                    self.lexer.line,
                    self.lexer.col,
                    match self.lexer.cur_char {
                        Some(ch) => format!("{:?}", ch),
                        None => "None".to_owned()
                    }
                );
                self.lexer.point_error();
            }
        }
    }

    fn parse_argument_list(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        if self.lexer.lookahead != Some(Token::RParen) {
            args.push(self.parse_expr());
            while let Some(Token::Comma) = self.lexer.lookahead {
                self.lexer.step_token();
                args.push(self.parse_expr());
            }
        }
        args
    }
}
