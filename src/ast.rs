#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Const { name: String, value: Expr },
    ExprStmt(Expr),
}
