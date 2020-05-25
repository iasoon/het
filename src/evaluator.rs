use super::parser::Expr;

pub fn eval(expr: &Expr) -> Expr {
    match expr {
        Expr::Value(val) => Expr::Value(*val),
        Expr::Identifier(id) => Expr::Identifier(id.clone()),
        Expr::Sexp(exprs) => {
            let vals = exprs.iter().map(eval).collect::<Vec<_>>();
            return apply(vals);
        }
    }
}

fn apply(exprs: Vec<Expr>) -> Expr {
    match &exprs[0] {
        Expr::Identifier(id) => {
            if let Some(res) = apply_fn(id, &exprs[1..]) {
                return res;
            }
        }
        _ => ()
    }
    return Expr::Sexp(exprs)
}

fn apply_fn(name: &str, args: &[Expr]) -> Option<Expr> {
    let mut arg_values = Vec::with_capacity(args.len());
    for arg in args.iter() {
        if let Expr::Value(value) = arg {
            arg_values.push(value);
        } else {
            return None;
        }
    }
    match name {
        "add" => {
            let res = arg_values.into_iter().sum();
            Some(Expr::Value(res))
        }
        _ => None,
    }
}