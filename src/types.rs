use super::ast;
use ast::Expr;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Number,
    Abs { arg_types: Vec<Type>, result: Box<Type> }
}

// TODO: please find a better solution for this
fn parse_type(name: &str) -> Result<Type, String> {
    match name {
        "Num" => Ok(Type::Number),
        _ => Err(format!("unknown type name \"{}\"", name)),
    }
}



#[derive(Clone)]
pub struct Env {
    bindings: HashMap<String, Type>,
}

impl Env {
    fn resolve<'a>(&'a self, name: &str) -> Option<&'a Type> {
        self.bindings.get(name)
    }

    fn default() -> Env {
        let mut bindings: HashMap<String, Type> = HashMap::new();
        bindings.insert("add".into(), Type::Abs {
            arg_types: vec![Type::Number, Type::Number],
            result: Box::new(Type::Number),
        });
        return Env { bindings };
    }
}

pub fn type_expr_default(expr: &Expr) -> Result<Type, String> {
    let env = Env::default();
    return type_expr(&env, expr);
}


fn type_expr(env: &Env, expr: &Expr) -> Result<Type, String> {
    match expr {
        Expr::Identifier(id) => {
            match env.resolve(id) {
                Some(ty) => Ok(ty.clone()),
                None => Err(format!("Unknown identifier {}", id)),
            }
        }
        Expr::Number(_) => Ok(Type::Number),
        Expr::Abstraction { args, body } => {
            let mut body_env = env.clone();
            let mut arg_types = Vec::with_capacity(args.len());
            for arg in args {
                let arg_type = match &arg.type_name {
                    Some(type_name) => parse_type(type_name)?,
                    None => return Err(format!("No type given for arg {}", arg.name)),
                };
                arg_types.push(arg_type.clone());
                body_env.bindings.insert(arg.name.clone(), arg_type);
            }
            let t = Type::Abs { 
                arg_types,
                result: Box::new(type_expr(&body_env, body)?),
            };
            Ok(t)
        }
        Expr::Application { abs, args } => {
            match type_expr(env, abs) {
                Ok(Type::Abs { arg_types, result}) => {
                    for i in 1..=args.len() {
                        let expected_type = &arg_types[arg_types.len() - i];
                        let given_type = type_expr(env, &args[args.len() - i])?;
                        if expected_type != &given_type {
                            return Err(format!("expected {:?} but got {:?}", expected_type, given_type));
                        }
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