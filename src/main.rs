mod lexer;
mod ast;
mod parser;
mod evaluator;
mod types;

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
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}

fn eval(text: &str) -> Result<(), String> {
    let tokens = lexer::lex_str(text)?;
    let exprs = parser::parse_tokens(&tokens)?;
    for e in exprs {
        println!("{:?}", e);
        //println!("{}", evaluator::eval(&e)?)
        println!("{:?}", types::type_expr_default(&e)?);
    }
    return Ok(());
}
