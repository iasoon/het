#[derive(Clone, Debug)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    Application { abs: Box<Expr>, args: Vec<Expr> },
    Abstraction { args: Vec<Declaration>, body: Box<Expr> },
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub name: String,
    pub type_name: Option<String>,
}