use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let t = self.peek().clone();
        self.pos += 1;
        t
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while *self.peek() != Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Token::Const => self.parse_const(),
            _ => Stmt::ExprStmt(self.parse_expr()),
        }
    }

    fn parse_const(&mut self) -> Stmt {
        self.advance(); // const

        let name = match self.advance() {
            Token::Ident(s) => s,
            _ => panic!("Expected identifier after const"),
        };

        match self.advance() {
            Token::Equal => {}
            _ => panic!("Expected '=' after const name"),
        }

        let value = self.parse_expr();

        if self.peek() == &Token::Semicolon {
            self.advance();
        }

        Stmt::Const { name, value }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.advance() {
            Token::Number(n) => Expr::Number(n),
            Token::Ident(name) => {
                if self.peek() == &Token::LParen {
                    self.advance(); // '('

                    let mut args = vec![];
                    if self.peek() != &Token::RParen {
                        args.push(self.parse_expr());
                        while self.peek() == &Token::Comma {
                            self.advance();
                            args.push(self.parse_expr());
                        }
                    }

                    if self.peek() == &Token::RParen {
                        self.advance();
                    }

                    Expr::Call { name, args }
                } else {
                    Expr::Ident(name)
                }
            }
            _ => Expr::Ident("?".to_string()),
        }
    }
}
