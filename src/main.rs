mod lexer;
mod parser;

fn main() {
    let text = "(hoi (tien plus tien))";
    match eval(text) {
        Ok(expr) => println!("{:?}", expr),
        Err(err) => println!("Error: {}", err),
    }
}

fn eval(text: &str) -> Result<parser::Expr, String> {
    let tokens = lexer::lex_str(text)?;
    let expr = parser::parse_tokens(&tokens)?;
    return Ok(expr);
}
