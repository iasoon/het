mod lexer;
mod parser;
mod evaluator;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    match eval(text) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("Error: {}", err),
    }
}

fn eval(text: &str) -> Result<parser::Expr, String> {
    let tokens = lexer::lex_str(text)?;
    let expr = parser::parse_tokens(&tokens)?;
    let res = evaluator::eval(&expr);
    return Ok(res);
}
