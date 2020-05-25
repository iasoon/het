use super::parser::Expr;

use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    PrimitiveFn(&'static dyn Fn(&[Value]) -> Result<Value, String>),
    Number(f64),
    Lambda { arg: String, body: Closure },
}

#[derive(Clone)]
pub struct Closure {
    env: Env,
    expr: Expr,
}

impl Closure {
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::PrimitiveFn(_) => write!(f, "<primitive fn>"),
            Value::Number(num) => write!(f, "{}", num),
            Value::Lambda { arg, body } => {
                write!(f, "\\{} -> {:?}", arg, body.expr)
            }
        }
    }
}

#[derive(Clone)]
struct Env {
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
        Expr::Value(num) => Ok(Value::Number(num.clone())),
        Expr::Identifier(id) => {
            match env.resolve(id) {
                Some(val) => Ok(val.clone()),
                None => Err(format!("unknown identifier {}", id))
            }
        },
        Expr::Sexp(exprs) => {
            let mut evaluated = Vec::with_capacity(exprs.len());
            for expr in exprs.iter() {
                let value = eval_(env, expr)?;
                evaluated.push(value);
            }
            apply(&evaluated)
        }
        Expr::Lambda { arg, body } => {
            Ok(Value::Lambda {
                arg: arg.clone(),
                body: Closure  { env: env.clone(), expr: (**body).clone()}
            })
        },
    }
}

fn apply(values: &[Value]) -> Result<Value, String> {
    match &values[0] {
        Value::PrimitiveFn(fun) => {
            fun(&values[1..])
        }
        Value::Lambda { arg, body } => {
            let mut env = body.env.clone();
            env.bindings.insert(arg.clone(), values[1].clone());
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