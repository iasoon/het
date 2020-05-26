use super::parser::Expr;

pub enum Type {
    Number,
    Abs { arg: Box<Type>, result: Box<Type> }
}