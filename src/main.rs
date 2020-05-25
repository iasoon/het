mod lexer;
mod parser;
mod evaluator;

use std::env;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    match eval(&contents) {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}

fn eval(text: &str) -> Result<evaluator::Value, String> {
    let tokens = lexer::lex_str(text)?;
    let expr = parser::parse_tokens(&tokens)?;
    let res = evaluator::eval(&expr);
    return res;
}
