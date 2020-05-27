use super::ast::Expr;

use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    PrimitiveFn(&'static dyn Fn(&[Value]) -> Result<Value, String>),
    Number(f64),
    Lambda { args: Vec<String>, body: Closure },
}

#[derive(Clone)]
pub struct Closure {
    env: Env,
    expr: Expr,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::PrimitiveFn(_) => write!(f, "<primitive fn>"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Lambda { args, body } => {
                write!(f, "\\{:#?} -> {:?}", args, body.expr)
            }
        }
    }
}

#[derive(Clone)]
pub struct Env {
    bindings: HashMap<String, Value>,
}

impl Env {
    fn resolve<'a>(&'a self, name: &str) -> Option<&'a Value> {
        self.bindings.get(name)
    }

    fn default() -> Env {
        let mut bindings: HashMap<String, Value> = HashMap::new();
        bindings.insert("add".into(), Value::PrimitiveFn(&fun_add));
        return Env { bindings };
    }
}

pub fn eval(expr: &Expr) -> Result<Value, String> {
    let env = Env::default();
    return eval_(&env, expr);
}

fn eval_(env: &Env, expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Number(num) => Ok(Value::Number(num.clone())),
        Expr::Identifier(id) => {
            match env.resolve(id) {
                Some(val) => Ok(val.clone()),
                None => Err(format!("unknown identifier {}", id))
            }
        },
        Expr::Application { abs, args } => {
            let abs_eval = eval_(env, abs)?;
            let mut args_eval = Vec::with_capacity(args.len());
            for arg in args {
                let arg_eval = eval_(env, arg)?;
                args_eval.push(arg_eval);
            }
            apply(&abs_eval, &args_eval)
        }
        Expr::Abstraction { args, body } => {
            Ok(Value::Lambda {
                args: args.iter().map(|d| d.name.clone()).collect(),
                body: Closure  { env: env.clone(), expr: (**body).clone()}
            })
        },
    }
}

fn apply(abs: &Value, args: &[Value]) -> Result<Value, String> {
    match abs {
        Value::PrimitiveFn(fun) => {
            fun(args)
        }
        Value::Lambda { args: arg_names, body } => {
            let mut env = body.env.clone();
            for (name, value) in arg_names.iter().zip(args.iter()) {
                env.bindings.insert(name.clone(), value.clone());
            }
            return eval_(&env, &body.expr );
        }
        _ => Err(format!("Type mismatch in apply"))
    }
}

fn fun_add(values: &[Value]) -> Result<Value, String> {
    let mut sum = 0.0;
    for value in values {
        if let Value::Number(num) = value {
            sum += num;
        } else {
            return Err("add: expected number".into());
        }
    }
    return Ok(Value::Number(sum))
}