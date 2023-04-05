use crate::{
    parser::{Expr, Stmt},
    lexer::Token
};

#[derive(Debug)]
pub enum Code {
    Axe,
    Chicken,
    Add,
    Fox,
    Rooster,
    Pick,
    Peck,
    Fr,
    Bbq,
    Push(i64),
    PushFloat(f64),
    PushVariable(String),

    CallFunc(String), // function name
    Div,
}

pub struct Compiler {
    code: Vec<Code>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
        }
    }

    pub fn compile(mut self, expr: &Stmt) -> Vec<Code> {
        self.compile_stmt(expr);
        self.code
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::StmtSeq(seq) => {
                for stmt in seq {
                    self.compile_stmt(&stmt);
                }
            }
            Stmt::Axe => self.code.push(Code::Axe),
            Stmt::Chicken => self.code.push(Code::Chicken),
            Stmt::Add => self.code.push(Code::Add),
            Stmt::Fox => self.code.push(Code::Fox),
            Stmt::Rooster => self.code.push(Code::Rooster),
            Stmt::Pick => self.code.push(Code::Pick),
            Stmt::Peck => self.code.push(Code::Peck),
            Stmt::Fr => self.code.push(Code::Fr),
            Stmt::Bbq => self.code.push(Code::Bbq),
            Stmt::Push(expr) => self.compile_expr(&expr),
            _ => todo!()
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Int(num) => self.code.push(Code::Push(*num)),
            Expr::Float(num) => self.code.push(Code::PushFloat(*num)),
            Expr::UnOp { op, operand } => {
                match op {
                    Token::Sub => {
                        self.compile_expr(&Expr::BinOp {
                            op: Token::Sub,
                            left: Box::new(Expr::Int(0)),
                            right: operand.clone()
                        });
                    }
                    Token::Add => {
                        self.compile_expr(operand);
                    }
                    _ => unreachable!()
                }
            }
            Expr::BinOp { op, left, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    Token::Plus => self.code.push(Code::Add),
                    Token::Sub => self.code.push(Code::Fox),
                    Token::Mul => self.code.push(Code::Rooster),
                    Token::Div => self.code.push(Code::Div),
                    _ => panic!("Unexpected operator {:?}", op),
                }
            }
            Expr::FunctionCall { name, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }
                self.code.push(Code::CallFunc(name.clone()));
            }
            Expr::Variable(name) => {
                self.code.push(Code::PushVariable(name.clone()));
            }
        }
    }
}
