#[derive(Clone, Debug)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    Application { abs: Box<Expr>, args: Vec<Expr> },
    Abstraction { args: Vec<String>, body: Box<Expr> },
}
