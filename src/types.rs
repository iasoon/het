use super::ast;
use ast::Expr;
use std::mem::swap;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Number,
    Abs { arg_types: Vec<Type>, result: Box<Type> },
    Var(usize),
}

// TODO: please find a better solution for this
fn parse_type(name: &str) -> Result<Type, String> {
    match name {
        "Num" => Ok(Type::Number),
        _ => Err(format!("unknown type name \"{}\"", name)),
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    bindings: Vec<Binding>,
}

#[derive(Debug, Clone)]
struct Binding {
    name: String,
    ty: Type,
}

#[derive(Clone)]
enum Constraint {
    Eq { t1: Type, t2: Type }
}

impl Env {
    fn new() -> Self {
        Env {
            bindings: Vec::new(),
        }
    }

    fn resolve<'a>(&'a self, name: &str) -> Option<&'a Type> {
        for binding in self.bindings.iter().rev() {
            if binding.name == name {
                return Some(&binding.ty);
            }
        }
        return None;
    }

    fn push_binding(&mut self, name: String, ty: Type) {
        self.bindings.push(Binding { name, ty });
    }

    fn push_unbound(&mut self, name: String) -> Type {
        let ty = Type::Var(self.bindings.len());
        self.push_binding(name, ty.clone());
        return ty;
    }

    fn pop_binding(&mut self) -> Binding {
        self.bindings.pop().unwrap()
    }

    fn resolve_type(&self, t: Type) -> Type {
        match t {
            Type::Var(idx) => self.bindings[idx].ty.clone(),
            _ => t,
        }
    }

    fn unify_types(&mut self, t1: Type, t2: Type) {
        let mut t1 = self.resolve_type(t1);
        let mut t2 = self.resolve_type(t2);
        order_typevars(&mut t1, &mut t2);

        if let Type::Var(idx) = t1 {
            self.bindings[idx].ty = t2;
        } else {
            assert_eq!(t1, t2);
        }
    }

    fn default() -> Env {
        let mut env = Env::new();

        env.push_binding("add".into(), Type::Abs {
            arg_types: vec![Type::Number, Type::Number],
            result: Box::new(Type::Number),
        });

        return env;
    }
}

fn order_typevars<'a>(t1: &'a mut Type, t2: &'a mut Type) {
    if let Type::Var(idx2) = t2 {
        if let Type::Var(idx1) = t1 {
            if idx1 < idx2 {
                swap(t1, t2);
            }
        } else {
            swap(t1, t2);
        }
    }
}


pub fn type_expr_default(expr: &Expr) -> Result<Type, String> {
    let mut env = Env::default();
    return type_expr(&mut env, expr);
}


fn type_expr(env: &mut Env, expr: &Expr) -> Result<Type, String> {
    match expr {
        Expr::Identifier(id) => {
            match env.resolve(id) {
                Some(ty) => Ok(ty.clone()),
                None => Err(format!("Unknown identifier {}", id)),
            }
        }
        Expr::Number(_) => Ok(Type::Number),
        Expr::Abstraction { args, body } => {
            for arg in args {
                let arg_type = match &arg.type_name {
                    Some(type_name) => parse_type(type_name)?,
                    None => { env.push_unbound(arg.name.clone()) },
                };
            }
            let body_type = type_expr(env, body)?;

            let mut arg_types = Vec::with_capacity(args.len());
            for _ in args {
                let Binding { name:_, ty } = env.pop_binding();
                arg_types.push(ty);
            };
            arg_types.reverse();

            Ok(Type::Abs {
                arg_types,
                result: Box::new(body_type),
            })
        }
        Expr::Application { abs, args } => {
            match type_expr(env, abs) {
                Ok(Type::Abs { arg_types, result}) => {
                    for i in 1..=args.len() {
                        let arg_type = arg_types[arg_types.len() - i].clone();
                        let expr_type = type_expr(env, &args[args.len() - i])?;
                        env.unify_types(arg_type, expr_type);
                    }
                    // at this point, arguments match
                    if arg_types.len() > args.len() {
                        return Ok(Type::Abs {
                            arg_types: arg_types[..arg_types.len() - args.len()].to_vec(),
                            result,
                        })
                    } else {
                        return Ok(*result);
                    }
                }
                Ok(ty) => {
                    Err(format!("not a function: {:?}, but {:?}", expr, ty))
                }
                Err(e) => Err(e),
            }
        }
    }
}