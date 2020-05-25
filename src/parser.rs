use super::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Sexp(Vec<Expr>),
    Value(f64),
}

struct TokenStream<'a> {
    pos: usize,
    tokens: &'a [Token],
}

impl<'a> TokenStream<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        TokenStream {
            pos: 0,
            tokens,
        }
    }

    fn pos(&self ) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
} 

fn parse_expr(stream: &mut TokenStream) -> Result<Expr, String> {
    skip_whitespace(stream);
    if let Ok(sexp) = parse_sexp(stream) {
        return Ok(sexp);
    }
    if let Ok(identifier) = parse_identifier(stream) {
        return Ok(identifier);
    }
    if let Ok(number) = parse_number(stream) {
        return Ok(number);
    }

    return Err("No rules matched".into())
}

fn skip_whitespace(stream: &mut TokenStream) {
    while let Some(Token::Whitespace(_)) = stream.pos() {
        stream.advance();
    }
}

fn parse_identifier(stream: &mut TokenStream) -> Result<Expr, String> {
    match stream.pos() {
        Some(Token::Identifier(id)) => {
            stream.advance();
            Ok(Expr::Identifier(id.clone()))
        }
        _ => Err("expected identifier".into())
    }
}

fn parse_number(stream: &mut TokenStream) -> Result<Expr, String> {
    match stream.pos() {
        Some(Token::Number(text)) => {
            stream.advance();
            match text.parse::<f64>() {
                Ok(num) => Ok(Expr::Value(num)),
                Err(err) => Err(format!("{}", err)),
            }
        }
        _ => Err("Not a number".into())
    }
}

fn parse_sexp(stream: &mut TokenStream) -> Result<Expr, String> {
    match stream.pos().unwrap() {
        Token::Symbol(sym) if *sym == '(' => (),
        _ => return Err("expected '('".into())

    }
    stream.advance();
    let mut buf = Vec::new();
    loop {
        match stream.pos() {
            None => return Err("unexpected EOS".into()),
            Some(Token::Symbol(sym)) if *sym == ')' => {stream.advance(); break},
            Some(_) => {
                let expr = parse_expr(stream)?;
                buf.push(expr);
            }
        }
    }
    return Ok(Expr::Sexp(buf));
}

pub fn parse_tokens(tokens: &[Token]) -> Result<Expr, String> {
    let mut stream = TokenStream::new(tokens);
    return parse_sexp(&mut stream);
}